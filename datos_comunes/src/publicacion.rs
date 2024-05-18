use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Publicacion {
    pub dni_usuario: u64,
    pub titulo: String,
    pub descripcion: String,
    pub imagenes: Vec<String>,
    pub precio: Option<u64>,
    pub pausada: bool,
}

impl Publicacion {
    pub fn new(titulo: String, descripcion: String, imagenes: Vec<String>, dni_usuario: u64) -> Self {
        Publicacion { 
            titulo,
            descripcion, 
            imagenes,
            dni_usuario,
            precio: None,
            pausada: true,
        }
    }
}

