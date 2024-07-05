use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize ,PartialEq, Clone)]
pub struct PeticionCambioContrasenia {
    pub email: String,
    pub codigo_seguridad: u64,
    pub usada: bool,
}