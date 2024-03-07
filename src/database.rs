use rusqlite::{Connection, Result};

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new(path: &str) -> Result<Self> {
        let conn = Connection::open(path)?;
        Ok(Database { conn })
    }

    pub fn create(&self) -> Result<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS servers (
            id INTEGER,
            channel_id INTEGER,
            role_id INTEGER
        )",
            [],
        )?;
        Ok(())
    }
}
