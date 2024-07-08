use chrono::{Date, DateTime, Local};
use serde::{Deserialize, Serialize};

// Con esto podemos separar los structs en distintos archivos pero que para importar siga siendo fácil
mod peticion_cambio_contrasenia;
pub use peticion_cambio_contrasenia::*;
mod publicacion;
pub use publicacion::*;
mod notificacion;
pub use notificacion::*;
mod trueque;
pub use trueque::*;
mod sucursal;
pub use sucursal::*;
mod estadisticas;
pub use estadisticas::*;
mod descuento;
pub use descuento::*;
mod tarjeta;
pub use tarjeta::*;



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

/*impl Sucursal {
    pub fn new (nombre: String, ) -> Sucursal {
        Sucursal {nombre}
    }
}*/

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseGetOffices {
    pub office_list: Vec<Sucursal>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryDeleteOffice {
    pub office_to_delete: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseDeleteOffice {
    pub sucursales: Vec<Sucursal>,
    pub eliminada: bool,
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
    pub puntos: u64,
    pub promedio_calificaciones: f64,
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
    pub sucursal : usize,
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
    //idice de la sucursal
    pub index: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseGetOffice{
    pub sucursal: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseTruequePorCodigos{
    pub trueque_encontrado: Option<Vec<usize>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseFinishTrade {
    pub respuesta: Result<bool,ErrorEnConcretacion>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryFinishTrade{
    pub id_trueque: usize,
    //pub ganancias: u64,
    pub estado: EstadoTrueque,
    pub ventas_ofertante: u64,
    pub ventas_receptor:u64,
    pub codigo_descuento_ofertante: Option<String>,
    pub codigo_descuento_receptor: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct QueryAskQuestion{
    pub dni_preguntante:u64,
    pub pregunta:String,
    pub id_publicacion:usize,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseAskQuestion{
    pub ok:bool,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct QueryAnswerQuestion{
    pub indice_pregunta:usize,
    pub id_publicacion: usize,
    pub respuesta:String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseAnswerQuestion{
    pub ok:bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryAgregarAGuardados{
    pub dni:u64,
    pub id_publicacion:usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseAgregarAGuardados{
    pub ok:bool,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct QueryPublicacionGuardada{
    pub dni:u64,
    pub id_publicacion:usize,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ResponsePublicacionGuardada{
    pub guardada:bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryEliminarGuardados{
    pub dni:u64,
    pub id_publicacion:usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseEliminarGuardados{
    pub ok:bool,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct QueryObtenerGuardadas{
    pub dni:u64,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseObtenerGuardadas{
    pub publicaciones_guardadas:Vec<usize>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryObtenerPreferencias{
    pub dni: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseObtenerPreferencias{
    pub preferencias : (Option<String>, Option<String>)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryActualizarPreferencias{
    pub dni: u64,
    pub preferencias : (Option<String>, Option<String>)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseActualizarPreferencias{}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuerySendChangePasswordCode{
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseSendChangePasswordCode{}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryValidarCambioContrasenia{
    pub email: String,
    pub codigo: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseValidarCambioContrasenia{
    pub datos_validos: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryCambioContraseniaLogIn{
    pub email: String,
    pub codigo: u64,
    pub nueva_contrasenia: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryCambioContraseniaPerfil{
    pub nueva_contrasenia: String,
    pub vieja_contrasenia: String,
    pub dni: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseCambioContrasenia{
    pub cambio: bool,
}

#[derive(Debug, Serialize, Deserialize ,PartialEq)]
pub struct QueryPagarPromocionPublicaciones {
    pub publicaciones: String,
    pub fecha_fin_promocion: DateTime<Local>,
    pub precio: u64,
}
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum ErrorEnConcretacion {
    DescuentoOfertanteInvalido,
    DescuentoOfertanteUtilizado,
    DescuentoOfertanteVencido,
    OfertanteNivelInsuficiente,
    DescuentoReceptorUtilizado,
    DescuentoReceptorVencido,
    DescuentoReceptorInvalido,
    ReceptorNivelInsuficiente,
}

impl ErrorEnConcretacion {
    pub fn traducir_a_receptor(self, traducir: bool) -> ErrorEnConcretacion {
        if traducir {
            match self {
                ErrorEnConcretacion::DescuentoOfertanteInvalido => ErrorEnConcretacion::DescuentoReceptorUtilizado,
                ErrorEnConcretacion::DescuentoOfertanteUtilizado => ErrorEnConcretacion::DescuentoReceptorVencido,
                ErrorEnConcretacion::DescuentoOfertanteVencido => ErrorEnConcretacion::DescuentoReceptorInvalido,
                ErrorEnConcretacion::OfertanteNivelInsuficiente => ErrorEnConcretacion::ReceptorNivelInsuficiente,
                _ => panic!("AAAA"),
            }
        } else {
            self
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryCreateDiscount{
    pub codigo_descuento : String,
    pub porcentaje : f64,
    pub reembolso_max : u64,
    pub nivel_min : u64,
    pub fecha_exp : Option<DateTime<Local>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseCreateDiscount{
    pub ok:bool,
}



#[derive(Debug, Serialize, Deserialize)]
pub struct QueryEliminarDescuento{
    pub index:usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseEliminarDescuento{
    pub ok:bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryObtenerDescuentos{
    pub nada:bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseObtenerDescuentos{
    pub descuentos:Vec<(usize, Descuento)>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct QueryObtenerDescuento{
    pub id:usize,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseObtenerDescuento{
    pub descuento:Descuento,

}
#[derive(Debug, Serialize, Deserialize)]
pub struct QueryGetUserDiscounts{
    pub dni : u64,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseGetUserDiscounts{
    pub discounts : Vec<Descuento>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryCalificarOfertante{
    pub dni:u64,
    pub calificacion:Option<u64>,
    pub id_trueque:usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseCalificarOfertante{
    pub ok:bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryCalificarReceptor{
    pub dni:u64,
    pub calificacion:Option<u64>,
    pub id_trueque:usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseCalificarReceptor{
    pub ok:bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryPagarPromocion {
    pub tarjeta: Tarjeta,
    pub precio: u64,
    pub publicaciones: Vec<usize>,
    pub fecha_limite_promocion: DateTime<Local>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponsePagarPromocion {
    pub pago: bool,
}