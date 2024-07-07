use std::{collections::HashMap, fs, ops::Deref, path::Path};

use chrono::{DateTime, Local, TimeZone};
use date_component::date_component;
use datos_comunes::*;
use log::info;
use serde::{Deserialize, Serialize};
use rand::prelude::*;
use tracing_subscriber::fmt::format;
use crate::{hash_str, mail::send_email};
use mpago::{client::MercadoPagoClientBuilder, payments::types::PaymentCreateOptions,payments::PaymentCreateBuilder};
use rust_decimal::Decimal;
use rust_decimal::prelude::FromPrimitive;

use self::usuario::{EstadoCuenta, Usuario};

pub mod usuario;







#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Database {

    usuarios: Vec<Usuario>,

    sucursales_auto_incremental: usize,
    sucursales: Vec<Sucursal>,

    publicaciones_auto_incremental: usize,
    publicaciones: HashMap<usize, Publicacion>,

    trueques_auto_incremental: usize,
    trueques: HashMap<usize, Trueque>,

    peticiones_cambio_contraseña: Vec<PeticionCambioContrasenia>,

    descuentos:Vec<Descuento>,
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
        let db: Database = get_database_por_defecto();
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

    pub fn agregar_trueque(&mut self, trueque: Trueque) -> usize {
        self.trueques.insert(self.trueques_auto_incremental, trueque);
        let auto_incremental_viejo = self.trueques_auto_incremental;
        self.trueques_auto_incremental += 1;
        self.guardar();
        auto_incremental_viejo
    }

    pub fn get_publicacion(&self, id: usize) -> Option<&Publicacion> {
        self.publicaciones.get(&id)
    }

    pub fn obtener_sucursales_activas (&self) -> Vec<Sucursal> {
        let sucursales_activas: Vec<Sucursal> = self.sucursales.iter().filter(|sucursal| sucursal.esta_activa).map(|sucursal| sucursal.clone()).collect();
        sucursales_activas
    }

    pub fn agregar_sucursal (&mut self, nueva: QueryAddOffice) -> bool {
        //ver si se puede agregar una sucursal con el mismo nombre que una "eliminada"
        if self.sucursales.iter().filter(|sucursal| sucursal.esta_activa).map(|sucursal| sucursal.nombre.to_lowercase()).find(|actual| actual == &nueva.office_to_add.to_lowercase()).is_none() {
            self.sucursales.push(Sucursal { nombre: nueva.office_to_add, esta_activa: true, id: self.sucursales_auto_incremental});
            self.sucursales_auto_incremental += 1;
            self.guardar();
            return true;
        }
        false
    }
    
    pub fn eliminar_sucursal (&mut self, eliminar: QueryDeleteOffice) -> (Vec<Sucursal>, bool) {
        //verifico si la sucursal tiene empleados. De tener, no la elimina
        if let Some (_empleado) = self.usuarios.iter().find(|usuario| usuario.rol == RolDeUsuario::Empleado { sucursal: eliminar.office_to_delete }) {
            return (self.obtener_sucursales_activas(), false);
        }

        //"elimino" la sucursal
        let ubicacion = self.sucursales.iter().
                                    position(|actual| actual.id == eliminar.office_to_delete);

        if let Some (indice) = ubicacion {
            self.sucursales[indice].esta_activa = false;
            self.guardar();
        }

        (self.obtener_sucursales_activas(), true)
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
        .filter(|(_, p)| !p.eliminada)
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
    pub fn eliminar_publicacion (&mut self, id : usize)->bool{
        if (self.publicaciones.get(&id).unwrap().ofertas.is_empty()) || (!Database::hay_trueques_activos(self.publicaciones.get(&id).unwrap().ofertas.clone(), &self.trueques.clone())){
            self.publicaciones.get_mut(&id).unwrap().eliminada = true;
            self.guardar();
            return true;
        }
        false
    }

    fn hay_trueques_activos(trueques_a_verificar: Vec<usize>, trueques: &HashMap<usize, Trueque>) -> bool {
        for id_trueque in trueques_a_verificar {
            if (trueques.get(&id_trueque).unwrap().estado == EstadoTrueque::Oferta) || (trueques.get(&id_trueque).unwrap().estado == EstadoTrueque::Definido) || (trueques.get(&id_trueque).unwrap().estado == EstadoTrueque::Pendiente) {
                return true;
            }
        }
        false
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
            if let Some(_notificacion) = usuario.notificaciones.get_mut(query.index) {
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


    pub fn tasar_publicacion(&mut self, query: &QueryTasarPublicacion)-> bool{
        let publicacion = self.publicaciones
        .get_mut(&query.id);

        if let Some(publicacion) = publicacion {
            publicacion.precio = query.precio;
            publicacion.pausada = false;
            self.guardar();  
            true
        } else {
            false
        }
    }

    pub fn enviar_notificacion(&mut self, indice_usuario_receptor: usize, titulo: String, detalle: String, url: String) {
        let nueva_notificacion = Notificacion {
            titulo,
            detalle,
            url,
        };
        self.usuarios[indice_usuario_receptor].notificaciones.push(nueva_notificacion);
        self.guardar();
    }

    pub fn crear_oferta(&mut self, query:QueryCrearOferta) -> Option<usize> {
        if let Some(_) = self.encontrar_dni(query.dni_receptor) {
            // Crea el Trueque en estado de Oferta
            let oferta = Trueque {
                oferta: (query.dni_ofertante, query.publicaciones_ofertadas.clone()),
                receptor: (query.dni_receptor, query.publicacion_receptora),
                sucursal: None,
                fecha_pactada: None,
                fecha_trueque: None,
                hora: None,
                minutos: None,
                estado: EstadoTrueque::Oferta,
                codigo_ofertante: None,
                codigo_receptor: None,
                valido: true,
                ventas_ofertante:None,
                ventas_receptor:None,
                calificacion_ofertante:None,
                calificacion_receptor:None,
            };
            let index = self.agregar_trueque(oferta);

            // Si la publicación existe, que se asume que si, se agrega la referencia al trueque
            //publicacion receptora
            if let Some(publicacion_receptor) = self.publicaciones.get_mut(&query.publicacion_receptora){
                publicacion_receptor.ofertas.push(index);

                //publicaciones ofertantes
                //publicacion 1, no verifico que exista ya que se asume que si exisitrá al menos una publicacion
                if let Some(publicacion_ofertante_1) = self.publicaciones.get_mut(&query.publicaciones_ofertadas.clone().get(0).unwrap()){
                    publicacion_ofertante_1.ofertas.push(index);
                }

                //publicacion 2, aqui verifico ya que puede no existir
                if let Some(indice_publi_2) = query.publicaciones_ofertadas.clone().get(1) {
                    if let Some(publicacion_ofertante_2) = self.publicaciones.get_mut(indice_publi_2){
                        publicacion_ofertante_2.ofertas.push(index);
                    }
                }
                self.guardar();
                return Some(index);
            }
            
        }
        return None
    }


    pub fn obtener_trueques (&self, query: QueryTruequesFiltrados) -> Vec<usize> {
        let obtenidos = self.trueques.iter().
                    filter(|(_, trueque)| trueque.estado != EstadoTrueque::Cancelado).
                    filter(|(_, trueque)| {
                        query.filtro_estado.as_ref().map(|estado_trueque| estado_trueque == &trueque.estado)
                        .unwrap_or(true)
                    }).
                    filter(|(_,trueque)|{
                        query.filtro_id_publicacion.map(|publicacion| (trueque.receptor.1 == publicacion) || (trueque.oferta.1.contains(&publicacion)))
                        .unwrap_or(true)
                    }).
                    filter(|(_, trueque)| {
                        query.filtro_dni_integrantes.map(|dni_a_buscar| (trueque.oferta.0 == dni_a_buscar) || (trueque.receptor.0 == dni_a_buscar))
                        .unwrap_or(true)
                    }).
                    filter(|(_, trueque)| {
                        query.filtro_sucursal.as_ref().map(|sucursal_filtro| {
                            if let Some(sucursal) = &trueque.sucursal {
                                sucursal_filtro == sucursal
                            }
                            else {
                                false
                            }
                        })
                        .unwrap_or(true)
                    });
        let respuesta = obtenidos.map(|(i, _)| *i).collect();
        respuesta
    }

    pub fn validar_trueque(&mut self, id: usize) {
        self.resolver_estado_trueque(&id);
    }
    pub fn get_trueque(&self, id: usize) -> Option<&Trueque> {
        self.trueques.get(&id)
    }

    pub fn get_estadisticas(&self, mut query: QueryEstadisticas) -> Option<ResponseEstadisticas> {
        let usuario_id = self.encontrar_dni(query.dni?)?;
        let rol = self.obtener_datos_usuario(usuario_id).rol.clone();
        if let RolDeUsuario::Empleado { sucursal } = rol {
            query.id_sucursal = Some(sucursal);
        }

        let mut cantidad_trueques_rechazados = 0;
        let mut cantidad_trueques_finalizados = 0;
        let mut cantidad_trueques_rechazados_con_ventas = 0;
        let mut pesos_trueques_rechazados = 0;
        let mut cantidad_trueques_finalizados_con_ventas = 0;
        let mut pesos_trueques_finalizados = 0;

        let fecha_entre = |fecha| {
            if let Some(fecha_min) = query.fecha_inicial {
                if fecha < fecha_min {
                    return false;
                }
            }
            if let Some(fecha_max) = query.fecha_final {
                if fecha > fecha_max {
                    return false;
                }
            }
            true
        };

        for trueque in self.trueques.values() {
            if trueque.sucursal == query.id_sucursal || query.id_sucursal.is_none() {
                if let Some(fecha_trueque) = trueque.fecha_trueque {
                    if !fecha_entre(fecha_trueque) {continue;}
                    // EY. ATENCIÓN: Se deberían tener en cuenta los trueques rechazados pero con ventas?
                    if trueque.estado == EstadoTrueque::Finalizado {
                        cantidad_trueques_finalizados += 1;
                        if trueque.ventas_ofertante.is_some() || trueque.ventas_receptor.is_some() {
                            cantidad_trueques_finalizados_con_ventas += 1;
                        }
                        pesos_trueques_finalizados += trueque.ventas_ofertante.unwrap_or(0) + trueque.ventas_receptor.unwrap_or(0);
                    }
                    if trueque.estado == EstadoTrueque::Rechazado {
                        cantidad_trueques_rechazados += 1;
                        if trueque.ventas_ofertante.is_some() || trueque.ventas_receptor.is_some() {
                            cantidad_trueques_rechazados_con_ventas += 1;
                        }
                        pesos_trueques_rechazados += trueque.ventas_ofertante.unwrap_or(0) + trueque.ventas_receptor.unwrap_or(0);
                    }
                }
            }
        }
        let query_nombre_sucursal = query.id_sucursal.map(|id| {
            self.obtener_sucursal(id)
        });
        Some(ResponseEstadisticas {
            cantidad_trueques_rechazados,
            cantidad_trueques_finalizados,
            cantidad_trueques_rechazados_o_finalizados: cantidad_trueques_rechazados + cantidad_trueques_finalizados,
            cantidad_trueques_finalizados_con_ventas,
            cantidad_trueques_rechazados_con_ventas,
            cantidad_trueques_con_ventas: cantidad_trueques_rechazados_con_ventas + cantidad_trueques_finalizados_con_ventas,
            pesos_trueques_rechazados,
            pesos_trueques_finalizados,
            pesos_trueques: pesos_trueques_rechazados + pesos_trueques_finalizados,
            query_fecha_inicial: query.fecha_inicial,
            query_fecha_final: query.fecha_final,
            query_nombre_sucursal,
        })
    }

    pub fn aceptar_oferta(&mut self, id:usize) -> bool {
        let trueque = self.trueques.get_mut(&id);
        if let Some(trueque) = trueque {
            if !trueque.valido {
                return false;
            }
            // aca se modificaria la variable de "en trueque"
            let publicacion_receptora = self.publicaciones.get_mut(&trueque.receptor.1);
            publicacion_receptora.unwrap().pausada = true; 
            let publicacion_receptora = self.publicaciones.get_mut(&trueque.receptor.1);
            publicacion_receptora.unwrap().en_trueque = true;
            if let Some(publi1) = trueque.oferta.1.get(0) {
                self.publicaciones.get_mut(publi1).unwrap().pausada = true;
                self.publicaciones.get_mut(publi1).unwrap().en_trueque = true;
            }
            else {
                log::info!("No hay publicacion 1");
            }
            if let Some(publi2) = trueque.oferta.1.get(1) {
                self.publicaciones.get_mut(publi2).unwrap().pausada = true;
                self.publicaciones.get_mut(publi2).unwrap().en_trueque = true;
            }
            else {
                log::info!("No hay publicacion 2");
            }
            trueque.estado = EstadoTrueque::Pendiente;
            for publicacion in trueque.get_publicaciones() {
                self.resolver_estado_trueques_de_publicacion(&publicacion);
            }
            self.guardar();
            return true;
        }
        false
    }

    /// Retorna todos los trueques en los que están
    fn trueques_de_publicacion(&self, id: &usize) -> Vec<usize> {
        let mut trueques = vec![];
        for (id_trueque, trueque) in self.trueques.iter() {
            if trueque.get_publicaciones().contains(id) {
                trueques.push(*id_trueque);
            }
        }
        trueques
    }
    fn resolver_estado_trueques_de_publicacion(&mut self, id: &usize) {
        for trueque in self.trueques_de_publicacion(id) {
            self.resolver_estado_trueque(&trueque);
        }
    }
    // invalida una oferta si alguna de sus publicaciones está en otro trueque. la deja valida en caso contrario.
    fn resolver_estado_trueque(&mut self, id: &usize) {
        let trueque = self.trueques.get(id).unwrap();
        match trueque.estado {
            EstadoTrueque::Oferta => {},
            EstadoTrueque::Pendiente => {return; },
            EstadoTrueque::Definido => {return; },
            EstadoTrueque::Finalizado => {return; },
            EstadoTrueque::Rechazado => {},//{return; },
            EstadoTrueque::Cancelado => {},
        }
        let mut todo_mal = false;
        for id_publicacion in trueque.get_publicaciones() {
    // para cada publicación, si está en un trueque cuyo estado es 
    // Pendiente,
    // Definido,
    // Finalizado,
    // tonces todo mal
            for id_otro_trueque in self.trueques_de_publicacion(&id_publicacion) {
                if id_otro_trueque == *id {continue; }
                let otro_trueque = self.trueques.get(&id_otro_trueque).unwrap();
                match otro_trueque.estado {
                    EstadoTrueque::Oferta => {},
                    EstadoTrueque::Pendiente => {todo_mal = true},
                    EstadoTrueque::Definido => {todo_mal = true},
                    EstadoTrueque::Finalizado => {todo_mal = true},
                    EstadoTrueque::Rechazado => {},
                    EstadoTrueque::Cancelado => {},
                }
            }
        }
        self.trueques.get_mut(id).unwrap().valido = !todo_mal;
    }

    pub fn rechazar_oferta(&mut self, id:usize) -> bool {
        let trueque = self.trueques.get_mut(&id);
        if let Some(trueque) = trueque {
            //por cada publicacion del trueque, modifico los booleanos "en_trueque" y "pausada" para que puedan 
            //volver a realizar trueques
            for publicacion in trueque.get_publicaciones() {
                self.publicaciones.get_mut(&publicacion).unwrap().en_trueque = false;
                self.publicaciones.get_mut(&publicacion).unwrap().pausada = false;
                self.resolver_estado_trueques_de_publicacion(&publicacion);
            }
            //elimino logicamente el trueque de la DB
            self.trueques.get_mut(&id).unwrap().estado = EstadoTrueque::Cancelado;
            self.guardar();
            return true;
        }
        false
    }

    //Lo comentado en esta funcion con respectoa  codigo, es para la verificacion de que no exista un trueque en la misma sucursal, fecha
    //y hora. No es necesario, pero lo dejo ya que esta hecho
    pub fn cambiar_trueque_a_definido(&mut self, query:QueryCambiarTruequeADefinido) -> (bool, Option<Vec<String>>) {
        //let trueques_copia = self.trueques.clone();
        
        //los hago antes a los codigos porque tira error de borrowing
        //codigos.0 ----> codigo_receptor
        //codigos.1 ----> codigo_ofertante
        let codigos = self.generar_codigos_de_trueque() ;
        let trueque = self.trueques.get_mut(&query.id);
        if let Some(trueque) = trueque {
            /*let hay_otros_trueques = trueques_copia.iter().
                filter(|(_, trueque)| {
                    trueque.sucursal.as_ref().map(|sucursal| sucursal == &query.sucursal)
                    .unwrap_or(false)
                }).
                filter(|(_, trueque)| {
                    trueque.fecha.map(|fecha| fecha == query.fecha)
                    .unwrap_or(false)
                }).
                filter(|(_, trueque)| {
                    trueque.hora.as_ref().map(|hora| hora == &query.hora)
                    .unwrap_or(false)
                }).
                filter(|(_, trueque)| {
                    trueque.minutos.as_ref().map(|minutos| minutos == &query.minutos)
                    .unwrap_or(false)
                });
            let hay_iguales: Vec<usize> = hay_otros_trueques.map(|(i, _)| *i).collect();
            log::info!("Trueques en misma hora y fecha: {}", hay_iguales.len());*/
           // if hay_iguales.is_empty() {
                //los hago antes a los codigos porque tira error de borrowing
                //codigos.0 ----> codigo_receptor
                //codigos.1 ----> codigo_ofertante
                trueque.estado = EstadoTrueque::Definido;
                trueque.fecha_pactada = Some(query.fecha);
                trueque.hora = Some(query.hora);
                trueque.minutos = Some(query.minutos);
                trueque.sucursal = Some(query.sucursal);
                trueque.codigo_receptor = Some(codigos.0);
                trueque.codigo_ofertante = Some(codigos.1);
                //obtengo receptor
                let receptor = self.usuarios.iter().find(|usuario| usuario.dni == trueque.receptor.0).unwrap();
                //obtengo ofertante
                let ofertante = self.usuarios.iter().find(|usuario| usuario.dni == trueque.oferta.0).unwrap();
                //creo mail receptor
                let mail_receptor = format!("Hola {}!\nUsted ha definido un Trueque para la fecha {}, en el horario {}:{}, junto al usuario {}, con DNI {}. Su codigo de receptor para presentar al momento del intercambio es: {}. Por favor, no lo extravíe.\n Si cree que esto es un error, por favor contacte a un administrador.", 
                                receptor.nombre_y_apellido, trueque.fecha_pactada.unwrap().format("%Y-%m-%d").to_string(), trueque.clone().hora.unwrap(), 
                                trueque.clone().minutos.unwrap(), ofertante.nombre_y_apellido, ofertante.dni, trueque.codigo_receptor.unwrap());
                
                //creo mail ofertante
                let mail_ofertante = format!("Hola {}!\nUsted ha definido un Trueque para la fecha {}, en el horario {}:{}, junto al usuario {}, con DNI {}. Su codigo de ofertante para presentar al momento del intercambio es: {}. Por favor, no lo extravíe.\n Si cree que esto es un error, por favor contacte a un administrador.", 
                                ofertante.nombre_y_apellido, trueque.fecha_pactada.unwrap().format("%Y-%m-%d").to_string(), trueque.clone().hora.unwrap(), 
                                trueque.clone().minutos.unwrap(), receptor.nombre_y_apellido, receptor.dni, trueque.codigo_ofertante.unwrap());
                
                //Creo un vec para pasarlo al main y enviarlo
                /* Contenido del Vec:
                0 --> Nombre Receptor
                1 --> Mail Receptor
                2 --> Mensaje Receptor
                3 --> Nombre Ofertante
                4 --> Mail Ofertante
                5 --> Mensaje Ofertante
                 */
                let mut contenidos_mensajes = Vec::new();
                contenidos_mensajes.push(receptor.nombre_y_apellido.clone());
                contenidos_mensajes.push(receptor.email.clone());
                contenidos_mensajes.push(mail_receptor.clone());
                contenidos_mensajes.push(ofertante.nombre_y_apellido.clone());
                contenidos_mensajes.push(ofertante.email.clone());
                contenidos_mensajes.push(mail_ofertante.clone());
                let mensajes = Some(contenidos_mensajes);

                for publicacion in trueque.get_publicaciones() {
                    self.resolver_estado_trueques_de_publicacion(&publicacion);
                }
                self.guardar();
                return (true, mensajes);
            }
        //}
        (false, None)
    }

    fn generar_codigos_de_trueque(&self) -> (u64, u64) {
        let mut receptor: u64 = 0;
        let mut ofertante: u64 = 0;

        let mut existe_combinacion = true;
        while existe_combinacion {
            //genero los codigos
            let mut rng = rand::thread_rng();
            receptor = rng.gen_range(1..1001);
            ofertante = rng.gen_range(1..1001);
    
            //verifico que no exista la combinacion
            existe_combinacion = self.trueques.iter().filter(|(_, trueque)| trueque.estado == EstadoTrueque::Definido).any(|(_, trueque)| {
                trueque.codigo_receptor.unwrap() == receptor && trueque.codigo_ofertante.unwrap() == ofertante
            });
        }
    
        (receptor, ofertante)
    }

    pub fn obtener_sucursal (&self, id: usize) -> String {
        let sucursal = self.sucursales.get(id).unwrap().clone();
        sucursal.nombre
    }
    pub fn alternar_pausa_publicacion (&mut self, id : &usize) -> bool{
        let trueques = &self.trueques;
        let publicacion = self.publicaciones.get_mut(id).unwrap();
        if publicacion.pausada {
            if !Database::hay_trueques_pendientes_definidos_finalizados(publicacion.ofertas.clone(), trueques) {
                publicacion.alternar_pausa();
                self.guardar();
                return true;
            }
        }
        else {
            publicacion.alternar_pausa();
            self.guardar();
            return true;
        }
        false
    }
        
    fn hay_trueques_pendientes_definidos_finalizados (trueques_a_verificar: Vec<usize>, trueques: &HashMap<usize, Trueque>) -> bool {
        for id_trueque in trueques_a_verificar {
            if (trueques.get(&id_trueque).unwrap().estado == EstadoTrueque::Definido) || (trueques.get(&id_trueque).unwrap().estado == EstadoTrueque::Pendiente) || (trueques.get(&id_trueque).unwrap().estado == EstadoTrueque::Finalizado) {
                return true;
            }
        }
        false
    }
    pub fn obtener_trueque_por_codigos (&self, query: QueryTruequesFiltrados) -> Vec<usize>  {
        let codigo_receptor = query.filtro_codigo_receptor.unwrap();
        let codigo_ofertante = query.filtro_codigo_ofertante.unwrap();
        log::info!("CODIGOS RECIBIDOS: RECEPTOR: {:?}, OFERTANTE: {:?}", codigo_receptor, codigo_ofertante);
        let obtenidos = self.trueques.iter()
                    .filter(|(_, trueque)| trueque.estado == EstadoTrueque::Definido)
                    .filter(|(_, trueque)| {
                        trueque.codigo_ofertante.map(|ofertante| ofertante == codigo_ofertante)
                        .unwrap_or(true)
                    })
                    .filter(|(_, trueque)| {
                        trueque.codigo_receptor.map(|receptor| receptor == codigo_receptor)
                        .unwrap_or(true)
                    })
                    .filter(|(_, trueque)| {
                        query.filtro_sucursal.as_ref().map(|sucursal_filtro| {
                            if let Some(sucursal) = &trueque.sucursal {
                                sucursal_filtro == sucursal
                            }
                            else {
                                true
                            }
                        })
                        .unwrap_or(true)
                    });
        let respuesta = obtenidos.map(|(i, _)| *i).collect();
        log::info!("RESPUESTA: {:?}", respuesta);
        respuesta
    }
    //puede concretarse o rechazarse
    pub fn finalizar_trueque (&mut self, query: QueryFinishTrade) -> Result<Vec<String>, ErrorEnConcretacion>{
        log::info!("Query: {:?}", query);

        //obtnego el trueque
        let trueque = self.trueques.get_mut(&query.id_trueque).unwrap();

        //verifico que este definido. De no ser asi (no debería pasar), salgo
        if trueque.estado != EstadoTrueque::Definido {
            return Ok(vec![]);
        }

        //actualizo la informacion del trueque y obtengo los datos necesarios para laburar
        trueque.estado = query.estado.clone();
        trueque.fecha_trueque = Some(Local::now());
        let mut ventas_ofertante = Some(query.ventas_ofertante);
        let mut ventas_receptor = Some(query.ventas_receptor);
        let dni_receptor = trueque.receptor.0;
        let dni_ofertante = trueque.oferta.0;
        let index_receptor = self.encontrar_dni(dni_receptor).unwrap();
        let index_ofertante = self.encontrar_dni(dni_ofertante).unwrap();
        
        //obtengo el trueque de vuelta por una cuestión de borrowing
        let trueque = self.trueques.get_mut(&query.id_trueque).unwrap();
        
        // Lógica que verifica que el usuario ofertante pueda aplicar el descuento
        if let Some(codigo_descuento) = query.codigo_descuento_ofertante{
            if let Some(descuento) = self.descuentos.iter().find(|d| d.codigo.trim() == codigo_descuento.trim()) {
                if descuento.vigente && descuento.alcanza_nivel(self.usuarios[index_ofertante].puntos) && !descuento.esta_vencido() {
                    let index_descuento_ingresado = self.descuentos.iter().position(|d| d.codigo == codigo_descuento);
                    if !(self.usuarios[index_ofertante].descuentos_utilizados.contains(&index_descuento_ingresado.unwrap())){
                        ventas_ofertante = Some(descuento.aplicar_descuento(ventas_ofertante.unwrap()));
                    } else {
                        return Err(ErrorEnConcretacion::DescuentoOfertanteUtilizado)
                    }
                } else {
                    return Err(ErrorEnConcretacion::DescuentoOfertanteInvalido)
                }
            } else {
                return Err(ErrorEnConcretacion::DescuentoOfertanteInvalido)
            }
        }
        
        // Lógica que verifica que el usuario receptor pueda aplicar el descuento
        if let Some(codigo_descuento) = query.codigo_descuento_receptor {
            if let Some(descuento) = self.descuentos.iter().find(|d| d.codigo == codigo_descuento) {
                if descuento.vigente && descuento.alcanza_nivel(self.usuarios[index_receptor].puntos) && !descuento.esta_vencido() {
                    let index_descuento_ingresado = self.descuentos.iter().position(|d| d.codigo == codigo_descuento);
                    if !(self.usuarios[index_receptor].descuentos_utilizados.contains(&index_descuento_ingresado.unwrap())){
                        ventas_receptor = Some(descuento.aplicar_descuento(ventas_receptor.unwrap()));
                    } else {
                        return Err(ErrorEnConcretacion::DescuentoReceptorUtilizado)
                    }
                } else {
                    return Err(ErrorEnConcretacion::DescuentoReceptorInvalido)
                }
            } else {
                return Err(ErrorEnConcretacion::DescuentoReceptorInvalido)
            }
        }

        //aplico las ventas a los usuarios
        trueque.ventas_ofertante = ventas_ofertante;
        trueque.ventas_receptor = ventas_receptor;
        
        // Obtengo el trueque de vuelta por una cuestion de borrowing
        //let trueque = self.trueques.get(&query.id_trueque).unwrap();

        //si el estado es "Finalizado", es decir, se concretó, aumento los puntos a los usuarios
        //de lo contrario, habilito a que se puedan realizar trueques con las publicaciones
        if query.estado == EstadoTrueque::Finalizado {
            self.usuarios[index_ofertante].puntos += 1;
            self.usuarios[index_receptor].puntos += 1;
            for id_publicacion in trueque.get_publicaciones() {
                self.publicaciones.get_mut(&id_publicacion).unwrap().intercambiada = true;
            }
        }
        else {
            //cambio el booleano "en_trueque" y "pausada" de cada publicacion
            for id_publicacion in trueque.get_publicaciones() {
                self.publicaciones.get_mut(&id_publicacion).unwrap().en_trueque = false;
                self.publicaciones.get_mut(&id_publicacion).unwrap().pausada= false;
            }
        }

        //guardo los cambios
        self.guardar();

        let mail_receptor; 
        let mail_ofertante;
        if query.estado == EstadoTrueque::Finalizado {
            //creo mail receptor
            mail_receptor = format!("Hola {}!\nUsted ha concretado un Trueque, junto al usuario {}, con DNI {}. \n Si cree que esto es un error, por favor contacte a un administrador.", 
            self.usuarios[index_receptor].nombre_y_apellido, self.usuarios[index_ofertante].nombre_y_apellido, self.usuarios[index_ofertante].dni);
            
            //creo mail ofertante
            mail_ofertante = format!("Hola {}!\nUsted ha concretado un Trueque, junto al usuario {}, con DNI {}. \n Si cree que esto es un error, por favor contacte a un administrador.", 
            self.usuarios[index_ofertante].nombre_y_apellido, self.usuarios[index_receptor].nombre_y_apellido, self.usuarios[index_receptor].dni);
        }
        else {
            //creo mail receptor
            mail_receptor = format!("Hola {}!\nUsted ha rechazado un Trueque, junto al usuario {}, con DNI {}. \n Si cree que esto es un error, por favor contacte a un administrador.", 
            self.usuarios[index_receptor].nombre_y_apellido, self.usuarios[index_ofertante].nombre_y_apellido, self.usuarios[index_ofertante].dni);
            
            //creo mail ofertante
            mail_ofertante = format!("Hola {}!\nUsted ha rechazado un Trueque, junto al usuario {}, con DNI {}. \n Si cree que esto es un error, por favor contacte a un administrador.", 
            self.usuarios[index_ofertante].nombre_y_apellido, self.usuarios[index_receptor].nombre_y_apellido, self.usuarios[index_receptor].dni);
        }
        
        // - Enviar notificaciones (puede incluir una opcion para calificar al usuario)

        //Creo un vec para pasarlo al main y enviarlo
        /* Contenido del Vec:
        0 --> Nombre Receptor
        1 --> Mail Receptor
        2 --> Mensaje Receptor
        3 --> Nombre Ofertante
        4 --> Mail Ofertante
        5 --> Mensaje Ofertante
            */

        log::info!("llegue hasta aca abajo");
        let mut contenidos_mensajes = Vec::new();
        contenidos_mensajes.push(self.usuarios[index_receptor].nombre_y_apellido.clone());
        contenidos_mensajes.push(self.usuarios[index_receptor].email.clone());
        contenidos_mensajes.push(mail_receptor.clone());
        contenidos_mensajes.push(self.usuarios[index_ofertante].nombre_y_apellido.clone());
        contenidos_mensajes.push(self.usuarios[index_ofertante].email.clone());
        contenidos_mensajes.push(mail_ofertante.clone());
        Ok(contenidos_mensajes)
    }

    pub fn preguntar(&mut self, query:QueryAskQuestion){
        let publicacion = self.publicaciones.get_mut(&query.id_publicacion);
        if let Some(publicacion) = publicacion{
            let pregunta = PregYRta {dni_preguntante : query.dni_preguntante, pregunta:query.pregunta, respuesta:None};
            publicacion.preguntas.push(pregunta);
            self.guardar();
        }else{
            log::error!("error al buscar la publicacion (no deberia pasar)");
        }
        self.guardar();
    }



    pub fn responder(&mut self, query:QueryAnswerQuestion){
        let publicacion = self.publicaciones.get_mut(&query.id_publicacion);
        if let Some(publicacion) = publicacion{
            if let Some(pregunta)=publicacion.preguntas.get_mut(query.indice_pregunta){
                pregunta.respuesta = Some(query.respuesta);
            }
        }
        self.guardar();
    }
    
    pub fn obtener_preferencias(&self, dni: u64) -> (Option<String>, Option<String>){
        self.usuarios.iter()
        .find(|u| u.dni == dni)
        .unwrap()
        .preferencias
        .clone()
    }

    pub fn actualizar_preferencias(&mut self, dni: u64, preferencias: (Option<String>, Option<String>)) {
        let mut nuevas_preferencias = (None, None);
        let usuario = self.usuarios.iter_mut()
        .find(|u| u.dni == dni)
        .unwrap();

        if let Some(preferencia_a) = preferencias.0 {
            if preferencia_a.is_empty() {
                nuevas_preferencias.0 = usuario.preferencias.0.clone();
            } else {
                nuevas_preferencias.0 = Some(preferencia_a)
            }
        } 

        if let Some(preferencia_b) = preferencias.1 {
            if preferencia_b.is_empty() {
                nuevas_preferencias.1 = usuario.preferencias.1.clone();
            } else {
                nuevas_preferencias.1 = Some(preferencia_b)
            }
        } 

        usuario.preferencias = nuevas_preferencias;
        self.guardar()
    }

    pub fn guardar_publicacion(&mut self, query:QueryAgregarAGuardados){
        let index = self.encontrar_dni(query.dni).unwrap();
        let usuario = self.usuarios.get_mut(index).unwrap();
        let publicacion = self.publicaciones.get(&query.id_publicacion);
        //si la publicacion existe entonces guardo el id en el vec de guardados
        if let Some(publicacion) = publicacion{
            usuario.publicaciones_guardadas.push(query.id_publicacion);
        } else{
            log::error!("hubo un error encontrando a la publicacion");
        }
        self.guardar();
    }

    pub fn eliminar_publicacion_guardadas(&mut self, query:QueryEliminarGuardados){
        let index = self.encontrar_dni(query.dni).unwrap();
        let usuario = self.usuarios.get_mut(index).unwrap();
        let publicacion = self.publicaciones.iter_mut().find(|p| p.0 == &query.id_publicacion);
        if let Some(publicacion) = publicacion{
            //retain retiene en el vector todos los elementos que cumplan con la clausula
            usuario.publicaciones_guardadas.retain(|p| p != publicacion.0);
        }
        else{
            log::error!("hubo un error encontrando a la publicacion");
        }
        self.guardar();
    }

    pub fn publicacion_guardada(&self, query:QueryPublicacionGuardada)-> bool{
        let index = self.encontrar_dni(query.dni).unwrap();
        let usuario = self.usuarios.get(index).unwrap();
        let publicacion_buscada = self.publicaciones.get(&query.id_publicacion);
        if let Some(publicacion_buscada) = publicacion_buscada{
            if usuario.publicaciones_guardadas.contains(&query.id_publicacion){
                return true
            }
        }
        false
    }

    pub fn obtener_publicaciones_guardadas(&self, query:QueryObtenerGuardadas)->Vec<usize>{
        let index = self.encontrar_dni(query.dni).unwrap();
        let usuario = self.usuarios.get(index).unwrap().clone();
        usuario.publicaciones_guardadas
    }

    pub fn generar_mail_recuperacion_contrasenia(&mut self, query: QuerySendChangePasswordCode) -> Vec<String> {
        //busco la posicion del usuario en el vector de existir
        let option_usuario = self.usuarios.iter().position(|usuario| usuario.email == query.email);
        if let Some(id_usuario) = option_usuario {
            //lo obtengo para obtener sus datos
            let usuario = self.usuarios.get(id_usuario).unwrap();
            let codigo_seguridad = Database::generar_codigo_cambio_contraseña(query.email.clone(), &self.peticiones_cambio_contraseña);
            let peticion = PeticionCambioContrasenia {codigo_seguridad, email: query.email.clone(), usada: false};
            self.peticiones_cambio_contraseña.push(peticion);
            self.guardar();
            //Creo un vec para pasarlo al main y enviarlo
            /* Contenido del Vec:
            0 --> Nombre 
            1 --> Mail 
            2 --> Mensaje 
            */
            let mensaje = format!("Hola {}!\nUsted ha solicitado un cambio de contraseña en la pagina Fedeteria. El código de seguridad para realizar el cambio de contraseña es {}. Dirijase a la sección de Inicio de Sesión, y presione 'Cambiar Contraseña'. Allí encontrará la guía para cambiar su contraseña. \n Si cree que esto es un error, por favor contacte a un administrador.", usuario.nombre_y_apellido, codigo_seguridad);
            let mut vec_mensajes = Vec::new();
            vec_mensajes.push(usuario.nombre_y_apellido.clone());
            vec_mensajes.push(query.email.clone());
            vec_mensajes.push(mensaje);
            return vec_mensajes;
        }
        return Vec::new();
    }

    fn generar_codigo_cambio_contraseña(email: String, peticiones: &Vec<PeticionCambioContrasenia>) -> u64 {
        let mut codigo: u64 = 0;

        let mut existe_combinacion = true;
        while existe_combinacion {
            //genero el codigo
            let mut rng = rand::thread_rng();
            codigo = rng.gen_range(1..1001);
    
            //verifico que no exista la combinacion
            existe_combinacion = peticiones.iter().filter(|peticion| !peticion.usada).any(|peticion| (peticion.codigo_seguridad == codigo) && (peticion.email == email));
        }
    
        codigo
    }

    pub fn validar_cambio_contrasenia (&self, query: QueryValidarCambioContrasenia) -> bool {
        self.peticiones_cambio_contraseña.iter()
                        .filter(|peticion| !peticion.usada)
                        .any(|peticion| (peticion.codigo_seguridad == query.codigo) && (peticion.email == query.email))
    }

    pub fn cambiar_contrasenia_login (&mut self, query: QueryCambioContraseniaLogIn) -> bool {
        //obtengo al usuario
        let option_usuario = self.usuarios.iter().position(|usuario| usuario.email == query.email);
        if let Some(id_usuario) = option_usuario {

            //hasheo la nueva contraseña para compararla y cambiarla si no es la misma
            let nueva_contrasenia_hash = hash_str(&query.nueva_contrasenia);
            if self.usuarios[id_usuario].contraseña != nueva_contrasenia_hash {

                //cambio la contraseña
                self.usuarios[id_usuario].contraseña = nueva_contrasenia_hash;

                //obtengo la peticion y la marco como usada
                let posicion_peticion = self.peticiones_cambio_contraseña.iter().position(|peticion| (peticion.email == query.email) && (peticion.codigo_seguridad == query.codigo));
                log::info!("POSICION PETICION: {:?}", posicion_peticion);
                self.peticiones_cambio_contraseña[posicion_peticion.unwrap()].usada = true;

                self.guardar();
                return true;
            }
        }
        false
    }

    pub fn cambiar_contrasenia_perfil (&mut self, query: QueryCambioContraseniaPerfil) -> bool {
        //obtengo el id del usuario
        let option_usuario = self.encontrar_dni(query.dni);
        if let Some(id_usuario) = option_usuario {
            //hasheo las contraseñas para compararlas y cambiarlas si se cumplen las condiciones
            let nueva_contrasenia_hash = hash_str(&query.nueva_contrasenia);
            let vieja_contrasenia_hash = hash_str(&query.vieja_contrasenia);
            if (self.usuarios[id_usuario].contraseña == vieja_contrasenia_hash) && (self.usuarios[id_usuario].contraseña != nueva_contrasenia_hash) {
                //cambio la contraseña
                self.usuarios[id_usuario].contraseña = nueva_contrasenia_hash;
                self.guardar();
                return true;
            }
        }
        false
    }

    pub fn crear_descuento(&mut self, query:QueryCreateDiscount)->Result<bool,ErrorCrearDescuento>{
        if let Some (fecha) = query.fecha_exp{
            //chequear si la fecha está despues
            //chequear que no haya dos codigos iguales
            if query.porcentaje > 1.0 && query.porcentaje < 0.0{
                return Err(ErrorCrearDescuento::PorcentajeInvalido);
            }
            let nuevo_descuento = Descuento{
                fecha_vencimiento : fecha,
                porcentaje : query.porcentaje,
                reintegro_maximo : query.reembolso_max,
                nivel_minimo : query.nivel_min,
                codigo : query.codigo_descuento,
                vigente : true,
            };
            self.descuentos.push(nuevo_descuento);
        }
        self.guardar();

        Ok(true)
    }

    pub fn obtener_descuentos(&self) -> Vec<Descuento>{
        self.descuentos.clone()
    }

    pub fn eliminar_descuento(&mut self, query:QueryEliminarDescuento) -> bool{
        let descuento = self.descuentos.get_mut(query.index);
        if let Some(descuento) = descuento{
            descuento.vigente = false;
        }
        self.descuentos.sort_by_key(|d| !d.vigente);
        self.guardar();
        true
    }

    pub fn obtener_descuentos_usuario(&self, query:QueryGetUserDiscounts)-> Vec<Descuento>{
        let index = self.encontrar_dni(query.dni);
        let usuario = self.usuarios.get(index.unwrap()).unwrap();
        let descuentos: Vec<Descuento> = self.descuentos.iter()
        .filter(|d| d.nivel_minimo <= (usuario.puntos / 5) as u64)
        .cloned()
        .collect();
        descuentos
    }

    /*
    pub fn enviar_dinero(amount:u64){
        let access_token = std::env::var("TEST-6367565001372366-070612-1af9f8ba91b75e6d7ff8e4cc68c0c4d9-421443948").expect("MERCADOPAGO_ACCESS_TOKEN debe estar configurado");
        let mp_client = MercadoPagoClientBuilder::builder(&access_token).build();
        let float_value: f64 = 200.0;
        let decimal_value = Decimal::from_f64(float_value).expect("Error al convertir a Decimal");
        mpago::payments::PaymentCreateBuilder(PaymentCreateOptions {
            transaction_amount: decimal_value,
            date_of_expiration: Some(date_of_expiration),
            ..Default::default()
        })
        .send(&mp_client)
        .await?;
    
}
*/


//el ofertante califica al receptor
pub fn calificar_ofertante(&mut self, query:QueryCalificarOfertante)-> bool{
    let trueque = self.trueques.get_mut(&query.id_trueque).unwrap();
    if let Some(calificacion) = query.calificacion{
        if calificacion > 10{
            return false;
        }
        trueque.calificacion_receptor = Some(calificacion);
    }
    self.guardar();
    true
}

//el receptor califica al ofertante
pub fn calificar_receptor(&mut self, query:QueryCalificarReceptor)-> bool{
    let trueque = self.trueques.get_mut(&query.id_trueque).unwrap();
    if let Some(calificacion) = query.calificacion{
        if calificacion > 10{
            return false;
        }
        trueque.calificacion_ofertante = Some(calificacion);
    }
    self.guardar();
    true
}

    pub fn calcular_promedio_calificaciones (&self, dni: u64) -> f64 {
        //obtengo las calificaciones del usuario (los trueques en los que no fue calificado, no se tienen en cuenta)
        let calificaciones: Vec<u64> = self.trueques.iter()
                            .filter(|(_, trueque)| trueque.usuario_participa(dni) && trueque.usuario_tiene_calificacion(dni))
                            .map(|(_, trueque)| trueque.get_calificacion(dni).unwrap())
                            .collect();

        //si no hay calificaciones, salgo
        if calificaciones.len() == 0 {
            return 0.0;
        }

        //sumo las calificaciones y las divido por la cantidad que son
        let calificiones_sumadas: u64 = calificaciones.iter().sum();
        let promedio_calificaciones: f64 = (calificiones_sumadas / calificaciones.len() as u64) as f64;

        return promedio_calificaciones;
    }

}

fn get_database_por_defecto() -> Database {
    use RolDeUsuario::*;
    let mut db: Database = Default::default();
    let sucursales = [
        "La Plata 1 y 50",
        "La Plata 3 y 33",
    ];
    // (nombre, dni, rol). la contraseña es igual al dni. el email se genera en base al nombre
    let usuarios = [
        ("Alan", 1, Dueño),
        ("Bauti", 2, Empleado { sucursal: 0 }),
        ("Carlos", 3, Empleado { sucursal: 1 } ),
        ("Delfina", 4, Normal),
        ("Esteban", 5, Normal),
    ];

    // (dni del dueño, nombre, descripcion, Option<precio>, vec![fotos])
    let publicaciones = [
        (3, "Sierra grande", "Mi linda sierra", Some(9_000_000), vec!["sierra.jpg"]),
        (5, "Heladera", "Se me quemó", Some(600), vec!["heladera quemada.jpg"]),
        (5, "Mouse", "Un mouse. Anda bien", None, vec!["mouse.jpg"]),
        (5, "Teclado", "Teclado tikitiki", Some(650), vec!["teclado.jpg"]),
        (5, "Curita", "Curita para sanar :)", Some(800), vec!["curita.jpg"]),
        (5, "Tenedor", "Tenedor. lo usé para comer milanesa.", Some(299), vec!["tenedor.jpg"]),
        (5, "Cuchara", "No es comestible", Some(300), vec!["cuchara.jpg"]),
        (5, "Martillo", "Un martillo normal. Ya no lo uso.", Some(1500), vec!["martillo.jpg", "martillin2.jpg"]),
        (4, "Tornillo", "Un tornillo sin usar jeje", Some(400), vec!["tornillo.jpg"]),
        (4, "Avena Danesa", "Riquísima avena que traje de Dinamarca. Es medio agresiva.", Some(900), vec!["solgryn.png"]),
        (4, "Destornillador", "Destornillador que podes usar para destornillar o bien para atornillar", Some(300), vec!["destornillador.jpg"]),
        (4, "Papel", "Papel SIN ESCRIBIR", Some(630), vec!["papel.jpg"]),
        (4, "Mancha", "Una mancha porfavor saquenla de mi piso", Some(370), vec!["mancha.jpg"]),
        (4, "Esponja", "Limpien no sean vagos dale", Some(230), vec!["esponja.jpg"]),
        (4, "Reloj", "Un reloj les juro que se mueve", Some(900), vec!["reloj.jpg"]),
        (4, "Hamaca", "Wiiiii", Some(1300), vec!["hamaca.jpg"]),
        (4, "Casa", "Perro y coche no incluidos. El pibe sí.", Some(6_000_000), vec!["casa.jpg"]),
    ];
    
    for sucursal in sucursales {
        db.agregar_sucursal(QueryAddOffice { office_to_add: sucursal.to_string() });
    }

    for (i, datos) in usuarios.into_iter().enumerate() {
        let nombre_y_apellido = datos.0.to_string();
        let dni = datos.1;
        let email = nombre_y_apellido.clone() + "@gmail.com";
        let contraseña = dni.to_string();
        let nacimiento = Local.with_ymd_and_hms(2000, 1, 1, 1, 1, 1).unwrap();
        db.agregar_usuario(QueryRegistrarUsuario { nombre_y_apellido, dni, email, contraseña, nacimiento }).unwrap();
        let rol = datos.2;
        db.cambiar_rol_usuario(QueryChangeUserRole { dni, new_role: rol });
        assert_eq!(db.encontrar_dni(dni).unwrap(), i);
    }

    for (dni_usuario, titulo, descripcion, precio, fotos) in publicaciones {
        let imagenes = fotos.iter().map(|nombre| {
            let from = format!("fotos_database_default/{}", nombre);
            // TODO: Que realmente se guarde en carpetas xd
            let relativo = format!("{}", nombre);
            let to = format!("db/imgs/{}", relativo);
            println!("from {from} to {to}");
            std::fs::copy(from, to).unwrap();
            relativo
        }).collect();
        db.agregar_publicacion(Publicacion {
            dni_usuario,
            titulo: titulo.to_string(),
            descripcion: descripcion.to_string(),
            imagenes,
            precio,
            pausada: precio.is_none(),
            en_trueque:false,
            eliminada: false,
            intercambiada: false,
            ofertas: vec![],
            preguntas: vec![],
            promocionada_hasta: None,
        });
    }

    db
}

