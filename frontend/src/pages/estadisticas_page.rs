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

    let ver_trueques_state = use_state(|| {false});
    let ver_ventas_state = use_state(|| {false});

    let estadisticas_mostradas = use_state(|| {None});

    let fecha_minima_state_c = fecha_minima_state.clone();
    let fecha_maxima_state_c = fecha_maxima_state.clone();

    
    let ver_trueques_state_c = ver_trueques_state.clone();
    let ver_ventas_state_c = ver_ventas_state.clone();
    let estadisticas_mostradas_c = estadisticas_mostradas.clone();
    let calcular_estadisticas = move || {
        let ver_trueques_state = ver_trueques_state_c.clone();
        let ver_ventas_state = ver_ventas_state_c.clone();
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
            ver_trueques: *ver_trueques_state, 
            ver_ventas: *ver_ventas_state, 
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

    let calcular_estadisticas_c = calcular_estadisticas.clone();
    let ver_trueques_cliqueado = Callback::from(move |event: MouseEvent| {
        let target = event.target().unwrap();
        let input: HtmlInputElement = target.unchecked_into();
        let cliqueado = input.checked();
        log::info!("Click trueques!!! {cliqueado}");
        ver_trueques_state.set(cliqueado);
        log::info!("Query de estadisticas desde ver_trueques_cliqueado");
        calcular_estadisticas_c();
    });

    let calcular_estadisticas_c = calcular_estadisticas.clone();
    let ver_ventas_cliqueado = Callback::from(move |event: MouseEvent| {
        let target = event.target().unwrap();
        let input: HtmlInputElement = target.unchecked_into();
        let cliqueado = input.checked();
        log::info!("Click ventas!!! {cliqueado}");
        ver_ventas_state.set(cliqueado);
        log::info!("Query de estadisticas desde ver_ventas_cliqueado");
        calcular_estadisticas_c();
    });

    html!{
        <>
            <div>
                <label> {"Desde esta fecha:"} </label>
            </div>
            <input type="date" name="fecha_desde" onchange={on_cambiada_fecha_minima}/>
            <div>
                <label> {"Hasta esta fecha:"} </label>
            </div>
            <input type="date" name="fecha_desde" onchange={on_cambiada_fecha_maxima}/>
            <br/>
            <input type="checkbox" id="trueques" onclick={ver_trueques_cliqueado}/>
            <label for="trueques"> {"Ver Trueques"} </label>
            <br/>
            <input type="checkbox" id="ventas" onclick={ver_ventas_cliqueado}/>
            <label for="ventas"> {"Ver Ventas"} </label>

            <br/>
            if let Some(est) = (*estadisticas_mostradas).clone() {
                <p> {format!("Cantidad de ventas: {}", est.cantidad_ventas)} </p>
                <p> {format!("Total recaudado en ventas: {}", est.pesos_ventas)} </p>
                <p> {format!("Cantidad de trueques: {}", est.cantidad_trueques)} </p>
                <p> {format!("Total recaudado en trueques: {}", est.pesos_trueques)} </p>
                <p> {format!("Total recaudado: {}", est.pesos_ventas + est.pesos_trueques)} </p>
            } else {
                <p> {"Calculando..."} </p>
            }
        </>
    }
}