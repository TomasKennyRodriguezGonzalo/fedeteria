use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

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
pub enum RolDeUsuario {
    Normal,
    Dueño,
    Empleado{sucursal: usize},
}

#[derive(Debug, Deserialize, Serialize)]
pub enum EstadoCuenta {
    Activa{intentos: u8},
    Bloqueada,
}