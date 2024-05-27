use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Notificacion {
    titulo : String,
    detalle : String,
    url : String,
}