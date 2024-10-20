use crate::{config, db::Connection, utils::ping};
use eyre::Result;
use html::get_index;
use rocket::{
    get, post,
    response::{content::RawHtml, Responder},
    routes,
    serde::{json::Json, Deserialize, Serialize},
};
use std::{collections::HashMap, env, str::FromStr};
use url::Url;

use super::{metrics::MountMetrics, DB, METRICS, NEW_CO_TX};

mod html;

#[derive(Serialize)]
struct ApiResponse {
    mollysocket: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
struct ConnectionData {
    pub uuid: String,
    pub device_id: u32,
    pub password: String,
    pub endpoint: String,
    pub ping: Option<bool>,
}

#[derive(Debug)]
enum RegistrationStatus {
    New,
    CredsUpdated,
    EndpointUpdated,
    Running,
    Forbidden,
    InvalidUuid,
    InvalidEndpoint,
    InternalError,
}

// This is used to send the reponse to Molly
impl From<RegistrationStatus> for String {
    fn from(r: RegistrationStatus) -> Self {
        match r {
            RegistrationStatus::New
            | RegistrationStatus::CredsUpdated
            | RegistrationStatus::EndpointUpdated
            | RegistrationStatus::Running => "ok",
            RegistrationStatus::Forbidden => "forbidden",
            RegistrationStatus::InvalidUuid => "invalid_uuid",
            RegistrationStatus::InvalidEndpoint => "invalid_endpoint",
            RegistrationStatus::InternalError => "internal_error",
        }
        .into()
    }
}

struct UA<'r>(&'r str);

#[rocket::async_trait]
impl<'r> rocket::request::FromRequest<'r> for UA<'r> {
    type Error = ();

    async fn from_request(
        request: &'r rocket::request::Request<'_>,
    ) -> rocket::request::Outcome<UA<'r>, ()> {
        let ua = request.headers().get_one("user-agent").unwrap_or("");
        rocket::request::Outcome::Success(UA(ua))
    }
}

enum Resp {
    Json(Json<ApiResponse>),
    Html(RawHtml<String>),
}

impl<'r> Responder<'r, 'r> for Resp {
    fn respond_to(self, request: &'r rocket::Request<'_>) -> rocket::response::Result<'r> {
        match self {
            Resp::Json(r) => r.respond_to(request),
            Resp::Html(r) => r.respond_to(request),
        }
    }
}

#[get("/")]
fn index(ua: UA) -> Resp {
    if ua.0.contains("Signal-Android") {
        Resp::Json(gen_api_rep(HashMap::new()))
    } else {
        Resp::Html(RawHtml(get_index()))
    }
}

#[get("/discover")]
fn discover() -> Json<ApiResponse> {
    gen_api_rep(HashMap::new())
}

#[post("/", format = "application/json", data = "<co_data>")]
async fn register(co_data: Json<ConnectionData>) -> Json<ApiResponse> {
    let mut status = registration_status(&co_data).await;
    match status {
        RegistrationStatus::New | RegistrationStatus::CredsUpdated => {
            if new_connection(&co_data).is_ok() {
                log::debug!("Connection succeeded");
                ping_endpoint(&co_data).await;
            } else {
                log::debug!("Could not start new connection");
                status = RegistrationStatus::InternalError;
            }
        }
        RegistrationStatus::EndpointUpdated => {
            if new_connection(&co_data).is_ok() {
                log::debug!("Connection succeeded");
                if co_data.ping.unwrap_or(false) {
                    ping_endpoint(&co_data).await;
                }
            } else {
                log::debug!("Could not start new connection");
                status = RegistrationStatus::InternalError;
            }
        }
        RegistrationStatus::Forbidden => {
            log::debug!("Connection is currently forbidden");
            if let Ok(co) = DB.get(&co_data.uuid) {
                if co.device_id != co_data.device_id || co.password != co_data.password {
                    if new_connection(&co_data).is_ok() {
                        log::debug!("Connection succeeded");
                        status = RegistrationStatus::CredsUpdated;
                        METRICS.forbiddens.dec();
                    } else {
                        log::debug!("Could not start new connection");
                        status = RegistrationStatus::InternalError;
                    }
                }
            } else {
                log::debug!("Could not get info in DB about the connection");
                status = RegistrationStatus::InternalError;
            }
        }
        RegistrationStatus::Running => {
            // If the connection is "Running" then the device creds still exists,
            // if the user register on another server or delete the linked device,
            // then the connection ends with a 403 Forbidden
            // If the connection is for an invalid uuid or an error occured : we
            // have nothing to do, except if the request ask for a ping
            DB.update_last_registration(&co_data.uuid).unwrap();
            if co_data.ping.unwrap_or(false) {
                ping_endpoint(&co_data).await;
            }
        }
        RegistrationStatus::InvalidEndpoint | RegistrationStatus::InvalidUuid => (),
        _ => {
            log::debug!("Status unknown: {status:?}");
            status = RegistrationStatus::InternalError;
        }
    }
    log::debug!("Status: {status:?}");
    gen_api_rep(HashMap::from([(
        String::from("status"),
        String::from(status),
    )]))
}

fn new_connection(co_data: &Json<ConnectionData>) -> Result<()> {
    let co = Connection::new(
        co_data.uuid.clone(),
        co_data.device_id,
        co_data.password.clone(),
        co_data.endpoint.clone(),
    );
    DB.add(&co).unwrap();
    if let Some(tx) = &*NEW_CO_TX.lock().unwrap() {
        let _ = tx.unbounded_send(co);
    }
    Ok(())
}

async fn ping_endpoint(co_data: &ConnectionData) {
    if let Err(e) = ping(Url::from_str(&co_data.endpoint).unwrap()).await {
        log::warn!(
            "Cound not ping the connection (uuid={}): {e:?}",
            &co_data.uuid
        );
    }
}

async fn registration_status(co_data: &ConnectionData) -> RegistrationStatus {
    let endpoint_valid = config::is_endpoint_valid(&co_data.endpoint).await;
    let uuid_valid = config::is_uuid_valid(&co_data.uuid);

    if !uuid_valid {
        return RegistrationStatus::InvalidUuid;
    }

    if !endpoint_valid {
        return RegistrationStatus::InvalidEndpoint;
    }

    let co = match DB.get(&co_data.uuid) {
        Ok(co) => co,
        Err(_) => {
            return RegistrationStatus::New;
        }
    };

    if co.device_id == co_data.device_id && co.password == co_data.password {
        // Credentials are not updated
        if co.forbidden {
            RegistrationStatus::Forbidden
        } else if co.endpoint != co_data.endpoint {
            RegistrationStatus::EndpointUpdated
        } else {
            RegistrationStatus::Running
        }
    } else {
        RegistrationStatus::CredsUpdated
    }
}

fn gen_api_rep(mut map: HashMap<String, String>) -> Json<ApiResponse> {
    map.insert(
        String::from("version"),
        env!("CARGO_PKG_VERSION").to_string(),
    );
    Json(ApiResponse { mollysocket: map })
}

pub async fn launch() {
    if !config::should_start_webserver() {
        log::warn!("The web server is disabled, making mollysocket run in an air gapped mode. With this clients are less easy to set up and push might break.");
        return;
    }

    let rocket_cfg = rocket::Config::figment()
        .merge(("address", config::get_host()))
        .merge(("port", config::get_port()));

    let _ = rocket::build()
        .configure(rocket_cfg)
        .mount("/", routes![index, discover, register])
        .mount_metrics("/metrics", &METRICS)
        .launch()
        .await;
}
