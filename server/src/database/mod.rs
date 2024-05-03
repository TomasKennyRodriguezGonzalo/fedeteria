use std::{fs, path::Path};

use chrono::{DateTime, Local, TimeZone};
use date_component::date_component;
use serde::{Deserialize, Serialize};

use self::{sucursal::Sucursal, usuario::Usuario};

pub mod usuario;
pub mod sucursal;


#[derive(Debug, Deserialize, Serialize)]
pub struct Database {
    usuarios: Vec<Usuario>,
    sucursales: Vec<Sucursal>,
}

const PATH: &str = "./db.json";

impl Database {
    pub fn cargar() -> Database {
        
        let res = fs::read_to_string(PATH);
        if let Ok(json) = res {
            if let Ok(db) = serde_json::from_str(&json) {
                db
            } else {
                Self::init()
            }
        } else {
            Self::init()
        }
    }

    // Crea una database y la guarda
    fn init() -> Database {
        let db = Database {
            usuarios: vec![],
            sucursales: vec![],
        };
        let path = Path::new(PATH);
        if path.exists() {
            log::warn!("Sobreescribiendo database anterior!");
        } else {
            log::info!("Creando una nueva database...");
        }
        db.guardar();
        db
    }
    pub fn guardar(&self) {
        let s = serde_json::to_string(self).unwrap();
        fs::write(PATH, &s).unwrap();
    }


    pub fn agregar_usuario(&mut self, dni: u64, email: String, contraseña: String, nacimiento: DateTime<Local>) -> Result<(), CrearUsuarioError> {
        if self.encontrar_dni(dni).is_some() {
            return Err(CrearUsuarioError::DNIExistente)
        }
        if self.encontrar_email(&email).is_some() {
            return Err(CrearUsuarioError::EmailExistente)
        }
        if !Self::nacimiento_valido(nacimiento) {
            return Err(CrearUsuarioError::MenorA18)
        }
        let u = Usuario::new(dni, email, contraseña, nacimiento);
        self.usuarios.push(u);
        self.guardar();
        Ok(())
    }
    pub fn encontrar_dni(&self, dni: u64) -> Option<usize> {
        self.usuarios.iter()
            // Asociar cada elemento con su id
            .enumerate()
            // Encontrar el elemento que tiene ese dni
            .find(|(_, usuario)| usuario.dni == dni)
            // Convertir el Option<(id, usuario)> en Option<id>
            .map(|(i, _)| i)
    }
    pub fn encontrar_email(&self, email: &str) -> Option<usize> {
        self.usuarios.iter()
            // Asociar cada elemento con su id
            .enumerate()
            // Encontrar el elemento que tiene ese email
            .find(|(_, usuario)| usuario.email == email)
            // Convertir el Option<(id, usuario)> en Option<id>
            .map(|(i, _)| i)
    }
    pub fn nacimiento_valido(fecha: DateTime<Local>) -> bool {
        let now = Local::now();
        let diff = date_component::calculate(&fecha, &now);
        diff.year >= 18
    }
}

#[derive(Debug)]
pub enum CrearUsuarioError {
    ErrorIndeterminado,
    DNIExistente,
    EmailExistente,
    MenorA18,
}

