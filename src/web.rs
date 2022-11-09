use crate::CONFIG;
use lazy_static::lazy_static;
use rocket::{get, routes, serde::json::Json};
use std::collections::HashMap;

type Response = HashMap<String, HashMap<String, String>>;

lazy_static! {
    static ref DISCOVER_REP: Response = HashMap::from([(
        String::from("mollysocket"),
        HashMap::from([(String::from("version"), CONFIG.version.clone())]),
    )]);
}

#[get("/")]
fn discover() -> Json<&'static Response> {
    Json(&DISCOVER_REP)
}

pub async fn launch() {
    let _ = rocket::build().mount("/", routes![discover]).launch().await;
}
