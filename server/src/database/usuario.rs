use std::hash::{Hash, Hasher, DefaultHasher};


use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

use super::sucursal::Sucursal;

#[derive(Debug, Deserialize, Serialize)]
pub struct Usuario {
    pub nombre_y_apellido: String,
    pub dni: u64,
    pub email: String,
    pub contraseña: u64,
    pub nacimiento: DateTime<Local>,
    pub rol: RolDeUsuario,
    pub estado: EstadoCuenta,
}

#[derive(Debug, Deserialize, Serialize)]
enum RolDeUsuario {
    Normal,
    Dueño,
    Empleado{sucursal: usize},
}

#[derive(Debug, Deserialize, Serialize)]
enum EstadoCuenta {
    Activa{intentos: u8},
    Bloqueada,
}

impl Usuario {
    pub fn new(nombre_y_apellido: String, dni: u64, email: String, contraseña: String, nacimiento: DateTime<Local>) -> Self {
        let contraseña = hash_str(&contraseña);
        Usuario {
            nombre_y_apellido,
            dni,
            email,
            contraseña,
            nacimiento,
            rol: RolDeUsuario::Normal,
            estado: EstadoCuenta::Activa { intentos: 0 }
        }
    }
}


fn hash_str(s: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    s.hash(&mut hasher);
    hasher.finish()
}

