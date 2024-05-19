use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use datos_comunes::{self, QueryChangeUserRole, QueryDeleteOffice, QueryGetUserRole, ResponseDeleteOffice, ResponseGetOffices, ResponseGetUserRole, RolDeUsuario, Sucursal};
use reqwasm::http::Request;
use crate::components::dni_input_field::DniInputField;
use crate::components::checked_input_field::CheckedInputField;
use crate::components::generic_button::GenericButton;
//use crate::components::indexed_button::IndexedButton;

#[function_component(ChangeUserRoleMolecule)]
pub fn change_user_rol_molecule () -> Html {
    let dni_state = use_state(|| 0);
    let cloned_dni_state = dni_state.clone();
    let dni_state_changed = Callback::from(move |dni:String|{
            cloned_dni_state.set(dni.parse::<u64>().unwrap());
    });
    let dni_state_cloned = dni_state.clone();

    let role_state = use_state(|| None);
    let cloned_role_state = role_state.clone();

    let informe = use_state(|| "".to_string());
    let informe_cloned = informe.clone();

    let clicks = use_state(|| 0);
    let clicks_cloned = clicks.clone();

    let search_user = Callback::from(move |()| {
        clicks_cloned.set(&*clicks_cloned + 1);
        let cloned_dni = dni_state_cloned.clone();
        let cloned_role_state = cloned_role_state.clone();
        let informe_cloned = informe_cloned.clone();
        {
            spawn_local(async move {
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
                                    informe_cloned.set(format!("El usuario con DNI {}, tiene rol {:?}.", *cloned_dni, user_role))
                                }else{
                                    log::error!("user not found (frontend)");
                                    informe_cloned.set(format!("El usuario con DNI {} no existe.", *cloned_dni))
                                }     
                            }
                            Err(error)=>{
                                log::error!("Error en deserializacion: {}", error);
                                informe_cloned.set("Ocurrio un error".to_string())
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

    let dni_state_cloned = dni_state.clone();
    let clicks_cloned = clicks.clone();

    let change_user_to_normal = Callback::from(move |()| {
        change_role (*dni_state_cloned.clone(), RolDeUsuario::Normal);
        clicks_cloned.set(0);
    });
    let clicks_cloned = clicks.clone();
    let dni_state_cloned = dni_state.clone();
    let change_user_to_employed = Callback::from(move |()| {
        change_role (*dni_state_cloned.clone(), RolDeUsuario::Empleado { sucursal: (0) });
        clicks_cloned.set(0);
    });
    let clicks_cloned = clicks.clone();
    let dni_state_cloned = dni_state.clone();
    let change_user_to_owner = Callback::from(move |()| {
        change_role (*dni_state_cloned.clone(), RolDeUsuario::Dueño);
        clicks_cloned.set(0);
    });
    let clicks_cloned = clicks.clone();
    let cancel_change = Callback::from(move |()| {
        clicks_cloned.set(0);
    });

    let clicks_cloned = clicks.clone();

    html!(
        <div class="change-user-role-box">
            <h2>{"Ingrese DNI del usuario al que desea cambiarle el Rol"}</h2>
            <DniInputField dni="DNI" label="DNI" tipo ="number" handle_on_change = {dni_state_changed} />
            <GenericButton text="Buscar Usuario" onclick_event={search_user}/>
            if *clicks_cloned != 0 {
                <h2>{&*informe}</h2>
                <h2>{"Seleccione que rol desea darle al usuario con DNI"}</h2>
                <GenericButton text="Cambiar a Normal" onclick_event={change_user_to_normal}/>
                <GenericButton text="Cambiar a Empleado" onclick_event={change_user_to_employed}/>
                <GenericButton text="Cambiar a Dueño" onclick_event={change_user_to_owner}/>
                <GenericButton text="Cancelar" onclick_event={cancel_change}/>
            }
        </div>
    )
}

fn change_role (dni: u64, rol: RolDeUsuario) {
    {
        spawn_local(async move {
            log::info!("entre al spawn local");
            let query = QueryChangeUserRole {dni: dni.clone(), new_role: rol.clone()};
            let respuesta = Request::post("/api/cambiar_rol_usuario")
                                                            .header("Content-Type", "application/json")
                                                            .body(serde_json::to_string(&query).unwrap())
                                                            .send()
                                                            .await;
            /*match respuesta{
                Ok(respuesta) =>{
                    let response:Result<ResponseGetUserRole, reqwasm::Error> = respuesta.json().await;
                    log::info!("deserailice la respuesta {:?}",response);
                    match response{
                        Ok(respuesta) => {
                            log::info!("{:?}", respuesta);
                            let string_respuesta = format!("Usuario con DNI {}, ahora tiene rol {:?}", dni, rol);
                            string_respuesta
                        }
                        Err(error)=>{
                            log::error!("Error en deserializacion: {}", error);
                            let string_respuesta = format!("Ha ocurrido un error");
                            string_respuesta
                        }
                    }
                }
                Err(error)=>{
                    let string_respuesta = format!("Ha ocurrido un error");
                    string_respuesta
                }
            }*/
        });
    }
}