use std::ops::{Range, RangeInclusive};

use chrono::{Date, DateTime, Local};
use serde::{Deserialize, Serialize};

use crate::{QueryAddOffice, Trueque};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Publicacion {
    pub dni_usuario: u64,
    pub titulo: String,
    pub descripcion: String,
    // Las imagenes son relativas (osea que hay que agregar /db/imgs o publication_images/ dependiendo del caso)
    pub imagenes: Vec<String>,
    pub precio: Option<u64>,
    pub pausada: bool,
    pub ofertas: Vec<Trueque>,
}

impl Publicacion {
    pub fn new(titulo: String, descripcion: String, imagenes: Vec<String>, dni_usuario: u64) -> Self {
        Publicacion { 
            titulo,
            descripcion, 
            imagenes,
            dni_usuario,
            precio: None,
            pausada: true,
            ofertas: Vec::new(),
        }
    }

    pub fn alternar_pausa(&mut self){
        self.pausada = !(self.pausada);
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct QueryPublicacion {
    pub id: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum ErrorPublicacion {
    ErrorIndeterminado,
    PublicacionInexistente,
}



pub type ResponsePublicacion = Result<Publicacion, ErrorPublicacion>;


#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct QueryPublicacionesFiltradas {
    pub filtro_dni: Option<u64>,
    pub filtro_nombre: Option<String>,
    pub filtro_precio_min: Option<u64>,
    pub filtro_precio_max: Option<u64>,
    // FALTA HACER: filtro por fecha
    pub filtro_fecha_min: Option<()>,
    pub filtro_fecha_max: Option<()>,
    pub filtro_pausadas:bool,
}


pub type ResponsePublicacionesFiltradas = Vec<usize>;

pub fn calcular_rango(precio : u64) -> RangeInclusive<u64> {
    match precio{
        0..=1000 =>  {0..=1000},
        1001..=2500 =>  {1001..=2500},
        2501..=5000 =>  {2501..=5000},
        5001..=7500 =>  {5001..=7500},
        7501..=10000 =>  {7501..=10000},
        10001..=20000 =>  {10001..=20000},
        20001..=40000 =>  {20001..=40000},
        40001..=70000 =>  {40001..=7000},
        70001..=100000 =>  {70001..=100000},
        100001..=u64::MAX =>  {100001..=u64::MAX},
    }
}