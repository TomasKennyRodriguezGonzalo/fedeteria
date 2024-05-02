use chrono::{DateTime, Local, TimeZone};
use date_component::date_component;

use self::usuario::Usuario;

pub mod usuario;


pub struct Database {
    usuarios: Vec<Usuario>,
}

impl Database {

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
        Ok(())
    }
    fn encontrar_dni(&self, dni: u64) -> Option<usize> {
        self.usuarios.iter()
            // Asociar cada elemento con su id
            .enumerate()
            // Encontrar el elemento que tiene ese dni
            .find(|(_, usuario)| usuario.dni == dni)
            // Convertir el Option<(id, usuario)> en Option<id>
            .map(|(i, _)| i)
    }
    fn encontrar_email(&self, email: &str) -> Option<usize> {
        self.usuarios.iter()
            // Asociar cada elemento con su id
            .enumerate()
            // Encontrar el elemento que tiene ese email
            .find(|(_, usuario)| usuario.email == email)
            // Convertir el Option<(id, usuario)> en Option<id>
            .map(|(i, _)| i)
    }
    fn nacimiento_valido(fecha: DateTime<Local>) -> bool {
        let now = Local::now();
        let diff = date_component::calculate(&fecha, &now);
        diff.year >= 18
    }
}

enum CrearUsuarioError {
    ErrorIndeterminado,
    DNIExistente,
    EmailExistente,
    MenorA18,
}

