use std::{borrow::BorrowMut, collections::HashMap, fs, ops::Deref, path::Path};

use chrono::{DateTime, Local};
use date_component::date_component;
use datos_comunes::*;
use serde::{Deserialize, Serialize};
use tracing_subscriber::filter::combinator::Not;

use self::usuario::{EstadoCuenta, Usuario};

pub mod usuario;

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Database {

    usuarios: Vec<Usuario>,
    sucursales: Vec<Sucursal>,

    publicaciones_auto_incremental: usize,
    publicaciones: HashMap<usize, Publicacion>,

}

pub const BASE_DIR: &str = "./db/";
pub const DB_PATH: &str = "./db/db.json";
pub const IMGS_DIR: &str = "./db/imgs/";

impl Database {
    /// Carga la database del archivo
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

    /// Crea una database nueva y la guarda
    fn init() -> Database {
        let db: Database = Default::default();
        let path = Path::new(DB_PATH);
        if path.exists() {
            log::warn!("Sobreescribiendo database anterior! =)");
        } else {
            log::warn!("Creando una nueva database...");
        }
        db.guardar();
        db
    }

    /// Guarda la database en el archivo
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
        fecha < now && diff.year >= 18
    }
    pub fn obtener_datos_usuario(&self, indice: usize) -> &Usuario {
        &self.usuarios[indice]
    }

    pub fn obtener_rol_usuario(&self, indice: usize) -> RolDeUsuario {
        self.usuarios[indice].rol.clone()
    }

    pub fn decrementar_intentos(&mut self, indice: usize) -> Result<u8, LogInError> {
        let res = self.usuarios[indice].estado.decrementar_intentos();
        self.guardar();
        res
    }

    pub fn resetear_intentos(&mut self, indice: usize) {
        self.usuarios[indice].estado.resetear_intentos();
        self.guardar();
    }

    pub fn agregar_publicacion(&mut self, publicacion: Publicacion) {
        self.publicaciones.insert(self.publicaciones_auto_incremental, publicacion);
        self.publicaciones_auto_incremental += 1;
        self.guardar();
    }

    pub fn get_publicacion(&self, id: usize) -> Option<&Publicacion> {
        self.publicaciones.get(&id)
    }

    pub fn obtener_sucursales (&self) -> Vec<Sucursal> {
        self.sucursales.clone()
    }

    pub fn agregar_sucursal (&mut self, nueva: QueryAddOffice) -> bool {
        if self.sucursales.iter().map(|sucursal| sucursal.nombre.to_lowercase()).find(|actual| actual == &nueva.office_to_add.to_lowercase()).is_none() {
            self.sucursales.push(Sucursal { nombre: nueva.office_to_add });
            self.guardar();
            return true;
        }
        false
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

    pub fn cambiar_usuario (&mut self,
        index: usize,
        full_name: Option<String>,
        email: Option<String>,
        born_date: Option<DateTime<Local>>
    ) -> ResponseCambiarDatosUsuario {
        if let Some(born_date) = born_date {
            if !Self::nacimiento_valido(born_date) {
                return Err(ErrorCambiarDatosUsuario::MenorA18)
            }
        }
        if let Some(email) = &email {
            if let Some(index_email) = self.encontrar_email(email) {
                // No debe dar error si ingresó el mismo email (supongo, u otro error? eh)
                if index_email != index {
                    return Err(ErrorCambiarDatosUsuario::EmailExistente)
                }
            }
        }
        let usuario_a_modificar = self.usuarios.get_mut(index);
        if let Some(user) = usuario_a_modificar {
            if let Some(full_name) = full_name {
                log::info!("Nombre de usuario cambiado a : [{}]", full_name);
                user.nombre_y_apellido = full_name;
            }
            if let Some(email) = email {
                log::info!("Email de usuario cambiado a : [{}]", email);
                user.email = email;
            }
            if let Some(born_date) = born_date {
                user.nacimiento = born_date;
            }
            self.guardar();
            Ok(())
        } else{
            log::info!("backend error"); 
            Err(ErrorCambiarDatosUsuario::ErrorIndeterminado)
        }
    }

    pub fn obtener_publicaciones(&self, query: QueryPublicacionesFiltradas) -> Vec<usize> {
        let publicaciones = self.publicaciones.iter()
        .filter(|(_, p)| {
            query.filtro_dni.map(|dni| dni == p.dni_usuario).unwrap_or(true)
        })
        .filter(|(_, publication)| {
            query.filtro_nombre.as_ref()
            .map(|nombre| publication.titulo.to_lowercase().contains(&nombre.to_lowercase()))
            .unwrap_or(true)
        })
        .filter(|(_, publication)| {
            query.filtro_precio_min.as_ref().map(
                |precio| {
                    if let Some(publicacion_precio) = publication.precio {
                        publicacion_precio >= *precio
                    } else {
                        false
                    }
                }
            ).unwrap_or(true)
        })
        .filter(|(_, publication)| {
            query.filtro_precio_max.as_ref().map(
                |precio| {
                    if let Some(publicacion_precio) = publication.precio {
                        publicacion_precio <= *precio
                    } else {
                        false
                    }
                }
            ).unwrap_or(true)
        });

        //si el filtro de pausadas esta activo entonces elimino las pausadas del retorno
        if query.filtro_pausadas{
            publicaciones
            .filter(|(_,publicacion)|{
                !publicacion.pausada
            })
            .map(|(i, _)| *i)
            .collect()
        } else{
            publicaciones
            .map(|(i, _)| *i)
            .collect()
        }
           
    }
    
    pub fn obtener_usuarios_bloqueados (&self) -> Vec<BlockedUser> {
        self.usuarios.iter().filter(|usuario| usuario.estado.esta_bloqueada())
                            .map(|usuario| BlockedUser { nombre: usuario.nombre_y_apellido.clone(), dni: usuario.dni.clone()})
                            .collect()
    }

    pub fn desbloquear_cuenta (&mut self, cuenta: QueryUnlockAccount) -> Result<Vec<BlockedUser>, DuringBlockError> {
        let index = self.usuarios.iter()
            .filter(|usuario| usuario.estado == EstadoCuenta::Bloqueada)
            .position(|usuario| usuario.dni == cuenta.dni);
        log::info!("el inedx encontrado es: {:?}",index);
        if index.is_none(){
            return Err(DuringBlockError::UserNotFound);
        }
        let unlock_index= self.usuarios
        .iter()
        .position(|user| user.dni == cuenta.dni)
        .unwrap();

        self.usuarios.get_mut(unlock_index).unwrap().estado.desbloquear();
        self.guardar();
        let nuevos_usuarios_bloqueados = self.obtener_usuarios_bloqueados();
        Ok(nuevos_usuarios_bloqueados)
    }

    pub fn cambiar_rol_usuario (&mut self, query: QueryChangeUserRole) -> bool {
        let index = self.usuarios.iter().position(|usuario| usuario.dni == query.dni).unwrap();
        self.usuarios.get_mut(index).unwrap().rol.cambiar_rol_usuario(query.new_role);
        self.guardar();
        true
    }

    pub fn alternar_pausa_publicacion (&mut self, id : &usize) {
        self.publicaciones.get_mut(id).unwrap().alternar_pausa();
        self.guardar();
    }

    pub fn eliminar_publicacion (&mut self, id : usize)->bool{
        self.publicaciones.remove(&id);
        self.guardar();
        true
    }

    pub fn obtener_notificaciones(&mut self, query:&QueryGetNotificaciones)->Vec<usize>{
        let index = self.usuarios.iter().position(|usuario| usuario.dni == query.dni).unwrap();
        let notificaciones = self.usuarios.get(index).unwrap().notificaciones.clone();
        self.guardar();
        notificaciones.iter().enumerate().map(|(i, _)| i).collect()
    }

    pub fn get_notificacion(&self, query : &QueryNotificacion) -> Option<Notificacion> {
        if let Some(usuario) = self.usuarios.clone().iter().find(|usuario| usuario.dni == query.dni) {
            if let Some(notificacion) = usuario.notificaciones.get(query.index) {
                return Some(notificacion.clone());
            }
        }
        None
    }

    pub fn eliminar_notificacion(&mut self, query:&QueryEliminarNotificacion) -> Vec<usize>{
        if let Some(usuario) = self.usuarios.iter_mut().find(|usuario| usuario.dni == query.dni) {
            if let Some(notificacion) = usuario.notificaciones.get_mut(query.index) {
                usuario.notificaciones.remove(query.index);
            }
        }
        let usuario = self.usuarios.iter().find(|usuario| usuario.dni == query.dni);
        let notificaciones = usuario.unwrap().notificaciones.clone();
        self.guardar();
        notificaciones.iter().enumerate().map(|(i, _)| i).collect()
    }


    pub fn obtener_publicaciones_sin_tasar(&self) -> Vec<usize> {
        let lista = self.publicaciones
            .iter()
            .filter(|(_, publicacion)| publicacion.precio.is_none())
            .map(|(i, _)| *i)
            .collect();
        log::info!("{lista:?}");
        lista
    }


    pub fn tasar_publicacion(&mut self, query:QueryTasarPublicacion)-> bool{
        let publicacion = self.publicaciones
        .get_mut(&query.id);

        if let Some(publicacion) = publicacion {
            publicacion.precio = query.precio;
            self.guardar();  
            true
        } else {
            false
        }
    }


    pub fn enviar_notificacion(&mut self, query:QueryEnviarNotificacion, index:Option<usize>)-> bool{
        if index.is_none(){
            log::error!("index de usuario inexistente!");
            return false
        }
        let usuario = self.usuarios.get_mut(index.unwrap());
        let nueva_notificacion = Notificacion{
            titulo : query.titulo,
            detalle : query.detalle,
            url : query.url,
        };
        usuario.unwrap().notificaciones.push(nueva_notificacion);
        self.guardar();
        true
    }




}
