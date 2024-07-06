use chrono::{Local, NaiveDate, TimeZone};
use datos_comunes::{QueryEstadisticas, ResponseEstadisticas};
use log::Log;
use web_sys::HtmlInputElement;
use yew_hooks::use_effect_once;
use yew_router::prelude::Link;
use yew::prelude::*;
use yewdux::use_store;
use crate::{request_post, router::Route, store::UserStore};
use wasm_bindgen::JsCast;

#[function_component(EstadisticasPage)]
pub fn log_in_page()-> Html{


    let (store, _dispatch) = use_store::<UserStore>();
    let dni = store.dni;

    let fecha_minima_state = use_state(|| {"".to_string()});
    let fecha_maxima_state = use_state(|| {"".to_string()});

    let estadisticas_mostradas = use_state(|| {None});

    let fecha_minima_state_c = fecha_minima_state.clone();
    let fecha_maxima_state_c = fecha_maxima_state.clone();

    
    let estadisticas_mostradas_c = estadisticas_mostradas.clone();
    let calcular_estadisticas = move || {
        let estadisticas_mostradas = estadisticas_mostradas_c.clone();
        let fecha_minima = (*fecha_minima_state_c).clone();
        let fecha_maxima = (*fecha_maxima_state_c).clone();
        let fecha_minima = if fecha_minima.is_empty() {None} else {Some(fecha_minima)};
        let fecha_maxima = if fecha_maxima.is_empty() {None} else {Some(fecha_maxima)};

        let fecha_minima = fecha_minima.map(|str_fecha| {
            let fecha = NaiveDate::parse_from_str(&str_fecha, "%Y-%m-%d").unwrap();
            Local.from_local_datetime(&fecha.into()).unwrap()
        });
        let fecha_maxima = fecha_maxima.map(|str_fecha| {
            let fecha = NaiveDate::parse_from_str(&str_fecha, "%Y-%m-%d").unwrap();
            Local.from_local_datetime(&fecha.into()).unwrap()
        });

        let query = QueryEstadisticas { 
            fecha_inicial: fecha_minima, 
            fecha_final: fecha_maxima, 
            id_sucursal: None,
        };
        log::info!("Query de estadisticas: {query:?}");

        estadisticas_mostradas.set(None);
        request_post("/api/get_estadisticas", query, move |response: ResponseEstadisticas| {
            log::info!("Respuesta de estadisticas: {response:?}");
            estadisticas_mostradas.set(Some(response));
        });
    };

    let calcular_estadisticas_c = calcular_estadisticas.clone();
    use_effect_once(move || {
        calcular_estadisticas_c();
        || {}
    });

    let fecha_minima_state_c = fecha_minima_state.clone();
    let calcular_estadisticas_c = calcular_estadisticas.clone();
    let on_cambiada_fecha_minima = Callback::from(move |event : Event| {
        let target = event.target().unwrap();
        let input:HtmlInputElement = target.unchecked_into();
        let input_value = input.value();
        log::info!("fecha minima: {input_value}");
        fecha_minima_state_c.set(input_value);
        log::info!("Query de estadisticas desde on_cambiada_fecha_minima");
        calcular_estadisticas_c();
    });


    let fecha_maxima_state_c = fecha_maxima_state.clone();
    let calcular_estadisticas_c = calcular_estadisticas.clone();
    let on_cambiada_fecha_maxima = Callback::from(move |event : Event| {
        let target = event.target().unwrap();
        let input:HtmlInputElement = target.unchecked_into();
        let input_value = input.value();
        log::info!("fecha maxima: {input_value}");
        fecha_maxima_state_c.set(input_value);
        log::info!("Query de estadisticas desde on_cambiada_fecha_maxima");
        calcular_estadisticas_c();
    });


    html!{
        <>
            // <p>
            // {format!("Hola!!!!, {dni:?}")}
            // </p>
            <div>
                <label> {"Desde esta fecha:"} </label>
            </div>
            <input type="date" name="fecha_desde" onchange={on_cambiada_fecha_minima}/>
            <div>
                <label> {"Hasta esta fecha:"} </label>
            </div>
            <input type="date" name="fecha_desde" onchange={on_cambiada_fecha_maxima}/>

            <div>
                <label> {"En esta sucursal:"} </label>
            </div>

            <br/>
            if let Some(est) = (*estadisticas_mostradas).clone() {
                <p> {crear_string_para_resp(&est)} </p>
                <p> {format!("Cantidad de trueques finalizados: {}", est.cantidad_trueques_finalizados)} </p>
                <p> {format!("Cantidad de trueques rechazados: {}", est.cantidad_trueques_rechazados)} </p>
                <p> {format!("Cantidad de trueques total: {}", est.cantidad_trueques_rechazados_o_finalizados)} </p>
                <p> {format!("Cantidad de trueques con ventas: {}", est.cantidad_trueques_con_ventas)} </p>
                <p> {format!("Cantidad de trueques finalizados con ventas: {}", est.cantidad_trueques_finalizados_con_ventas)} </p>
                <p> {format!("Pesos recaudados por ventas en trueques finalizados: {}", est.pesos_trueques_finalizados)} </p>
                <p> {format!("Cantidad de trueques rechazados con ventas: {}", est.cantidad_trueques_rechazados_con_ventas)} </p>
                <p> {format!("Pesos recaudados por ventas en trueques rechazados: {}", est.pesos_trueques_rechazados)} </p>
                <p> {format!("Total recaudado por ventas: {}", est.pesos_trueques)} </p>
            } else {
                <p> {"Calculando..."} </p>
            }
        </>
    }
}

fn crear_string_para_resp(resp: &ResponseEstadisticas) -> String {
    let fecha_inicial = resp.query_fecha_inicial;
    let fecha_final = resp.query_fecha_final;
    let nombre_sucursal = resp.query_nombre_sucursal.clone();
    let mut s = String::new();
    s += "Mostrando estadísticas";
    let mut alguna_fecha = false;
    s += &match (fecha_inicial, fecha_final) {
        (None, None) => "".to_string(),
        (Some(fecha_inicial), None) => {
            alguna_fecha = true;
            format!(" desde {}", fecha_inicial.format("%Y-%m-%d"))
        },
        (None, Some(fecha_final)) => {
            alguna_fecha = true;
            format!(" hasta {}", fecha_final.format("%Y-%m-%d"))
        },
        (Some(fecha_inicial), Some(fecha_final)) => {
            alguna_fecha = true;
            format!(" entre {} y {}", fecha_inicial.format("%Y-%m-%d"), fecha_final.format("%Y-%m-%d"))
        },
    };
    if let Some(sucursal) = nombre_sucursal {
        if alguna_fecha {
            s += ",";
        }
        s += &format!(" para la sucursal {}", sucursal);
    } else {
        if !alguna_fecha {
            s += " totales"
        }
    }
    s += ".";
    return s;
}

// test para probar que queden bien todas las variantes
#[test]
fn test_crear_string_para_resp() {
    let fecha = NaiveDate::parse_from_str("2024-07-07", "%Y-%m-%d").unwrap();
    let fecha_inicial = Local.from_local_datetime(&fecha.into()).unwrap();
    let fecha = NaiveDate::parse_from_str("2024-07-17", "%Y-%m-%d").unwrap();
    let fecha_final = Local.from_local_datetime(&fecha.into()).unwrap();
    let nombre_sucursal = "La Plata";

    let fecha_inicial = Some(fecha_inicial);
    let fecha_final = Some(fecha_final);
    let nombre_sucursal = Some(&nombre_sucursal);

    let casos_posibles = [
        (fecha_inicial, fecha_final, nombre_sucursal),
        (None, fecha_final, nombre_sucursal),
        (fecha_inicial, None, nombre_sucursal),
        (fecha_inicial, fecha_final, None),
        (None, None, nombre_sucursal),
        (fecha_inicial, None, None),
        (None, fecha_final, None),
        (None, None, None),
    ];
    for caso in casos_posibles {
        let resp = ResponseEstadisticas {
            query_fecha_inicial: caso.0,
            query_fecha_final: caso.1,
            query_nombre_sucursal: caso.2.map(|s| s.to_string()),
            ..Default::default()
        };
        println!("Caso: {caso:?}. Texto: {}", crear_string_para_resp(&resp));
    }
}