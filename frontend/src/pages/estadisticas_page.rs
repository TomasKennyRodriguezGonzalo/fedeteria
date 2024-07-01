use log::Log;
use web_sys::HtmlInputElement;
use yew_router::prelude::Link;
use yew::prelude::*;
use yewdux::use_store;
use crate::{router::Route, store::UserStore};
use wasm_bindgen::JsCast;

#[function_component(EstadisticasPage)]
pub fn log_in_page()-> Html{


    let (store, _dispatch) = use_store::<UserStore>();
    let dni = store.dni;

    let fecha_minima_state = use_state(|| {"".to_string()});
    let fecha_maxima_state = use_state(|| {"".to_string()});

    let ver_trueques_state = use_state(|| {false});
    let ver_ventas_state = use_state(|| {false});

    let fecha_minima_state_c = fecha_minima_state.clone();
    let on_cambiada_fecha_minima = Callback::from(move |event : Event| {
        let target = event.target().unwrap();
        let input:HtmlInputElement = target.unchecked_into();
        let input_value = input.value();
        log::info!("fecha minima: {input_value}");
        fecha_minima_state_c.set(input_value);
    });


    let fecha_maxima_state_c = fecha_maxima_state.clone();
    let on_cambiada_fecha_maxima = Callback::from(move |event : Event| {
        let target = event.target().unwrap();
        let input:HtmlInputElement = target.unchecked_into();
        let input_value = input.value();
        log::info!("fecha maxima: {input_value}");
        fecha_maxima_state_c.set(input_value);
    });

    let ver_trueques_cliqueado = Callback::from(move |event: MouseEvent| {
        let target = event.target().unwrap();
        let input: HtmlInputElement = target.unchecked_into();
        let cliqueado = input.checked();
        log::info!("Click trueques!!! {cliqueado}");
        ver_trueques_state.set(cliqueado);
    });

    let ver_ventas_cliqueado = Callback::from(move |event: MouseEvent| {
        let target = event.target().unwrap();
        let input: HtmlInputElement = target.unchecked_into();
        let cliqueado = input.checked();
        log::info!("Click ventas!!! {cliqueado}");
        ver_ventas_state.set(cliqueado);
    });


    html!{
        <>
            <p>
            {format!("Hola!!!!, {dni:?}")}
            </p>
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
        </>
    }
}