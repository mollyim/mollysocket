use crate::{
    db::{Connection, OptTime},
    error::Error,
    CONFIG,
};
use rocket::{
    get, post, routes,
    serde::{json::Json, Deserialize, Serialize},
};
use std::{collections::HashMap, time::SystemTime};

use super::{DB, TX};

#[derive(Serialize)]
struct Response {
    mollysocket: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
struct ConnectionData {
    pub uuid: String,
    pub device_id: u32,
    pub password: String,
    pub endpoint: String,
}

enum RegistrationStatus {
    New,
    Running,
    Forbidden,
    InvalidUuid,
    InvalidEndpoint,
    InternalError,
}

impl From<RegistrationStatus> for String {
    fn from(r: RegistrationStatus) -> Self {
        String::from(match r {
            RegistrationStatus::New => "ok",
            RegistrationStatus::Running => "ok",
            RegistrationStatus::Forbidden => "forbidden",
            RegistrationStatus::InvalidUuid => "invalid_uuid",
            RegistrationStatus::InvalidEndpoint => "invalid_endpoint",
            RegistrationStatus::InternalError => "internal_error",
        })
    }
}

#[get("/")]
fn discover() -> Json<Response> {
    gen_rep(HashMap::new())
}

#[post("/signal", format = "application/json", data = "<co_data>")]
async fn register(co_data: Json<ConnectionData>) -> Json<Response> {
    let mut status = registration_status(&co_data.uuid, &co_data.endpoint).await;
    match status {
        RegistrationStatus::New => {
            if let Err(_) = new_connection(co_data) {
                status = RegistrationStatus::InternalError;
            }
        }
        RegistrationStatus::Forbidden => {
            if let Ok(co) = DB.get(&co_data.uuid) {
                if co.device_id != co_data.device_id || co.password != co_data.password {
                    if let Ok(_) = new_connection(co_data) {
                        status = RegistrationStatus::Running;
                    } else {
                        status = RegistrationStatus::InternalError;
                    }
                }
            } else {
                status = RegistrationStatus::InternalError;
            }
        }
        // If the connection is "Running" then the device creds still exists,
        // if the user register on another server or delete the linked device,
        // then the connection ends with a 403 Forbidden
        // If the connection is for an invalid uuid or an error occured : we ignore it
        _ => {}
    }
    gen_rep(HashMap::from([(
        String::from("status"),
        String::from(status),
    )]))
}

fn new_connection(co_data: Json<ConnectionData>) -> Result<(), Error> {
    let co = Connection {
        uuid: co_data.uuid.clone(),
        device_id: co_data.device_id,
        password: co_data.password.clone(),
        endpoint: co_data.endpoint.clone(),
        forbidden: false,
        last_registration: OptTime::from(SystemTime::now()),
    };
    DB.add(&co)?;
    if let Some(tx) = &*TX.lock().unwrap() {
        let _ = tx.unbounded_send(co);
    }
    Ok(())
}

async fn registration_status(uuid: &str, endpoint: &str) -> RegistrationStatus {
    let endpoint_valid = CONFIG.is_endpoint_valid(endpoint).await;
    if CONFIG.is_uuid_valid(uuid) {
        if let Ok(co) = DB.get(uuid) {
            if co.forbidden {
                return RegistrationStatus::Forbidden;
            } else {
                return if endpoint_valid {
                    RegistrationStatus::Running
                } else {
                    RegistrationStatus::InvalidEndpoint
                };
            }
        }
    } else {
        return RegistrationStatus::InvalidUuid;
    }
    if endpoint_valid {
        RegistrationStatus::New
    } else {
        RegistrationStatus::InvalidEndpoint
    }
}

fn gen_rep(mut map: HashMap<String, String>) -> Json<Response> {
    map.insert(String::from("version"), CONFIG.version.clone());
    Json(Response { mollysocket: map })
}

pub async fn launch() {
    let _ = rocket::build()
        .mount("/", routes![discover, register])
        .launch()
        .await;
}
