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
    pub fn aplicar_descuento(&self, dinero:u64)->u64{
        let dinero_porcentaje = dinero as f64 * self.porcentaje;
        if dinero_porcentaje >= self.reintegro_maximo as f64{
            let dinero_descontado = dinero-self.reintegro_maximo;
            return dinero_descontado;
        }
        dinero - ((dinero as f64) * self.porcentaje).round() as u64
    }

    pub fn alcanza_nivel(&self, cant_trueques:i64)->bool{
        let nivel = (cant_trueques as f32 / 5.0) as u64;
        if nivel >= self.nivel_minimo{
            return true;
        }
        false
    }

    pub fn esta_vencido(&self) -> bool {
        return Local::now() >= self.fecha_vencimiento 
    }
}

