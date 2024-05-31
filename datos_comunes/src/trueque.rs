use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Trueque {
    //oferta.0 --> indice del usuario en el vec de usuarios
    //oferta.1 --> coleccion de indices de publicaciones de oferta 
    pub oferta: (u64, Vec<usize>),
    //receptor.0 --> indice del usuario en el vec de usuarios
    //receptor.1 --> indice de publicacion de receptor 
    pub receptor: (u64, usize),
    pub sucursal: Option<String>,
    pub horario: Option<DateTime<Local>>,
    pub estado: EstadoTrueque,
    pub codigo_ofertante: Option<u64>,
    pub codigo_receptor: Option<u64>,
}

impl Trueque {
    pub fn aceptar (&mut self) {
        //verifico que no este finalizada para que por algun error inesperado, no se vuelva a un estado previo
        if self.estado != EstadoTrueque::Finalizado {
            self.estado = EstadoTrueque::Pendiente;
            self.horario = None;
            self.sucursal = None;
        }
    }

    pub fn definir (&mut self, 
                        horario: DateTime<Local>, 
                        sucursal_elegida: String, 
                        codigo_ofertante: u64, 
                        codigo_receptor: u64
                    ) {
         //verifico que no este finalizada para que por algun error inesperado, no se vuelva a un estado previo
        if self.estado != EstadoTrueque::Finalizado {
            self.sucursal = Some(sucursal_elegida);
            self.horario = Some(horario);
            self.estado = EstadoTrueque::Definido;
            self.codigo_ofertante = Some(codigo_ofertante);
            self.codigo_receptor = Some(codigo_receptor);
        }
    }

    pub fn finalizar (&mut self) {
        self.estado = EstadoTrueque::Finalizado
    }

    pub fn verificar_codigos (&self, codigo_ofertante: u64, codigo_receptor: u64) -> bool{
        (self.codigo_ofertante == Some(codigo_ofertante)) && (self.codigo_receptor == Some(codigo_receptor))
    }
}

#[derive(Debug, Deserialize, Serialize, Clone ,PartialEq)]
pub enum EstadoTrueque {
    Oferta,
    Pendiente,
    Definido,
    Finalizado,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct QueryObtenerTrueque {
    pub id: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum ErrorObtenerTrueque {
    ErrorIndeterminado,
    TruequeInexistente,
}



pub type ResponseObtenerTrueque = Result<Trueque, ErrorObtenerTrueque>;