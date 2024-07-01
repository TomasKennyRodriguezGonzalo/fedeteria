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
    pub cantidad_trueques: usize,
    pub cantidad_ventas: usize,
    pub pesos_trueques: usize,
    pub pesos_ventas: usize,
}