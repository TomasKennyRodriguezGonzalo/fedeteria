use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryEstadisticas {
    pub fecha_inicial: Option<DateTime<Local>>,
    pub fecha_final: Option<DateTime<Local>>,
    pub ver_trueques: bool,
    pub ver_ventas: bool,
    pub id_sucursal: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseEstadisticas {
    pub cantidad_trueques_rechazados_o_finalizados: usize,
    pub cantidad_trueques_con_ventas: usize,
    pub pesos_trueques: u64,
    pub cantidad_trueques_rechazados: usize,
    pub cantidad_trueques_finalizados: usize,
    pub cantidad_trueques_rechazados_con_ventas: usize,
    pub pesos_trueques_rechazados: u64,
    pub cantidad_trueques_finalizados_con_ventas: usize,
    pub pesos_trueques_finalizados: u64,
}