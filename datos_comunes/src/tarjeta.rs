use chrono::{Local, NaiveDate, TimeZone};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Tarjeta {
    pub dni_titular: u64,
    pub nombre_titular: String,
    pub numero_tarjeta: u64,
    pub codigo_seguridad: u64,
    pub anio_caducidad: u64,
    pub mes_caducidad: u64,
    pub monto: i64,
}

impl Tarjeta {
    pub fn esta_vencida (&self) -> bool {
    // Año y mes
    let anio = self.anio_caducidad as i32;
    let mes = self.mes_caducidad as u32;

    // Día arbitrario (tomo el último dia del mes, 28 por febrero, que se joda si la tiene cerca de vencer)
    let dia: u32 = 28;

    // Creo la fecha
    let naive_date = NaiveDate::from_ymd_opt(anio, mes, dia);
    
    if let Some(fecha) = naive_date {
        // Convertir NaiveDate a LocalDate
        let fecha_vencimiento = Local.from_local_datetime(&fecha.and_hms_opt(0, 0, 0).unwrap()).unwrap();

        //creo la fecha actual y comparo
        let fecha_actual = Local::now();

        //comparo y devuelvo
        if fecha_vencimiento < fecha_actual {
            return true;
        }
        else {
            return false
        }
    }
    false
    }
}

// PartialEq comparando todo menos el monto
impl PartialEq for Tarjeta {
    fn eq(&self, other: &Self) -> bool {
        self.codigo_seguridad == other.codigo_seguridad &&
            self.dni_titular == other.dni_titular &&
            self.anio_caducidad == other.anio_caducidad &&
            self.mes_caducidad == other.mes_caducidad &&
            self.nombre_titular == other.nombre_titular &&
            self.numero_tarjeta == other.numero_tarjeta
    }
}