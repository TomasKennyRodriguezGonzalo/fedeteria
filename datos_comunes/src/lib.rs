use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

// Con esto podemos separar los structs en distintos archivos pero que para importar siga siendo fácil
mod publicacion;
pub use publicacion::*;
mod notificacion;
pub use notificacion::*;
mod trueque;
pub use trueque::*;



#[derive(Debug, Serialize, Deserialize)]
pub struct QueryOfertasDePublicacion {
    pub id : usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseOfertasDePublicacion {
    pub ofertas : Vec<usize>,
}

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

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryAddOffice {
    pub office_to_add: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseAddOffice {
    pub agrego: bool,
    pub respuesta: Vec<Sucursal>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QueryGetUserInfo {
    pub dni: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResponseGetUserInfo {
    pub nombre_y_ap: String,
    pub email: String,
    pub nacimiento: DateTime<Local>,
    pub puntos: i64,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub enum RolDeUsuario {
    Normal,
    Dueño,
    Empleado{sucursal: usize},
}

impl RolDeUsuario {
    pub fn cambiar_rol_usuario (&mut self, new_role: RolDeUsuario) {
        *self = new_role;
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
    pub full_name: Option<String>,
    pub email: Option<String>,
    pub born_date: Option<DateTime<Local>>,

}


pub type ResponseCambiarDatosUsuario = Result<(), ErrorCambiarDatosUsuario>;

#[derive(Debug, Serialize, Deserialize)]
pub enum ErrorCambiarDatosUsuario {
    ErrorIndeterminado,
    EmailExistente,
    MenorA18,
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

pub type ResponseUnlockAccount = Result<Vec<BlockedUser>, DuringBlockError>;


#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum DuringBlockError{
    UserNotFound,
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
    pub id : usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseTogglePublicationPause {
    pub changed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryEliminarPublicacion{
    pub id: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseEliminarPublicacion{
    pub ok: bool,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct QueryGetNotificaciones{
    pub dni:u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseNotificaciones{
    pub notificaciones:Vec<usize>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct QueryEliminarNotificacion{
    pub dni:u64,
    pub index:usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseEliminarNotificacion{
    pub notificaciones:Vec<usize>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryNotificacion{
    pub dni:u64,
    pub index:usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseNotificacion{
    pub notificacion : Option<Notificacion>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryTieneNotificacion{
    pub dni:u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseTieneNotificacion{
    pub tiene_notificacion : bool
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryPublicacionesSinTasar{
    pub dni:u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponsePublicacionesSinTasar{
    pub publicaciones:Vec<usize>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryTasarPublicacion{
    pub id:usize,
    pub precio:Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseTasarPublicacion{
    //podria no tener respuesta, charlar
    pub tasado:bool
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryObtenerPrecioMaxDeRango{
    pub rango:Option<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseObtenerPrecioMaxDeRango{
    pub precio_max:Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryCrearOferta{
    pub dni_ofertante : u64,
    pub publicaciones_ofertadas : Vec<usize>,
    pub dni_receptor : u64,
    pub publicacion_receptora : usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseCrearOferta{
    pub estado : bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryObtenerTrueques {
    pub estado: EstadoTrueque,
    pub id_publicacion : Option<usize>,
    pub dni : Option<u64>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseObtenerTrueques {
    //indice de los trueques en el vec
    pub trueques: Vec<usize>
}


#[derive(Debug, Serialize, Deserialize)]
pub struct QueryObtenerTrueque {
    pub id: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryAceptarOferta {
    pub id: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryRechazarOferta {
    pub id: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseAceptarOferta {
    pub aceptada: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseRechazarOferta {
    pub rechazada: bool,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct QueryCambiarTruequeADefinido {
    pub id: usize,
    pub sucursal : String,
    pub fecha: DateTime<Local>,
    pub hora: String,
    pub minutos: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseCambiarTruequeADefinido {
    pub cambiado: bool
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryGetOffice {
    //dni del empleado o dueño
    pub dni: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseGetOffice{
    //si hay una sucursal, es empleado, sino, dueño
    pub sucursal: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseTruequePorCodigos{
    pub trueque_encontrado: Option<Vec<usize>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseFinishTrade {
    pub respuesta: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryFinishTrade{
    pub id_trueque: usize,
    pub estado: EstadoTrueque,
}