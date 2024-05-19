use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

// Con esto podemos separar los structs en distintos archivos pero que para importar siga siendo fácil
mod publicacion;
pub use publicacion::*;

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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QueryGetUserInfo {
    pub dni:u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResponseGetUserInfo {
    pub nombre_y_ap:String,
    pub email:String,
    pub nacimiento: DateTime<Local>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub enum RolDeUsuario {
    Normal,
    Dueño,
    Empleado{sucursal: usize},
}

impl RolDeUsuario {
    pub fn cambiar_rol_usuario (&mut self, new_role: RolDeUsuario) -> bool {
        *self = new_role;
        true
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QueryGetUserRole {
    pub dni:u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResponseGetUserRole {
    pub rol : RolDeUsuario,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QueryCambiarDatosUsuario{
    pub dni:u64,
    pub full_name:String,
    pub email:String,
    pub born_date:DateTime<Local>,

}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResponseCambiarDatosUsuario{
    pub datos_cambiados:bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResponseObtenerPublicacioneseUsuario{
    pub datos_cambiados:bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BlockedUser {
    pub nombre : String,
    pub dni: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResponseGetBlockedAccounts {
    pub blocked_users : Vec<BlockedUser>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryUnlockAccount {
    pub dni: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResponseUnlockAccount {
    pub blocked_users : Vec<BlockedUser>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryChangeUserRole {
    pub dni: u64,
    pub new_role: RolDeUsuario,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseChangeUserRole {
    pub changed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryTogglePublicationPause {
    pub id : String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseTogglePublicationPause {
    pub changed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryEliminarPublicacion{
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseEliminarPublicacion{
    pub ok: bool,
}
