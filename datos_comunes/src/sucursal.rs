use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize ,PartialEq, Clone)]
pub struct Sucursal {
    pub nombre: String,
    pub esta_activa: bool,
    pub id: usize,
}