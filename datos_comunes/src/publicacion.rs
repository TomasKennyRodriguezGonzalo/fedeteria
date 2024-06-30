use std::ops::RangeInclusive;

use serde::{Deserialize, Serialize};

//use crate::Trueque;
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct PregYRta {
    pub dni_preguntante: u64,
    pub pregunta:String,
    pub respuesta:Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Publicacion {
    pub dni_usuario: u64,
    pub titulo: String,
    pub descripcion: String,
    // Las imagenes son relativas (osea que hay que agregar /db/imgs o publication_images/ dependiendo del caso)
    pub imagenes: Vec<String>,
    pub precio: Option<u64>,
    pub pausada: bool,
    pub en_trueque: bool,
    //indice de las ofertas/trueques en el vec de la database
    pub ofertas: Vec<usize>,
    pub preguntas:Vec<PregYRta>,
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
            en_trueque: false,
            ofertas: Vec::new(),
            preguntas: Vec::new(),
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


#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
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
        1..=1000 =>  {1..=1000},
        1001..=2500 =>  {1001..=2500},
        2501..=5000 =>  {2501..=5000},
        5001..=7500 =>  {5001..=7500},
        7501..=10000 =>  {7501..=10000},
        10001..=20000 =>  {10001..=20000},
        20001..=40000 =>  {20001..=40000},
        40001..=70000 =>  {40001..=70000},
        70001..=100000 =>  {70001..=100000},
        100001..=u64::MAX =>  {100001..=u64::MAX},
        0 => {0..=0}
    }
}
pub fn get_string_de_rango(precio: u64, incluir_precio: bool) -> String {
    let rango = calcular_rango(precio);
    let (mut min, max) = (*rango.start(), *rango.end());
    if min == 0 {
        min = 1;
    }
    let base = {
        if max == u64::MAX {
            format!("${}+", min)
        } else {
            format!("${}-${}", min, max)
        }
    };
    if incluir_precio {
        format!("{base} (${precio})")
    } else {
        base
    }
    // precio.to_string()
}