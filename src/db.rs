use eyre::Result;
use rusqlite::{self, Row};
use std::{
    sync::{Arc, Mutex},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use crate::CONFIG;

mod migrations;

pub struct MollySocketDb {
    db: Arc<Mutex<rusqlite::Connection>>,
}

#[derive(Debug)]
pub struct Connection {
    pub uuid: String,
    pub device_id: u32,
    pub password: String,
    pub endpoint: String,
    pub forbidden: bool,
    pub last_registration: OptTime,
}

#[derive(Debug)]
pub struct OptTime(pub Option<SystemTime>);

impl From<&OptTime> for u64 {
    fn from(i: &OptTime) -> u64 {
        let instant = match i.0 {
            Some(instant) => instant,
            None => return 0,
        };
        match instant.duration_since(UNIX_EPOCH) {
            Ok(duration) => duration.as_secs(),
            Err(_) => 0,
        }
    }
}

impl From<u64> for OptTime {
    fn from(i: u64) -> OptTime {
        if i == 0 {
            return OptTime(None);
        }
        let duration = Duration::from_secs(i);
        OptTime(UNIX_EPOCH.checked_add(duration))
    }
}

impl From<SystemTime> for OptTime {
    fn from(t: SystemTime) -> Self {
        OptTime(Some(t))
    }
}

impl Connection {
    fn map(row: &Row) -> Result<Connection> {
        Ok(Connection {
            uuid: row.get(0)?,
            device_id: row.get(1)?,
            password: row.get(2)?,
            endpoint: row.get(3)?,
            forbidden: row.get(4)?,
            last_registration: OptTime::from(row.get::<usize, u64>(5)?),
        })
    }
}

impl MollySocketDb {
    pub fn new() -> Result<MollySocketDb> {
        let db = rusqlite::Connection::open(CONFIG.user_cfg.db.clone())?;
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
        Ok(MollySocketDb {
            db: Arc::new(Mutex::new(db)),
        })
    }

    pub fn add(&self, co: &Connection) -> Result<()> {
        self.db.lock().unwrap().execute(
            "INSERT INTO connections(uuid, device_id, password, endpoint, forbidden, last_registration)
            VALUES (?, ?, ?, ?, ?, ?);",
            [&co.uuid, &co.device_id.to_string(), &co.password, &co.endpoint, &String::from(if co.forbidden { "1" } else { "0" }), &u64::from(&co.last_registration).to_string()]
        )?;
        Ok(())
    }

    pub fn list(&self) -> Result<Vec<Connection>> {
        self.db
            .lock()
            .unwrap()
            .prepare("SELECT * FROM connections;")?
            .query_and_then([], Connection::map)?
            .collect::<Result<Vec<Connection>>>()
    }

    pub fn get(&self, uuid: &str) -> Result<Connection> {
        self.db
            .lock()
            .unwrap()
            .prepare("SELECT * FROM connections WHERE uuid=?1 LIMIT 1")?
            .query_and_then([uuid], Connection::map)?
            .next()
            .ok_or(rusqlite::Error::QueryReturnedNoRows)?
    }

    pub fn rm(&self, uuid: &str) -> Result<()> {
        self.db
            .lock()
            .unwrap()
            .execute("DELETE FROM connections WHERE uuid=?1;", [&uuid])?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_db() {
        let db = MollySocketDb::new().unwrap();
        let uuid = "0d2ff653-3d88-43de-bcdb-f6657d3484e4";
        db.add(&Connection {
            uuid: String::from(uuid),
            device_id: 1,
            password: String::from("pass"),
            endpoint: String::from("http://0.0.0.0/"),
            forbidden: false,
            last_registration: OptTime(None),
        })
        .unwrap();
        assert!(db
            .list()
            .unwrap()
            .iter()
            .map(|co| &co.uuid)
            .any(|row_uuid| row_uuid == uuid));
        db.rm(uuid).unwrap();
    }
}
