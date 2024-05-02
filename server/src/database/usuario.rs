use std::hash::{DefaultHasher, Hash, Hasher};

use chrono::{DateTime, Local};

pub struct Usuario {
    pub dni: u64,
    pub email: String,
    pub contraseña: u64,
    pub nacimiento: DateTime<Local>,
}

impl Usuario {
    pub fn new(dni: u64, email: String, contraseña: String, nacimiento: DateTime<Local>) -> Self {
        let contraseña = hash_str(&contraseña);
        Usuario {
            dni,
            email,
            contraseña,
            nacimiento,
        }
    }
}


fn hash_str(s: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    s.hash(&mut hasher);
    hasher.finish()
}

