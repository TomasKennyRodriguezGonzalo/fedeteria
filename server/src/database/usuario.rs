use std::hash::{DefaultHasher, Hash, Hasher};

use chrono::{DateTime, Local};
use datos_comunes::{LogInError, Notificacion, Publicacion, RolDeUsuario};
use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Usuario {
    pub nombre_y_apellido: String,
    pub dni: u64,
    pub email: String,
    pub contraseña: u64,
    pub nacimiento: DateTime<Local>,
    pub rol: RolDeUsuario,
    pub estado: EstadoCuenta,
    pub notificaciones: Vec<Notificacion>,
    pub puntos: u64,
    pub publicaciones_guardadas:Vec<usize>,
    pub descuentos_utilizados: Vec<usize>, 
    pub preferencias: (Option<String>, Option<String>),
}



#[derive(Debug, Deserialize, Serialize,Clone,PartialEq)]
pub enum EstadoCuenta {
    Activa{intentos: u8},
    Bloqueada,
}

impl EstadoCuenta{
    pub fn decrementar_intentos(&mut self)->Result<u8,LogInError>{
        match self {
            EstadoCuenta::Activa { ref mut intentos } => {
                if *intentos > 0 {
                    *intentos -= 1;              
                }
                if *intentos == 0 {
                    *self = EstadoCuenta::Bloqueada;     
                    return Err(LogInError::BlockedUser)         
                }
                Ok(*intentos)
            }
            EstadoCuenta::Bloqueada => {
                Err(LogInError::BlockedUser)
            }
        }
    }

    pub fn resetear_intentos(&mut self){
        *self = EstadoCuenta::Activa { intentos: 3 }
    }

    pub fn esta_bloqueada (&self) -> bool {
        match self {
            EstadoCuenta::Activa { .. } => false,
            EstadoCuenta::Bloqueada => true,
        }
    }

    pub fn desbloquear (&mut self) {
        self.resetear_intentos();
    }
}

impl Usuario {
    pub fn new(nombre_y_apellido: String, dni: u64, email: String, contraseña: String, nacimiento: DateTime<Local>) -> Self {
        let contraseña = hash_str(&contraseña);
        Usuario {
            nombre_y_apellido,
            dni,
            email,
            contraseña,
            nacimiento,
            rol: RolDeUsuario::Normal,
            estado: EstadoCuenta::Activa { intentos: 3 },
            notificaciones : Vec::new(),
            puntos : 0,
            publicaciones_guardadas:Vec::new(),
            preferencias: (None, None),
            descuentos_utilizados:Vec::new(),
        }
    }
    pub fn sumar_punto(&mut self){
        self.puntos+=1;
    }
}


fn hash_str(s: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    s.hash(&mut hasher);
    hasher.finish()
}

