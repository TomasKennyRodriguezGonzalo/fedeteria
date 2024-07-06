use std::{cell::RefCell, ops::Deref, rc::Rc};

use chrono::{DateTime, Local, NaiveDate, TimeZone};
use datos_comunes::{QueryEstadisticas, ResponseEstadisticas, ResponseGetOffices};
use log::Log;
use web_sys::{HtmlInputElement, StorageEvent};
use yew_hooks::use_effect_once;
use yew_router::prelude::Link;
use yew::prelude::*;
use yewdux::use_store;
use crate::{convenient_request::request_get, request_post, router::Route, store::UserStore};
use wasm_bindgen::JsCast;

#[function_component(EstadisticasPage)]
pub fn log_in_page()-> Html{

    let estadisticas_mostradas = use_state(|| {None});
    let lista_sucursales = use_state(|| vec![]);

    let state_query = use_state(|| Rc::new(RefCell::new(
        QueryEstadisticas {
            ..Default::default()
        }
    )));
    
    let state_query_c = state_query.clone();
    let estadisticas_mostradas_c = estadisticas_mostradas.clone();
    let calcular_estadisticas = move || {
        let state_query = state_query_c.clone();
        let estadisticas_mostradas = estadisticas_mostradas_c.clone();
        // let fecha_minima = (*state_query).fecha_minima.clone();
        // let fecha_maxima = (*state_query).fecha_maxima.clone();
        // let fecha_minima = if fecha_minima.is_empty() {None} else {Some(fecha_minima)};
        // let fecha_maxima = if fecha_maxima.is_empty() {None} else {Some(fecha_maxima)};

        // let fecha_minima = fecha_minima.map(|str_fecha| {
        //     let fecha = NaiveDate::parse_from_str(&str_fecha, "%Y-%m-%d").unwrap();
        //     Local.from_local_datetime(&fecha.into()).unwrap()
        // });
        // let fecha_maxima = fecha_maxima.map(|str_fecha| {
        //     let fecha = NaiveDate::parse_from_str(&str_fecha, "%Y-%m-%d").unwrap();
        //     Local.from_local_datetime(&fecha.into()).unwrap()
        // });

        // let query = (*state_query).clone();
        // log::info!("Query de estadisticas: {query:?}");
        let query = (*state_query_c).borrow().clone();
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

    let lista_sucursales_c = lista_sucursales.clone();
    use_effect_once(move || {
        request_get("/api/obtener_sucursales", move |response: ResponseGetOffices| {
            lista_sucursales_c.set(response.office_list);
        });
        || {}
    });

    let state_query_c = state_query.clone();
    let calcular_estadisticas_c = calcular_estadisticas.clone();
    let on_cambiada_fecha_minima = Callback::from(move |event : Event| {
        let target = event.target().unwrap();
        let input: HtmlInputElement = target.unchecked_into();
        let input_value = input.value();
        let mut query = (*state_query_c).borrow_mut();
        query.fecha_inicial = fecha_string_a_local_option(input_value);
        drop(query);
        // fecha_minima_state_c.set(input_value);
        calcular_estadisticas_c();
    });


    let state_query_c = state_query.clone();
    let calcular_estadisticas_c = calcular_estadisticas.clone();
    let on_cambiada_fecha_maxima = Callback::from(move |event : Event| {
        let target = event.target().unwrap();
        let input: HtmlInputElement = target.unchecked_into();
        let input_value = input.value();
        let mut query = (*state_query_c).borrow_mut();
        query.fecha_final = fecha_string_a_local_option(input_value);
        drop(query);
        calcular_estadisticas_c();
    });

    let state_query_c = state_query.clone();
    let calcular_estadisticas_c = calcular_estadisticas.clone();
    let on_sucursal_elegida = Callback::from(move|event: Event| {
        let target = event.target().unwrap();
        let input: HtmlInputElement = target.unchecked_into();
        let value: i64 = input.value().parse().unwrap();
        
        let sucursal_elegida = if value <= -1 {
            None
        } else {
            Some(value as usize)
        };

        let mut query = (*state_query_c).borrow_mut();
        query.id_sucursal = sucursal_elegida;

        drop(query);
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

                <select id="sucursales" onchange={on_sucursal_elegida}>
                <option value="-1">{"Ninguna"}</option>
                {
                    (*lista_sucursales).iter().enumerate().map(|(index, sucursal)| html!{
                        <option value={index.to_string()}>{sucursal.nombre.clone()}</option>
                    }).collect::<Html>()
                }
            </select>
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
    } else if !alguna_fecha {
        s += " totales"
    }
    s += ".";
    s
}

fn fecha_string_a_local_option(fecha: String) -> Option<DateTime<Local>> {
    if fecha.is_empty() {
        None
    } else {
        let naive_fecha = NaiveDate::parse_from_str(&fecha, "%Y-%m-%d").unwrap();
        let local_fecha = Local.from_local_datetime(&naive_fecha.into()).unwrap();
        Some(local_fecha)
    }
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