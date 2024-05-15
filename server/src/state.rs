use crate::database::Database;


pub struct ServerState {
    pub db: Database,
}

impl ServerState {
    pub fn new(db: Database) -> Self {
        ServerState {
            db
        }
    }
}