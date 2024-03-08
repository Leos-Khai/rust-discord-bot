use rusqlite::{Connection, Result};

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new(path: &str) -> Result<Self> {
        let conn = Connection::open(path)?;
        let db = Database { conn };

        if !db.tables_exist()? {
            db.init_db()?;
        }

        Ok(db)
    }

    fn tables_exist(&self) -> Result<bool> {
        let mut stmt = self.conn.prepare(
            "SELECT COUNT(*) FROM (
                SELECT 1 FROM sqlite_master WHERE type='table' AND name='servers'
                INTERSECT
                SELECT 1 FROM sqlite_master WHERE type='table' AND name='channel_id'
                INTERSECT
                SELECT 1 FROM sqlite_master WHERE type='table' AND name='role_id'
                INTERSECT
                SELECT 1 FROM sqlite_master WHERE type='table' AND name='voice_channel_id'
            )",
        )?;
        let count: usize = stmt.query_row([], |row| row.get(0))?;
        Ok(count == 4)
    }

    fn init_db(&self) -> Result<()> {
        self.conn.execute_batch(
            "PRAGMA foreign_keys=OFF;
                BEGIN TRANSACTION;
    
                CREATE TABLE IF NOT EXISTS servers (
                    server_id INTEGER PRIMARY KEY,
                    server_name TEXT NOT NULL
                );
    
                CREATE TABLE IF NOT EXISTS channel_id (
                    channel_id INTEGER PRIMARY KEY,
                    server_id INTEGER NOT NULL,
                    channel_name TEXT NOT NULL,
                    FOREIGN KEY (server_id) REFERENCES servers(server_id)
                );
    
                CREATE TABLE IF NOT EXISTS role_id (
                    role_id INTEGER PRIMARY KEY,
                    channel_id INTEGER NOT NULL,
                    role_name TEXT NOT NULL,
                    FOREIGN KEY (channel_id) REFERENCES channel_id(channel_id)
                );
    
                CREATE TABLE IF NOT EXISTS voice_channel_id (
                    voice_channel_id INTEGER PRIMARY KEY,
                    channel_id INTEGER NOT NULL,
                    voice_channel_name TEXT NOT NULL,
                    FOREIGN KEY (channel_id) REFERENCES channel_id(channel_id)
                );
    
                COMMIT;
                PRAGMA foreign_keys=ON;",
        )?;
        Ok(())
    }

    pub fn print_existing_tables(&self) -> Result<()> {
        let mut stmt = self
            .conn
            .prepare("SELECT name FROM sqlite_master WHERE type='table'")?;
        let table_names = stmt
            .query_map([], |row| row.get::<_, String>(0))?
            .filter_map(|result| result.ok())
            .collect::<Vec<_>>();

        if table_names.is_empty() {
            println!("No tables found in the database.");
        } else {
            println!("Existing tables in the database:");
            for table_name in table_names {
                println!("- {}", table_name);
            }
        }

        Ok(())
    }
}
