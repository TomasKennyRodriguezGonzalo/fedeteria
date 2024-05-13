use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryRegistrarUsuario {
    pub nombre_y_apellido: String,
    pub dni: u64,
    pub email: String,
    pub contraseña: String,
    pub nacimiento: DateTime<Local>,
}


pub type ResponseRegistrarUsuario = Result<(), CrearUsuarioError>;

#[derive(Debug, Serialize, Deserialize)]
pub enum CrearUsuarioError {
    ErrorIndeterminado,
    DNIExistente,
    EmailExistente,
    MenorA18,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryObtenerUsuario {
    pub dni:u64
}


#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseObtenerUsuario {
    pub nombre:String
}

#[derive(Debug, Serialize, Deserialize,Clone)]
pub enum LogInError {
    UserNotFound,
    BlockedUser,
    IncorrectPassword{ intentos : u8 },
}

pub type ResponseLogIn = Result<ResponseStatus, LogInError>;


#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseStatus{
    pub status:bool
}


#[derive(Debug, Serialize, Deserialize)]
pub struct QueryLogin {
    pub dni: u64,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize ,PartialEq, Clone)]
pub struct Sucursal {
    pub nombre: String,
}

impl Sucursal {
    pub fn new (nombre: String) -> Sucursal {
        Sucursal {nombre}
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseGetOffices {
    pub office_list: Vec<Sucursal>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryDeleteOffice {
    pub office_to_delete: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseDeleteOffice {
    pub respuesta: Vec<Sucursal>,
}

