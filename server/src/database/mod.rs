use std::{fs, path::Path};

use chrono::{DateTime, Local};
use date_component::date_component;
use datos_comunes::{CrearUsuarioError, LogInError, QueryRegistrarUsuario, ResponseRegistrarUsuario,Sucursal,QueryDeleteOffice, RolDeUsuario};
use serde::{Deserialize, Serialize};

use self::usuario::Usuario;

pub mod usuario;


#[derive(Debug, Deserialize, Serialize)]
pub struct Database {
    usuarios: Vec<Usuario>,
    sucursales: Vec<Sucursal>,
}

pub const BASE_DIR: &str = "./db/";
pub const DB_PATH: &str = "./db/db.json";
pub const IMGS_DIR: &str = "./db/imgs/";

impl Database {
    pub fn cargar() -> Database {
        
        let res = fs::read_to_string(DB_PATH);
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
        let path = Path::new(DB_PATH);
        if path.exists() {
            log::warn!("Sobreescribiendo database anterior!");
        } else {
            log::warn!("Creando una nueva database...");
        }
        db.guardar();
        db
    }
    pub fn guardar(&self) {
        let s = serde_json::to_string_pretty(self).unwrap();
        std::fs::create_dir_all(BASE_DIR).unwrap();
        std::fs::create_dir_all(IMGS_DIR).unwrap();
        fs::write(Path::new("./db/db.json"), s).unwrap();
    }


    pub fn agregar_usuario(&mut self, datos: QueryRegistrarUsuario) -> ResponseRegistrarUsuario {
        if !Self::nacimiento_valido(datos.nacimiento) {
            return Err(CrearUsuarioError::MenorA18)
        }
        if self.encontrar_dni(datos.dni).is_some() {
            return Err(CrearUsuarioError::DNIExistente)
        }
        if self.encontrar_email(&datos.email).is_some() {
            return Err(CrearUsuarioError::EmailExistente)
        }
        let u = Usuario::new(datos.nombre_y_apellido, datos.dni, datos.email, datos.contraseña, datos.nacimiento);
        self.usuarios.push(u);
        self.guardar();
        Ok(())
    }
    pub fn get_ultimo_usuario(&self) -> &Usuario {
        &self.usuarios[self.usuarios.len() - 1]
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
    pub fn obtener_datos_usuario(&self, indice:usize) -> &Usuario {
        &self.usuarios[indice]
    }

    pub fn obtener_rol_usuario(&self, indice:usize) -> RolDeUsuario {
        self.usuarios[indice].rol.clone()
    }

    pub fn decrementar_intentos(&mut self, indice:usize)-> Result<u8, LogInError>{
        let res = &self.usuarios[indice].estado.decrementar_intentos();
        self.guardar();
        res.clone()
    }

    pub fn resetear_intentos(&mut self, indice:usize){
        self.guardar();
        self.usuarios[indice].estado.resetear_intentos();
    }

    pub fn obtener_sucursales (&self) -> Vec<Sucursal> {
        self.sucursales.clone()
    }

    pub fn eliminar_sucursal (&mut self, eliminar: QueryDeleteOffice) -> Vec<Sucursal> {
        let ubicacion = self.sucursales.iter().
                                    position(|actual| actual.nombre == eliminar.office_to_delete);

        if let Some (i_eliminar) = ubicacion {
            self.sucursales.remove(i_eliminar);
            self.guardar();
        }

        self.sucursales.clone()
    }
}
