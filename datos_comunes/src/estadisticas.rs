use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryEstadisticas {
    pub fecha_inicial: Option<DateTime<Local>>,
    pub fecha_final: Option<DateTime<Local>>,
    pub id_sucursal: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResponseEstadisticas {
    pub query_fecha_inicial: Option<DateTime<Local>>,
    pub query_fecha_final: Option<DateTime<Local>>,
    pub query_nombre_sucursal: Option<String>,
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