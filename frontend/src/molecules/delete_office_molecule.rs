use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use datos_comunes::{self, QueryDeleteOffice, ResponseDeleteOffice, ResponseGetOffices};
use reqwasm::http::Request;
use yewdux::use_store;
use crate::components::generic_button::GenericButton;
use crate::components::indexed_button::IndexedButton;
use crate::information_store::InformationStore;
use crate::molecules::confirm_prompt_button_molecule::ConfirmPromptButtonMolecule;


#[function_component(DeleteOfficeMolecule)]
pub fn delete_office_molecule () -> Html {
    let state_office_list = use_state(|| Vec::new());
    let state_office_list_clone = state_office_list.clone();

    let (information_store, information_dispatch) = use_store::<InformationStore>();
    
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
    
    let show_button_state = use_state(|| false);
    let cloned_show_button_state = show_button_state.clone();
    let reject_office = Callback::from(move |_e:MouseEvent|{
        let cloned_show_button_state = cloned_show_button_state.clone();
        cloned_show_button_state.set(false);
    });
    
    let cloned_show_button_state = show_button_state.clone();
    
    let state_index_office_to_delete = use_state(|| 0);
    let state_index_office_to_delete_clone = state_index_office_to_delete.clone();
    let change_index_office_to_delete = Callback::from(move |index: usize| {
        state_index_office_to_delete_clone.set(index);
        cloned_show_button_state.set(true);
    });
    let state_index_office_to_delete_clone = state_index_office_to_delete.clone(); 
    let cloned_show_button_state = show_button_state.clone();
    
    let cloned_information_dispatch = information_dispatch.clone();
    let delete_office = Callback::from(move |_e: MouseEvent| {
        let cloned_information_dispatch = cloned_information_dispatch.clone();
        cloned_show_button_state.set(false);
        let index = &*state_index_office_to_delete_clone.clone(); 
        let oficces_deleted_boolean_clone = oficces_deleted_boolean_clone.clone();
        let state_office_list_clone = state_office_list_clone.clone();
        let office_to_delete = (&state_office_list_clone).clone().get(*index).unwrap().nombre.clone();
        let informe_cloned = informe_cloned.clone();
        {   
                    let cloned_information_dispatch = cloned_information_dispatch.clone();
                    let informe_cloned = informe_cloned.clone();
                    let state_office_list_clone = state_office_list_clone.clone();
                    spawn_local(async move {
                        let cloned_information_dispatch = cloned_information_dispatch.clone();
                        let office_to_delete = office_to_delete.clone();
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
                                        cloned_information_dispatch.reduce_mut(|store| store.messages.push(format!("Sucursal {} eliminada con éxito", office_to_delete)));
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
    });

    let state_office_list_clone = &*state_office_list.clone();
    log::info!("state office value: {:?}",*state_office_list_clone);

    html!(
        <div class="delete-office-box">
            <h1 class="title">{"Eliminar Sucursal"}</h1>
            <section>
            <GenericButton text = "Ver Sucursales" onclick_event = {get_offices} />
            <ul class="office-list">
            if &*clicks != &0 {
                if !state_office_list_clone.is_empty() {
                    <div class="showing-offices">
                    {
                        state_office_list_clone.iter().enumerate().map(|(index, sucursal)| {
                            html!(
                                <li class="show-office">
                                    <h2>{ sucursal.nombre.clone() }</h2>
                                    <IndexedButton text="Borrar Sucursal" index={index.clone()} onclick_event={change_index_office_to_delete.clone()}/>
                                </li>
                            )
                        }).collect::<Html>()
                    }
                    if !(&*informe).is_empty() {
                        <h2 class="error-text">{&*informe}</h2>
                    }
                    </div>
                } else{
                    <h1>{"No existen sucursales"}</h1>
                }
            }
            </ul>
            </section>
            if (&*show_button_state).clone(){
                <ConfirmPromptButtonMolecule text = {format!("¿Desea eliminar la sucursal {}?", (&*state_office_list)[*state_index_office_to_delete].nombre )} confirm_func = {delete_office} reject_func = {reject_office}  />
            }
        </div>
    )
}