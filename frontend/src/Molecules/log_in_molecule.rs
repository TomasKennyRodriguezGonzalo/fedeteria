use std::default;
use std::ops::Deref;

use datos_comunes::{QueryObtenerUsuario, ResponseObtenerUsuario};
use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use serde_json::json;
use yew::prelude::*;
use crate::store::{UserStore};
use crate::Components::generic_button::GenericButton;
use crate::Components::generic_input_field::GenericInputField;
use crate::Components::dni_input_field::DniInputField;
use wasm_bindgen_futures::spawn_local;
use crate::router::Route;
use yew_router::prelude::*;
use yewdux::{
    log::{log, Level},
    prelude::*, Context,
};


#[derive(Default)]
pub struct State{
    pub username:String,
    pub password:String,
}

#[derive(Serialize,Deserialize)]
pub struct AuthResponse{
    pub data:User
}

#[derive(Serialize,Deserialize)]
pub struct User{
    pub id:u32,
    pub username:String,
    pub token:String,
}

#[function_component(LogInMolecule)]
pub fn log_in_molecule()-> Html{

    let state = use_state(State::default);

    let dni_state:UseStateHandle<u64> = use_state(|| 0);
    let cloned_dni_state = dni_state.clone();
    let dni_changed = Callback::from(move |dni:String|{
            cloned_dni_state.set(dni.parse::<u64>().unwrap());
    });
    
    let password_state = use_state(|| "no password yet".to_owned());
    let cloned_password_state = password_state.clone();
    let password_changed = Callback::from(move |password|{
        cloned_password_state.set(password);
    });

    let login_response = use_state(|| "false".to_string());
    let login_response_c = login_response.clone();
    let cloned_dni_state = dni_state.clone();
    let cloned_password_state = password_state.clone();
    let navigator = use_navigator().unwrap();

    let (store, dispatch) = use_store::<UserStore>();
    let dispatch_cloned = dispatch.clone();




    let dispatch_cloned = dispatch.clone();


    let submit_clicked_example = Callback::from(move |()| {

        let dispatch_cloned = dispatch_cloned.clone();
        let login_response_c = login_response_c.clone();
        {
            let dni = &*cloned_dni_state;
            let password = &*cloned_password_state;
            {
                let dni = dni.clone();
                let password = password.clone();
                let navigator = navigator.clone();
                let dispatch_cloned = dispatch_cloned.clone();
                spawn_local(async move {
                        let mut url = "/api/check_login".to_string();
                        let cloned_dni = dni.clone();
                        url += &format!("?dni={cloned_dni}&password={password}");
                        let resp = Request::get(&url).send().await.unwrap();
                        if resp.text().await.unwrap() == "true"{
                            login_response_c.set("true".to_string());
                            let dispatch_cloned = dispatch_cloned.clone();
                            dispatch_cloned.reduce_mut(|store|{
                                store.dni = Some(dni);
                                store.login_fail = false;
                                store.login_faliures=0;
                            });
                            navigator.push(&Route::Home);
                        } else{
                            dispatch_cloned.reduce_mut(|store|{
                                store.login_fail = true;
                                store.login_faliures+=1;
                            });
                        }
                        
                    })     
            }
        }
    });


    let onsubmit = Callback::from(move |event:SubmitEvent|{
        event.prevent_default();
    });


    let (store, dispatch) = use_store::<UserStore>();

    let intentos_restantes = 3 - store.clone().login_faliures;

    html! {
        <div class="login-box">
            <h1>{"Login"}</h1>
            <section>
                <div>
                    <form {onsubmit}>
                        if store.login_fail == true{
                            <div>
                                {"Error al iniciar sesion, datos incorrectos"}
                            </div>
                            <div>
                                {"intentos restantes: "} {intentos_restantes}
                            </div>
                        }
                        <DniInputField dni = "dni" label="Dni" tipo = "number" handle_on_change = {dni_changed} />
                        <GenericInputField name = "password" label="Password" tipo = "password" handle_on_change = {password_changed} />
                        <GenericButton text = "submit" onclick_event = {submit_clicked_example} />
                    </form>
                </div>
            </section>
        </div>
    }

}

