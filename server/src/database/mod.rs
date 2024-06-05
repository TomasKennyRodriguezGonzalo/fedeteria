use std::{collections::HashMap, fs, path::Path};

use chrono::{DateTime, Local, TimeZone};
use date_component::date_component;
use datos_comunes::*;
use serde::{Deserialize, Serialize};
use tokio::runtime::TryCurrentError;
use tower_http::follow_redirect::policy::PolicyExt;

use self::usuario::{EstadoCuenta, Usuario};

pub mod usuario;

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Database {

    usuarios: Vec<Usuario>,
    sucursales: Vec<Sucursal>,

    publicaciones_auto_incremental: usize,
    publicaciones: HashMap<usize, Publicacion>,

    trueques_auto_incremental: usize,
    trueques: HashMap<usize, Trueque>,
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

    pub fn crear_oferta(&mut self, query:QueryCrearOferta) -> bool {
        if let Some(indice) = self.encontrar_dni(query.dni_receptor) {
            // Crea el Trueque en estado de Oferta
            let oferta = Trueque{
                oferta: (query.dni_ofertante, query.publicaciones_ofertadas),
                receptor: (query.dni_receptor, query.publicacion_receptora),
                sucursal: None,
                fecha: None,
                hora: None,
                minutos: None,
                estado: EstadoTrueque::Oferta,
                codigo_ofertante: None,
                codigo_receptor: None,
            };
            let index = self.agregar_trueque(oferta);

            // Si la publicación existe, que se asume que si, se agrega la referencia al trueque
            if let Some(publicacion) = self.publicaciones.get_mut(&query.publicacion_receptora){
                publicacion.ofertas.push(index);
                self.guardar();
                return true
            }
            
        }
        return false
    }

    /* .filter(|(_, publication)| {
            query.filtro_nombre.as_ref()
            .map(|nombre| publication.titulo.to_lowercase().contains(&nombre.to_lowercase()))
            .unwrap_or(true) */

    /*pub fn obtener_trueques (&self, query: QueryObtenerTrueques) -> Vec<usize> {
        let obtenidos = self.trueques.iter().
                        enumerate().
                        filter(|(_, trueque)| trueque.1.estado == query.estado).
                        filter(|(_,trueque)|{
                            query.id_publicacion.as_ref()
                            .map(|publicacion| trueque.1.receptor.1 == *publicacion)
                            .unwrap_or(true)
                        }) 
                        .map(|(indice, _)| indice).
                        collect();

        obtenidos
    }*/

    pub fn obtener_trueques (&self, query: QueryTruequesFiltrados) -> Vec<usize> {
        let obtenidos = self.trueques.iter().
                    filter(|(_, trueque)| {
                        query.filtro_estado.as_ref().map(|estado_trueque| estado_trueque == &trueque.estado)
                        .unwrap_or(true)
                    }).
                    filter(|(_,trueque)|{
                        query.filtro_id_publicacion.map(|publicacion| (trueque.receptor.1 == publicacion) || (trueque.oferta.1.contains(&publicacion)))
                        .unwrap_or(true)
                    }).
                    /*filter(|(_, trueque)| {
                        query.filtro_ofertante.map(|dni_ofertante| trueque.oferta.0 == dni_ofertante)
                        .unwrap_or(true)
                    }).
                    filter(|(_, trueque)| {
                        query.filtro_receptor.map(|dni_receptor| trueque.receptor.0 == dni_receptor)
                        .unwrap_or(true)
                    }).*/
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

    pub fn get_trueque (&self, id: usize) -> Option<&Trueque> {
        self.trueques.get(&id)
    }

    pub fn aceptar_oferta(&mut self, id:usize) -> bool {
        let trueque = self.trueques.get_mut(&id);
        if let Some(trueque) = trueque {
            //aca se modificaria la variable de "en trueque"
            let publicacion_receptora = self.publicaciones.get_mut(&trueque.receptor.1);
            publicacion_receptora.unwrap().pausada = true; 
            if let Some(publi1) = trueque.oferta.1.get(0) {
                self.publicaciones.get_mut(publi1).unwrap().alternar_pausa();
            }
            else {
                log::info!("No hay publicacion 1");
            }
            if let Some(publi2) = trueque.oferta.1.get(1) {
                self.publicaciones.get_mut(publi2).unwrap().alternar_pausa();
            }
            else {
                log::info!("No hay publicacion 2");
            }
            trueque.estado = EstadoTrueque::Pendiente;
            self.guardar();
            return true;
        }
        false
    }

    pub fn rechazar_oferta(&mut self, id:usize) -> bool {
        let trueque = self.trueques.get_mut(&id);
        if let Some(_) = trueque {
            self.trueques.remove(&id);
            self.guardar();
            return true;
        }
        false
    }

    pub fn cambiar_trueque_a_definido(&mut self, query:QueryCambiarTruequeADefinido) -> bool {
        let trueques_copia = self.trueques.clone();
        let trueque = self.trueques.get_mut(&query.id);
        if let Some(trueque_actual) = trueque {
            let hay_otros_trueques = trueques_copia.iter().
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
            log::info!("Trueques en misma hora y fecha: {}", hay_iguales.len());
            if hay_iguales.is_empty() {
                trueque_actual.estado = EstadoTrueque::Definido;
                trueque_actual.fecha = Some(query.fecha);
                trueque_actual.hora = Some(query.hora);
                trueque_actual.minutos = Some(query.minutos);
                trueque_actual.sucursal = Some(query.sucursal);
                self.guardar();
                return true;
            }
        }
        false
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
        ("Fede", 1, Dueño),
        ("Lucas", 2, Empleado { sucursal: 0 }),
        ("Matías", 3, Normal),
    ];

    // (dni del dueño, nombre, descripcion, Option<precio>, vec![fotos])
    let publicaciones = [
        (3, "Martillo", "Un martillo normal. Ya no lo uso.", Some(1500), vec!["martillo.jpg", "martillin2.jpg"]),
        (3, "Sierra grande", "Mi linda sierra", Some(9_000_000), vec!["sierra.jpg"]),
        (1, "Heladera", "Se me quemó", Some(600), vec!["heladera quemada.jpg"]),
        (2, "Casa", "Perro y coche no incluidos. El pibe sí.", Some(6_000_000), vec!["casa.jpg"]),
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
            ofertas: vec![],
        });
    }

    db
}