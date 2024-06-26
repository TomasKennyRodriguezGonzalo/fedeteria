use datos_comunes::{QueryAddOffice, ResponseAddOffice, ResponseGetOffices};
use yew::prelude::*;
use reqwasm::http::Request;
use wasm_bindgen_futures::spawn_local;
use yewdux::use_store;

use crate::{components::{checked_input_field::CheckedInputField, generic_button::GenericButton}, information_store::InformationStore, molecules::confirm_prompt_button_molecule::ConfirmPromptButtonMolecule};

#[function_component(CreateOfficeMolecule)]
pub fn create_office_molecule() -> Html {

    let (_information_store, information_dispatch) = use_store::<InformationStore>();

    let state_office_list = use_state(|| Vec::new());
    let state_office_list_clone = state_office_list.clone();

    let clicks = use_state(|| 0);
    let clicks_cloned = clicks.clone();

    let get_offices = Callback::from(move |()| {
        let state_office_list_clone = state_office_list_clone.clone();
        clicks_cloned.set(&*clicks_cloned + 1);
        {
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
                                    state_office_list_clone.set(respuesta.office_list);
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
        }
    });
    
    let office_name_state = use_state(|| "".to_string());
    let office_name_state_cloned = office_name_state.clone();
    let office_name_changed = Callback::from(move |office_name : String| {
        office_name_state_cloned.set(office_name);
    });
    let office_name_state_cloned = office_name_state.clone();

    let show_button_state = use_state(|| false);
    let cloned_show_button_state = show_button_state.clone();
    let show_button = Callback::from(move |()| {
        cloned_show_button_state.set(true);
    });
    let cloned_show_button_state = show_button_state.clone();
    let hide_button = Callback::from(move |_e:MouseEvent|{
        cloned_show_button_state.set(false);
    });
    let cloned_show_button_state = show_button_state.clone();

    let state_office_list_clone = state_office_list.clone();

    let informe = use_state(|| "".to_string());
    let informe_cloned = informe.clone();

    let cloned_information_dispatch = information_dispatch.clone();
    let add_office = Callback::from(move |_e: MouseEvent| {
        let cloned_information_dispatch = cloned_information_dispatch.clone();
        cloned_show_button_state.set(false);
        let office_to_add = &*office_name_state_cloned.clone();
        let state_office_list_clone = state_office_list_clone.clone();
        let informe_cloned = informe_cloned.clone();
        {   
            let state_office_list_clone = state_office_list_clone.clone();
            let informe_cloned = informe_cloned.clone();
            let office_to_add = office_to_add.clone();
            if !office_to_add.is_empty() {
                spawn_local(async move {
                    let cloned_information_dispatch = cloned_information_dispatch.clone();
                    let state_office_list_clone = state_office_list_clone.clone();
                    let informe_cloned = informe_cloned.clone();
                    let office_to_add = office_to_add.clone();
                    log::info!("entre al spawn local");
                    let query = QueryAddOffice {office_to_add: office_to_add.clone()};
                    let respuesta = Request::post("/api/agregar_sucursal")
                                                                    .header("Content-Type", "application/json")
                                                                    .body(serde_json::to_string(&query).unwrap())
                                                                    .send()
                                                                    .await;
                    match respuesta{
                        Ok(respuesta) =>{
                            let response:Result<ResponseAddOffice, reqwasm::Error> = respuesta.json().await;
                            log::info!("deserailice la respuesta {:?}",response);
                            match response{
                                Ok(respuesta) => {
                                    state_office_list_clone.set(respuesta.respuesta.clone());
                                    if respuesta.agrego {
                                        cloned_information_dispatch.reduce_mut(|store| store.messages.push(format!("Sucursal {} creada con éxito", office_to_add)));
                                    }
                                    else {
                                        informe_cloned.set("Sucursal ya existente".to_string());
                                    }
                                    log::info!("{:?}", respuesta.respuesta);
                                }
                                Err(error)=>{
                                    log::error!("Error en deserializacion: {}", error);
                                    informe_cloned.set("Ocurrio un error".to_string());
                                }
                            }
                        }
                        Err(error)=>{
                            log::error!("Error en llamada al backend: {}", error);
                            informe_cloned.set("Ocurrio un error".to_string());
                        }
                    }
                });
            }
            else {
                informe_cloned.set("No se ha ingresado una sucursal".to_string());
            }
    }
    });

    let onsubmit = Callback::from(move |submit_event : SubmitEvent| {
        submit_event.prevent_default();
    });

    let state_office_list_clone = state_office_list.clone();

    let informe_cloned = informe.clone();

    html!(
        <div class="create-office-box">
            <h1 class="title">{"Agregar Sucursal"}</h1>
            <form onsubmit={onsubmit}>
                <GenericButton text="Ver Sucursales Actuales" onclick_event={get_offices}/>
                if &*clicks != &0 {
                    if !state_office_list_clone.is_empty() {
                        <ul class="showing-offices">
                        {
                            state_office_list_clone.iter().enumerate().map(|(index, sucursal)| {
                                html!(
                                    <li class="show-office">
                                        <h2>{ format!("Nº{}: Sucursal {}", index, sucursal.nombre.clone()) }</h2>
                                    </li>
                                )
                            }).collect::<Html>()
                        }
                        </ul>
                    } else{
                        <h1>{"No existen sucursales"}</h1>
                    }
                }
                <CheckedInputField name="office-name" label="Ingrese el nombre de la sucursal: " tipo="text" on_change={office_name_changed}/>
                <GenericButton text="Cargar Sucursal" onclick_event={show_button}/>
                <h2 class="error-text">{&*informe_cloned}</h2>
            </form>
            if (&*show_button_state).clone() {
                <ConfirmPromptButtonMolecule text = "¿Desea agregar la sucursal ingresada?" confirm_func = {add_office} reject_func = {hide_button}  />
            }
        </div>
    )
}