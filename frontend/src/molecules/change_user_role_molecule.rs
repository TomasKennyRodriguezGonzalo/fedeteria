use web_sys::window;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use datos_comunes::{self, QueryChangeUserRole, QueryDeleteOffice, QueryGetUserRole, ResponseDeleteOffice, ResponseGetOffices, ResponseGetUserRole, RolDeUsuario, Sucursal};
use reqwasm::http::Request;
use yewdux::use_store;
use crate::components::dni_input_field::DniInputField;
use crate::components::checked_input_field::CheckedInputField;
use crate::components::generic_button::GenericButton;
use crate::information_store::InformationStore;

#[function_component(ChangeUserRoleMolecule)]
pub fn change_user_rol_molecule () -> Html {

    let (_information_store, information_dispatch) = use_store::<InformationStore>();

    let dni_state = use_state(|| 0);
    let cloned_dni_state = dni_state.clone();
    let role_state = use_state(|| None);
    let cloned_role_state = role_state.clone();
    let dni_state_changed = Callback::from(move |dni:String|{
        cloned_dni_state.set(dni.parse::<u64>().unwrap());
        cloned_role_state.set(None);
    });
    
    let select_value_state = use_state(|| -1);
    
    
    let informe = use_state(|| "".to_string());
    let informe_cloned = informe.clone();
    
    let clicks = use_state(|| 0);
    let clicks_cloned = clicks.clone();
    
    let state_office_list: UseStateHandle<Vec<Sucursal>> = use_state(|| Vec::new());
    let state_office_list_clone = state_office_list.clone();
    
    let cloned_role_state = role_state.clone();
    
    let dni_state_cloned = dni_state.clone();
    let search_user = Callback::from(move |()| {
        let dni_state_cloned = dni_state_cloned.clone();
        let state_office_list_clone = state_office_list_clone.clone();
        clicks_cloned.set(&*clicks_cloned + 1);
        let cloned_dni = dni_state_cloned.clone();
        let cloned_role_state = cloned_role_state.clone();
        let informe_cloned = informe_cloned.clone();
        {
            spawn_local(async move {
                let state_office_list_clone = state_office_list_clone.clone();
                let cloned_dni = &*cloned_dni;
                let informe_cloned = informe_cloned.clone();
                let query = QueryGetUserRole { dni : *cloned_dni };
                let response = Request::post("/api/obtener_rol").header("Content-Type", "application/json").body(serde_json::to_string(&query).unwrap()).send().await;
                match response{
                    Ok(response) => {
                        let response:Result<Option<ResponseGetUserRole>, reqwasm::Error> = response.json().await;
                        log::info!("deserialice la respuesta {:?}", response);
                        match response{
                            Ok(response) => {  
                                if response.is_some(){
                                    let user_role = response.unwrap().rol;
                                    cloned_role_state.set(Some(user_role.clone()));
                                    spawn_local(async move {
                                        let state_office_list_clone = state_office_list_clone.clone();
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
                                }else{
                                    log::error!("user not found (frontend)");
                                    informe_cloned.set(format!("El usuario con DNI {} no existe.", *cloned_dni))
                                }     
                            }
                            Err(error)=>{
                                informe_cloned.set("Ocurrio un error".to_string());
                                log::error!("Error en deserializacion: {}", error);
                            }
                        }
                    }
                    Err(error)=>{
                        log::error!("Error en llamada al backend: {}", error);
                        informe_cloned.set("Ocurrio un error".to_string())
                    }
                }
            });
        }
    });


    let information_dispatch_cloned = information_dispatch.clone();
    let cloned_role_state = role_state.clone();
    let dni_state_cloned = dni_state.clone();
    let change_user_to_normal = Callback::from(move |()| {
        change_role (*dni_state_cloned.clone(), RolDeUsuario::Normal);
        information_dispatch_cloned.reduce_mut( |store| store.messages.push(format!("El usuario con DNI {} cambió su rol a Normal.", (&*dni_state_cloned.clone()))));
        cloned_role_state.set(None);
    });
    
    let cloned_role_state = role_state.clone();
    let information_dispatch_cloned = information_dispatch.clone();
    let dni_state_cloned = dni_state.clone();
    let select_value_state_cloned = select_value_state.clone();
    let change_user_to_employed = Callback::from(move |()| {
        let select_value_state_cloned = select_value_state_cloned.clone();
        change_role (*dni_state_cloned.clone(), RolDeUsuario::Empleado { sucursal: ((&*select_value_state_cloned).clone() as usize) });
        information_dispatch_cloned.reduce_mut( |store| store.messages.push(format!("El usuario con DNI {} cambió su rol a Empleado de la sucursal {}.", (&*dni_state_cloned.clone()), ((&*select_value_state_cloned).clone() as usize))));
        cloned_role_state.set(None);
    });
    
    let cancel_change = Callback::from( move |()| {
        // Refreshes to reset the first load states all over the code
        if let Some(window) = window() {
            window.location().reload().unwrap();
        }
    });
    
    let cloned_role_state = role_state.clone();
    let information_dispatch_cloned = information_dispatch.clone();
    let dni_state_cloned = dni_state.clone();
    let change_user_to_owner = Callback::from(move |()| {
        change_role (*dni_state_cloned.clone(), RolDeUsuario::Dueño);
        information_dispatch_cloned.reduce_mut( |store| store.messages.push(format!("El usuario con DNI {} cambió su rol a Dueño.", (&*dni_state_cloned.clone()))));
        cloned_role_state.set(None);
    });
    
    let select_value_state_cloned = select_value_state.clone();
    let select_onchange = Callback::from(move|event: Event| {
        let select_value_state_cloned = select_value_state_cloned.clone();
        let target = event.target().unwrap();
        let input:HtmlInputElement = target.unchecked_into();
        let value: i32 = input.value().parse().unwrap();
        select_value_state_cloned.set(value);
        log::info!("Select changed to {}", value)
    });
    
    let dni_state_cloned = dni_state.clone();
    let cloned_role_state = role_state.clone();

 

    html!(
        <div class="change-user-role-box">
            <h1 class="title">{"Cambiar Rol de Usuario"}</h1>
            <div class="dni-input">
                <DniInputField dni="DNI" label="Ingrese DNI del usuario al que desea cambiarle el Rol" tipo ="number" handle_on_change = {dni_state_changed} />
                <GenericButton text="Buscar Usuario" onclick_event={search_user}/>
            </div>
            if (&*cloned_role_state).is_some() {
                    {
                       match (&*role_state).clone() { 
                        Some(role) => {
                            match role {
                                RolDeUsuario::Normal | RolDeUsuario::Dueño => {html!{<h1 class="user-info">{format!("El usuario DNI {} actualmente tiene rol {:?}", &*dni_state_cloned.clone(), role)}</h1>}},
                                RolDeUsuario::Empleado { sucursal } => {html!{<h1 class="user-info">{format!("El usuario DNI {} actualmente tiene rol Empleado en la sucursal {}", &*dni_state_cloned.clone(), sucursal)}</h1>}},
                            }
                        }
                        None => {html!{}}
                        }
                    }
                <ul class="change-role-option-list">
                    <h2>{format!("Seleccione que rol desea darle al usuario con DNI {}", (&*dni_state))}</h2>
                    <li><GenericButton text="Cambiar a Normal" onclick_event={change_user_to_normal}/></li>
                    <li>
                        <div class="employee-option">
                            <label for="select-employee">{"Seleccione una sucursal para el empleado"}</label>
                            <br/>
                            <select value="select-employee" id="sucursales" onchange={select_onchange}>
                                <option value="-1">{"---"}</option>
                                {
                                    (&*state_office_list).iter().enumerate().map(|(index, sucursal)| html!{
                                        <option value={index.to_string()}>{sucursal.nombre.clone()}</option>
                                    }).collect::<Html>()
                                }
                            </select>
                            if (&*select_value_state).clone() != -1 { 
                                <GenericButton text="Cambiar a Empleado" onclick_event={change_user_to_employed}/>
                            } else {
                                <button class="disabled-dyn-element">{"Cambiar a Empleado"}</button>
                            }
                        </div>
                    </li>
                    <li><GenericButton text="Cambiar a Dueño" onclick_event={change_user_to_owner}/></li>
                    <li><GenericButton text="Cancelar" onclick_event={cancel_change}/></li>
                </ul>
            } else if !(&*informe.clone()).is_empty() {
                <h1 class="error-text">{&*informe}</h1>
            }
        </div>
    )
}

fn change_role (dni: u64, rol: RolDeUsuario) {
    {
        spawn_local(async move {
            log::info!("entre al spawn local");
            let query = QueryChangeUserRole {dni: dni.clone(), new_role: rol.clone()};
            let _respuesta = Request::post("/api/cambiar_rol_usuario")
                                                            .header("Content-Type", "application/json")
                                                            .body(serde_json::to_string(&query).unwrap())
                                                            .send()
                                                            .await;
            if let Some(window) = window() {
                window.location().reload().unwrap();
            }
        });
    }
}