use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Trueque {
    //oferta.0 --> indice del usuario en el vec de usuarios
    //oferta.1 --> coleccion de indices de publicaciones de oferta 
    pub oferta: (u64, Vec<usize>),
    //receptor.0 --> indice del usuario en el vec de usuarios
    //receptor.1 --> coleccion de indices de publicaciones de receptor 
    pub receptor: (u64, Vec<usize>),
    pub sucursal: Option<String>,
    pub horario: Option<DateTime<Local>>,
    pub estado: EstadoTrueque,
    pub hash_ofertante: Option<u64>,
    pub hash_receptor: Option<u64>,
}

impl Trueque {
    pub fn new (    
                dni_ofertante: u64, 
                dni_receptor: u64, 
                id_publicaciones_ofertante: Vec<usize>, 
                id_publicaciones_receptor: Vec<usize>

            ) -> Trueque {
        Trueque {   
                    oferta: ((dni_ofertante, id_publicaciones_ofertante)), 
                    receptor: ((dni_receptor, id_publicaciones_receptor)),
                    sucursal: None,
                    horario: None,
                    estado: EstadoTrueque::Oferta,
                    hash_ofertante: None,
                    hash_receptor: None,
                }
    }

    pub fn aceptar (&mut self) {
        //verifico que no este finalizada para que por algun error inesperado, no se vuelva a un estado previo
        if self.estado != EstadoTrueque::Finalizado {
            self.estado = EstadoTrueque::Pendiente;
            self.horario = None;
            self.sucursal = None;
        }
    }

    pub fn definir (    
                        &mut self, 
                        horario: DateTime<Local>, 
                        sucursal_elegida: String, 
                        hash_ofertante: u64, 
                        hash_receptor: u64
                    ) {
         //verifico que no este finalizada para que por algun error inesperado, no se vuelva a un estado previo
        if self.estado != EstadoTrueque::Finalizado {
            self.sucursal = Some(sucursal_elegida);
            self.horario = Some(horario);
            self.estado = EstadoTrueque::Definido;
            self.hash_ofertante = Some(hash_ofertante);
            self.hash_receptor = Some(hash_receptor);
        }
    }

    pub fn finalizar (&mut self) {
        self.estado = EstadoTrueque::Finalizado
    }

    pub fn verificar_codigos (&self, hash_ofertante: u64, hash_receptor: u64) -> Option<Trueque> {
        let mut ofertante_iguales = false;
        let mut receptor_iguales = false;

        if let Some(ofertante) = self.hash_ofertante {
            ofertante_iguales = ofertante == hash_ofertante;
        }
        if let Some(receptor) = self.hash_receptor {
            receptor_iguales = receptor == hash_receptor;
        }

        if (ofertante_iguales) && (receptor_iguales) {
            Some(self);
        }
        
        None
    }
}

#[derive(Debug, Deserialize, Serialize, Clone ,PartialEq)]
pub enum EstadoTrueque {
    Oferta,
    Pendiente,
    Definido,
    Finalizado,
}