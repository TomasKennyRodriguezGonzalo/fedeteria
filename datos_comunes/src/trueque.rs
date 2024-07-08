use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct QueryTruequesFiltrados {
    //indice de una publicacion que compone al trueque
    pub filtro_id_publicacion: Option<usize>,
    //dni ofertante
    //pub filtro_ofertante: Option<u64>,
    //dni receptor
    //pub filtro_receptor: Option<u64>,
    //filtro por dni de ofertante o receptor
    pub filtro_dni_integrantes: Option<u64>,
    pub filtro_estado: Option<EstadoTrueque>,
    pub filtro_codigo_ofertante: Option<u64>,
    pub filtro_codigo_receptor: Option<u64>,
    pub filtro_sucursal: Option<usize>,
    // FALTA HACER: filtro por fecha
    pub filtro_fecha_pactada: Option<()>,
    pub filtro_fecha_trueque: Option<()>,
}

pub type ResponseTruequesFiltrados = Vec<usize>;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Trueque {
    /// oferta.0 --> dni del usuario ofertante.
    /// oferta.1 --> coleccion de indices de publicaciones de oferta. 
    pub oferta: (u64, Vec<usize>),
    /// receptor.0 --> dni del usuario receptor.
    /// receptor.1 --> indice de publicacion de receptor.
    pub receptor: (u64, usize),
    pub sucursal: Option<usize>,
    pub fecha_pactada: Option<DateTime<Local>>,
    pub fecha_trueque: Option<DateTime<Local>>,
    pub hora: Option<String>,
    pub minutos: Option<String>,
    pub estado: EstadoTrueque,
    pub codigo_ofertante: Option<u64>,
    pub codigo_receptor: Option<u64>,
    //pub ganancias: u64,
    // Para el front end...
    pub valido: bool,
    // Ventas en bruto, Option<ventas tras aplicar descuento>
    pub ventas_ofertante:Option<(u64, Option<u64>)>,
    pub ventas_receptor:Option<(u64, Option<u64>)>,
    pub calificacion_ofertante:Option<u64>,
    pub calificacion_receptor:Option<u64>,
}

impl Trueque {
    pub fn aceptar (&mut self) {
        //verifico que no este finalizada para que por algun error inesperado, no se vuelva a un estado previo
        if self.estado != EstadoTrueque::Finalizado {
            self.estado = EstadoTrueque::Pendiente;
            self.fecha_pactada = None;
            self.fecha_trueque = None;
            self.hora = None;
            self.minutos = None;
            self.sucursal = None;
        }
    }

    pub fn definir (&mut self, 
                        fecha_pactada: DateTime<Local>, 
                        hora: String,
                        minutos: String,
                        sucursal_elegida: usize, 
                        codigo_ofertante: u64, 
                        codigo_receptor: u64
                    ) {
         //verifico que no este finalizada para que por algun error inesperado, no se vuelva a un estado previo
        if self.estado != EstadoTrueque::Finalizado {
            self.sucursal = Some(sucursal_elegida);
            self.fecha_pactada = Some(fecha_pactada);
            self.hora = Some(hora);
            self.minutos = Some(minutos);
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

    pub fn get_publicaciones(&self) -> Vec<usize> {
        let mut pubs = self.oferta.1.clone();
        pubs.push(self.receptor.1);
        pubs
    }

    pub fn usuario_participa (&self, dni: u64) -> bool {
        if self.oferta.0 == dni || self.receptor.0 == dni {
            return true;
        }
        false
    }

    pub fn get_calificacion (&self, dni: u64) -> Option<u64> {
        if self.oferta.0 == dni {
            return self.calificacion_ofertante;
        }
        else if self.receptor.0 == dni {
            return self.calificacion_receptor;
        }
        None
    }

    pub fn usuario_tiene_calificacion (&self, dni: u64) -> bool {
        if self.oferta.0 == dni {
            return self.calificacion_ofertante.is_some();
        }
        else if self.receptor.0 == dni {
            return self.calificacion_receptor.is_some();
        }
        false
    }
}

#[derive(Debug, Deserialize, Serialize, Clone ,PartialEq)]
pub enum EstadoTrueque {
    Cancelado,
    Oferta,
    Pendiente,
    Definido,
    Finalizado,
    Rechazado
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