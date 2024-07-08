use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Tarjeta {
    pub dni_titular: u64,
    pub nombre_titular: String,
    pub numero_tarjeta: usize,
    pub codigo_seguridad: u64,
    pub anio_caducidad: u64,
    pub mes_caducidad: u64,
    pub monto: i64,
}

impl PartialEq for Tarjeta {
    fn eq(&self, other: &Self) -> bool {
        return
            self.codigo_seguridad == other.codigo_seguridad &&
            self.dni_titular == other.dni_titular &&
            self.anio_caducidad == other.anio_caducidad &&
            self.mes_caducidad == other.mes_caducidad &&
            self.nombre_titular == other.nombre_titular &&
            self.numero_tarjeta == other.numero_tarjeta;
    }
    fn ne(&self, other: &Self) -> bool {
        return
            !(self.codigo_seguridad == other.codigo_seguridad &&
            self.dni_titular == other.dni_titular &&
            self.anio_caducidad == other.anio_caducidad &&
            self.mes_caducidad == other.mes_caducidad &&
            self.nombre_titular == other.nombre_titular &&
            self.numero_tarjeta == other.numero_tarjeta);
    }
}