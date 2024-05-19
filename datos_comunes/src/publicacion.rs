use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Publicacion {
    pub dni_usuario: u64,
    pub titulo: String,
    pub descripcion: String,
    // Las imagenes son relativas (osea que hay que agregar /db/imgs o publication_images/ dependiendo del caso)
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

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct QueryPublicacion {
    pub id: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum ErrorPublicacion {
    ErrorIndeterminado,
    PublicacionInexistente,
}

pub type ResponsePublicacion = Result<Publicacion, ErrorPublicacion>;