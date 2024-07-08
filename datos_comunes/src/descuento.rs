use chrono::prelude::*;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct Descuento{
    pub fecha_vencimiento: DateTime<Local>,
    pub porcentaje:f64,
    pub reintegro_maximo:u64,
    pub nivel_minimo:u64,
    pub codigo:String,
    pub vigente:bool,
}

impl Descuento{
    pub fn aplicar_descuento(&self, dinero: u64) -> u64{
        assert!(self.porcentaje > 0.0 && self.porcentaje <= 1.0);
        let dinero_porcentaje = (dinero as f64 * self.porcentaje).ceil() as u64;
        let descuento = dinero_porcentaje.min(self.reintegro_maximo);
        dinero - descuento
    }

    pub fn alcanza_nivel(&self, cant_trueques: u64) -> bool{
        let nivel = cant_trueques / 5;
        if nivel >= self.nivel_minimo {
            return true;
        }
        false
    }

    pub fn esta_vencido(&self) -> bool {
        Local::now() > self.fecha_vencimiento 
    }
}

