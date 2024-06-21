use datos_comunes::{EstadoTrueque, QueryTruequesFiltrados, ResponseGetOffices};
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew_router::hooks::use_navigator;
use crate::components::generic_button::GenericButton;
use crate::molecules::trade_grid_molecule::TradeGridMolecule;
use crate::router::Route;
use yew::prelude::*;
use yew_hooks::use_effect_once;
use reqwasm::http::Request;
use wasm_bindgen::JsCast;

#[function_component(DefinedTradesPage)]
pub fn defined_trades_page () -> Html {

    let query = QueryTruequesFiltrados {
        filtro_codigo_ofertante: None,
        filtro_codigo_receptor: None,
        filtro_dni_integrantes: None,
        filtro_estado: Some(EstadoTrueque::Definido),
        filtro_fecha_pactada: None,
        filtro_fecha_trueque: None,
        filtro_id_publicacion: None,
        filtro_sucursal: None,
    };

    //traigo las sucursales
    let office_list_state = use_state(|| Vec::new());
    let cloned_office_list_state = office_list_state.clone();
    use_effect_once(move || {
        let cloned_office_list_state = cloned_office_list_state.clone();
        spawn_local(async move {
            log::info!("entre al spawn local");
            let respuesta = Request::get("/api/obtener_sucursales")
            .header("Content-Type", "application/json")
            .send()
                                                    .await;
            match respuesta{
                Ok(respuesta) =>{
                    let response:Result<ResponseGetOffices, reqwasm::Error> = respuesta.json().await;
                    log::info!("deserailice la respuesta {:?}",response);
                    match response{
                        Ok(respuesta) => {           
                            cloned_office_list_state.set(respuesta.office_list);
                        }
                        Err(error)=>{
                            log::error!("Error en deserializacion: {}", error);
                        }
                    }
                }
                Err(error)=>{
                    log::error!("Error en llamada al backend: {}", error);
                    
                }
            }
            
        });
        || {}
    });

    //select_sucursal
    let select_sucursal_value_state = use_state(|| -1);
    let select_sucursal_value_state_cloned = select_sucursal_value_state.clone();
    let select_sucursal_onchange = Callback::from(move|event: Event| {
        let select_sucursal_value_state_cloned = select_sucursal_value_state_cloned.clone();
        let target = event.target().unwrap();
        let input:HtmlInputElement = target.unchecked_into();
        let value: i32 = input.value().parse().unwrap();
        select_sucursal_value_state_cloned.set(value);
        log::info!("Select sucursal changed to {}", value)
    });

    //busco los trueques de la sucursal ingresada
    let navigator = use_navigator().unwrap();
    let cloned_query = query.clone();
    let cloned_select_sucursal_value_state = select_sucursal_value_state.clone();
    let search_defined_trades_office = Callback::from(move |()| {
        let navigator_cloned = navigator.clone();
        let mut cloned_query = cloned_query.clone();
        let selected_office_index = (&*cloned_select_sucursal_value_state).clone() as usize;
        cloned_query.filtro_sucursal = Some(selected_office_index);

        let _ = navigator_cloned.push_with_query(&Route::SearchTrueques, &cloned_query);
    });

    let cloned_office_list_state = office_list_state.clone();

    html! {
       <div class="show-defined-trades-options">
            <select value="select-sucursal" id="sucursales" onchange={select_sucursal_onchange.clone()}>
                <option value="-1">{"---"}</option>
                {
                    (&*cloned_office_list_state).iter().enumerate().map(|(index, sucursal)| html!{
                        <option value={index.to_string()}>{sucursal.nombre.clone()}</option>
                    }).collect::<Html>()
                }
            </select>
            <br/>
            if (&*select_sucursal_value_state).clone() != -1 {
                <GenericButton text="Buscar Trueques Definidos de Sucursal" onclick_event={search_defined_trades_office}/>
            }
            <TradeGridMolecule query={Some(query)}/>
       </div>
    }
}