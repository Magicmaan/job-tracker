use color_eyre::Result;

#[derive(Debug)]
pub struct Database {
    connection: rusqlite::Connection,
}
impl Default for Database {
    fn default() -> Self {
        Self::new("job_tracker.db").expect("Failed to create database")
    }
}

impl Database {
    pub fn new(db_path: &str) -> rusqlite::Result<Self> {
        let connection = rusqlite::Connection::open(db_path)?;
        Ok(Database { connection })
    }
    pub fn create(&self) -> Result<()> {
        self.connection.execute_batch(
            "
                CREATE TABLE IF NOT EXISTS job_applications (
                    id INTEGER PRIMARY KEY,
                    company_name TEXT NOT NULL,
                    position TEXT NOT NULL,
                    position_category TEXT NOT NULL,
                    work_type TEXT NOT NULL,
                    location TEXT NOT NULL,
                    location_type TEXT NOT NULL,
                    application_date TEXT NOT NULL,
                    status TEXT NOT NULL,
                    is_active BOOLEAN NOT NULL,
                    notes TEXT,
                    contact_info TEXT,
                    url TEXT,
                    files TEXT NOT NULL
                );
            ",
        )?;
        Ok(())
    }
    pub fn connection(&self) -> &rusqlite::Connection {
        &self.connection
    }
}
