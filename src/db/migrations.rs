use eyre::Result;

const CURRENT_VERSION: i32 = 1;

pub trait Migration {
    fn migrate(&self) -> Result<()>;
}

impl Migration for rusqlite::Connection {
    fn migrate(&self) -> Result<()> {
        let _user_version: i32 =
            self.query_row("SELECT user_version FROM pragma_user_version;", [], |row| {
                row.get(0)
            })?;

        // Upgrade version
        Ok(self.pragma_update(None, "user_version", CURRENT_VERSION)?)
    }
}
