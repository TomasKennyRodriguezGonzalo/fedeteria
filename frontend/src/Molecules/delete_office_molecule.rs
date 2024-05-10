use std::str::FromStr;

use serde_json::from_str;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use crate::Components::generic_button::GenericButton;
use crate::Components::generic_input_field::GenericInputField;
use crate::Components::{generic_button, generic_input_field};
use datos_comunes::{self, QueryDeleteOffice, ResponseDeleteOffice, ResponseGetOffices, Sucursal};
use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use crate::store::UserStore;
use crate::router::Route;


#[function_component(DeleteOfficeMolecule)]
pub fn delete_office_molecule () -> Html {
    let state_office_list = use_state(|| Vec::new());
    let state_office_list_clone = state_office_list.clone();

    let get_offices = Callback::from(move |()| {
        let state_office_list_clone = state_office_list_clone.clone(); {
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
    
    let input_text = use_state(|| "".to_string());
    let input_text_clone = input_text.clone();
    let input_text_changed = Callback::from(move |text|{
        input_text_clone.set(text);
    });



    let state_office_list_clone = state_office_list.clone();

    let oficces_deleted_boolean = use_state(|| false);
    let oficces_deleted_boolean_clone = oficces_deleted_boolean.clone();

    let input_text_clone_2 = input_text.clone();
    let delete_office = Callback::from(move |()| {
            let oficces_deleted_boolean_clone = oficces_deleted_boolean_clone.clone();
            let state_office_list_clone = state_office_list_clone.clone();
            let input_text_clone_3 = &*input_text_clone_2.clone();
                { 
                    let state_office_list_clone = state_office_list_clone.clone();
                    let input_text_clone_4 = input_text_clone_3.clone();
                    spawn_local(async move {
                        log::info!("entre al spawn local");
                        //let cloned_error_state = cloned_error_state.clone();
                        let query = QueryDeleteOffice {office_to_delete: input_text_clone_4.clone()};
                        let respuesta = Request::post("/api/eliminar_sucursal")
                                                                        .header("Content-Type", "application/json")
                                                                        .body(serde_json::to_string(&query).unwrap())
                                                                        .send()
                                                                        .await;
                        match respuesta{
                            Ok(respuesta) =>{
                                let response:Result<ResponseDeleteOffice, reqwasm::Error> = respuesta.json().await;
                                log::info!("deserailice la respuesta {:?}",response);
                                match response{
                                    Ok(respuesta) => {
                                        oficces_deleted_boolean_clone.set(true);
                                        state_office_list_clone.set(respuesta.respuesta.clone());
                                        log::info!("{:?}", respuesta.respuesta);
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

    let state_office_list_clone = state_office_list.clone();
    log::info!("state office value: {:?}",*state_office_list_clone);


    html!(
        <div class="delete_office_box">
            <h1>{"Eliminar Sucursal"}</h1>
            <section>
            <GenericButton text = "Ver Sucursales" onclick_event = {get_offices} />
            <ul class="office_list">
                    if !(&*state_office_list_clone).is_empty() {
                        {(&*state_office_list_clone).iter().map(|sucursal|{html!{<p>{"sucursal: "}{sucursal.nombre.clone()}</p>}}).collect::<Html>()}
                        <GenericInputField name ="Sucursal a Borrar" label="Ingrese sucursal" tipo = "delete_offfice" handle_on_change = {input_text_changed.clone()} />
                        <GenericButton text = "Confirmar Eliminación" onclick_event = {delete_office.clone()} />
                    } else{
                        <h1>{"No existen sucursales :("}</h1>
                    }
            </ul>
            </section>
        </div>
    )
}