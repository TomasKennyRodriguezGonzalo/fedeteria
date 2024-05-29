use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize,Debug)]
pub struct Notificacion {
    pub titulo : String,
    pub detalle : String,
    pub url : String,
}