use rusqlite;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::CONFIG;

pub struct MollySocketDb {
    db: rusqlite::Connection,
}

#[derive(Debug)]
pub struct Connection {
    pub uuid: String,
    pub device_id: u32,
    pub password: String,
    pub endpoint: String,
    pub forbidden: bool,
    pub last_registration: Option<SystemTime>,
}

impl MollySocketDb {
    pub fn new() -> Result<MollySocketDb, rusqlite::Error> {
        let db = rusqlite::Connection::open(CONFIG.db.clone())?;
        db.execute_batch(
            "
CREATE TABLE IF NOT EXISTS connections(
    uuid TEXT UNIQUE ON CONFLICT REPLACE,
    device_id INTEGER,
    password TEXT,
    endpoint TEXT,
    forbidden BOOLEAN NOT NULL CHECK (forbidden IN (0, 1)),
    last_registration INTEGER
)
            ",
        )?;
        Ok(MollySocketDb { db })
    }

    pub fn add(&self, co: Connection) -> Result<(), rusqlite::Error> {
        self.db.execute(
            "INSERT INTO connections(uuid, device_id, password, endpoint, forbidden, last_registration)
            VALUES (?, ?, ?, ?, ?, ?);",
            [co.uuid, co.device_id.to_string(), co.password, co.endpoint, String::from(if co.forbidden { "1" } else { "0" }), instant_to_int(co.last_registration).to_string()]
        )?;
        Ok(())
    }

    pub fn list(&self) -> Result<Vec<Connection>, rusqlite::Error> {
        self.db
            .prepare("SELECT * FROM connections;")?
            .query_map([], |raw| {
                Ok(Connection {
                    uuid: raw.get(0)?,
                    device_id: raw.get(1)?,
                    password: raw.get(2)?,
                    endpoint: raw.get(3)?,
                    forbidden: raw.get(4)?,
                    last_registration: int_to_instant(raw.get(5)?),
                })
            })?
            .collect()
    }

    pub fn rm(&self, uuid: &str) -> Result<(), rusqlite::Error> {
        self.db
            .execute("DELETE FROM connections WHERE uuid=?1;", [&uuid])?;
        Ok(())
    }
}

fn instant_to_int(instant: Option<SystemTime>) -> u64 {
    let instant = match instant {
        Some(instant) => instant,
        None => return 0,
    };
    match instant.duration_since(UNIX_EPOCH) {
        Ok(duration) => duration.as_secs(),
        Err(_) => 0,
    }
}
fn int_to_instant(instant_as_int: u64) -> Option<SystemTime> {
    if instant_as_int <= 0 {
        return None;
    }
    let duration = Duration::from_secs(instant_as_int);
    UNIX_EPOCH.checked_add(duration)
}
