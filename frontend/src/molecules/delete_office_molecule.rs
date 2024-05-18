use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use datos_comunes::{self, QueryDeleteOffice, ResponseDeleteOffice, ResponseGetOffices, Sucursal};
use reqwasm::http::Request;
use crate::components::generic_input_field::GenericInputField;
use crate::components::generic_button::GenericButton;
//use crate::components::indexed_button::IndexedButton;


#[function_component(DeleteOfficeMolecule)]
pub fn delete_office_molecule () -> Html {
    let state_office_list = use_state(|| Vec::new());
    let state_office_list_clone = state_office_list.clone();

    let clicks = use_state(|| 0);
    let clicks_cloned = clicks.clone();

    let get_offices = Callback::from(move |()| {
        let state_office_list_clone = state_office_list_clone.clone();
        clicks_cloned.set(&*clicks_cloned + 1);  {
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

    let state_office_list_clone = state_office_list.clone();

    let oficces_deleted_boolean = use_state(|| false);
    let oficces_deleted_boolean_clone = oficces_deleted_boolean.clone();

    let informe = use_state(|| "".to_string());
    let informe_cloned = informe.clone();

    let state_input_text = use_state(|| "".to_string());
    let state_input_text_clone = state_input_text.clone();
    let state_input_text_changed = Callback::from(move |text|{
        state_input_text_clone.set(text);
    });
    let state_input_text_clone = state_input_text.clone();

    let state_result_prompt = use_state(|| false);
    let state_result_prompt_cloned = state_result_prompt.clone();
    let change_to_true_result_prompt = Callback::from(move |()| {
        state_result_prompt_cloned.set(true)
    });
    let state_result_prompt_cloned = state_result_prompt.clone();
    let change_to_false_result_prompt = Callback::from(move |()| {
        state_result_prompt_cloned.set(false)
    });


    let delete_office = Callback::from(move |()/*index: usize*/| {
        let state_input_text_clone = state_input_text_clone.clone();
        let office_to_delete = &*state_input_text_clone.clone();
        let state_input_text_clone = state_input_text_clone.clone();
        let oficces_deleted_boolean_clone = oficces_deleted_boolean_clone.clone();
        let state_office_list_clone = state_office_list_clone.clone();
        //let office_to_delete = (&state_office_list_clone).clone().get(index).unwrap().nombre.clone();
        let informe_cloned = informe_cloned.clone();
            {   
                if existe_sucursal(&*state_office_list_clone.clone(), office_to_delete.to_string()) {
                    let informe_cloned = informe_cloned.clone();
                    let state_office_list_clone = state_office_list_clone.clone();
                    //let office_to_delete = office_to_delete.clone();
                    spawn_local(async move {
                        let office_to_delete = &*state_input_text_clone.clone();
                        log::info!("entre al spawn local");
                        let query = QueryDeleteOffice {office_to_delete: office_to_delete.clone()};
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
                                        informe_cloned.set("Sucursal Eliminada".to_string());
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
                    informe_cloned.set("La sucursal ingresada no existe".to_string());
                }
            }
    });

    let state_office_list_clone = &*state_office_list.clone();
    log::info!("state office value: {:?}",*state_office_list_clone);

    html!(
        <div class="delete_office_box">
            <h1>{"Eliminar Sucursal"}</h1>
            <section>
            <GenericButton text = "Ver Sucursales" onclick_event = {get_offices} />
            <ul class="office_list">
            if &*clicks != &0 {
                if !state_office_list_clone.is_empty() {
                    <div class="showing-offices">
                    {
                        state_office_list_clone.iter().enumerate().map(|(index, sucursal)| {
                            html!(
                                <div class="show-office">
                                //<h2>{ sucursal.nombre.clone() }</h2>
                                //<IndexedButton text="Borrar Sucursal" index={index.clone()} onclick_event={delete_office.clone()}/>
                                    <h2> { format!("{}: {}", index, sucursal.nombre.clone()) }</h2>
                                </div>
                            )
                        }).collect::<Html>()
                    }
                    <GenericInputField name ="Sucursal a Borrar" label="Ingrese numero de sucursal a borrar" tipo = "delete_offfice" handle_on_change = {state_input_text_changed} />
                    <GenericButton text = "Borrar Sucursal" onclick_event = {change_to_true_result_prompt} />
                    if *state_result_prompt {
                        <h2> {"¿Desea eliminar la sucursal ingresada?"}</h2>
                        <GenericButton text="Confirmar" onclick_event={delete_office}/>
                        <GenericButton text="Cancelar" onclick_event={change_to_false_result_prompt}/>
                    }
                    <h2>{&*informe}</h2>
                    </div>
                } else{
                   <h1>{"No existen sucursales"}</h1>
                }
            }
            </ul>
            </section>
        </div>
    )
}

fn existe_sucursal (sucursales: &Vec<Sucursal>, buscada: String) -> bool {
    sucursales.iter().map(|actual| &*actual.nombre).find(|actual| actual.eq(&buscada)).is_some()
}