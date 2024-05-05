use std::default;
use std::ops::Deref;

use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use serde_json::json;
use yew::prelude::*;
use crate::store::UserStore;
use crate::Components::generic_button::GenericButton;
use crate::Components::generic_input_field::GenericInputField;
use wasm_bindgen_futures::spawn_local;
use crate::router::Route;
use yew_router::prelude::*;
use yewdux::prelude::*;

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

    let username_state = use_state(|| "no username yet".to_owned());
    let cloned_username_state = username_state.clone();
    let username_changed = Callback::from(move |username:String|{
            cloned_username_state.set(username.clone());
    });
    
    let password_state = use_state(|| "no password yet".to_owned());
    let cloned_password_state = password_state.clone();
    let password_changed = Callback::from(move |password|{
        cloned_password_state.set(password);
    });

    let login_response = use_state(|| "false".to_string());
    let login_response_c = login_response.clone();
    let cloned_username_state = username_state.clone();
    let cloned_password_state = password_state.clone();
    let navigator = use_navigator().unwrap();

    let (store, dispatch) = use_store::<UserStore>();
    let dispatch_cloned = dispatch.clone();

    let submit_clicked_example = Callback::from(move |()| {
        let login_response_c = login_response_c.clone();
        {
            let username = &*cloned_username_state;
            let password = &*cloned_password_state;
            {
                let username = username.clone();
                let password = password.clone();
             //   let navigator = navigator.clone();
                let dispatch_cloned = dispatch_cloned.clone();
                    spawn_local(async move {
                        let mut url = "/api/check_login".to_string();
                        let cloned_username = username.clone();
                        url += &format!("?username={cloned_username}&password={password}");
                        let resp = Request::get(&url).send().await.unwrap();
                        if resp.text().await.unwrap() == "true"{
                            login_response_c.set("true".to_string());
                            let dispatch_cloned = dispatch_cloned.clone();
                            dispatch_cloned.reduce_mut(|store|{
                                store.user = username;
                            });
                          //  navigator.push(&Route::Home);
                        } else{
                            login_response_c.set("false".to_string());
                        }
                        
                    })
            }
        }
    });

    /* 

    POSIBLE FUNCION DE RESPUESTA DE LOGIN

    let login_response = use_state(|| "false".to_string());
    let cloned_username_state = username_state.clone();
    let cloned_password_state = password_state.clone();
    let submit_clicked_real = call_backend_for_auth_response(cloned_username_state.clone(), cloned_password_state.clone(), login_response.clone());
    let submit_clicked_real = Callback::from(move |_| {
        let login_response_c = login_response_c.clone();
        {
            let username = &*cloned_username_state;
            let password = &*cloned_password_state;
            {
                let username = username.clone();
                let password = password.clone();
                    spawn_local(async move {
                        let mut url = "/api/check_login".to_string();
                        url += &format!("?username={username}&password={password}");
                        let resp:AuthResponse = Request::get(&url)
                        .send()
                        .await
                        .unwrap()
                        .json()
                        .await
                        .unwrap();

                    })
            }
        }
    });

     esta funcion retorna un usuario con un id unico, el nombre de usuario, la contraseña y el token


        let onsubmit = Callback::from(move |event:SubmitEvent|{
            event.prevent_default();
            let username_state = username_state.clone();
            let password_state = password_state.clone();
            spawn_local(async move {
                let result:AuthResponse = Request::post("http://localhost:8080/")
                .header("Content-Type", "application/json")
                .body(json!({
                    "username": *username_state.clone(),
                    "password": *password_state.clone()
                })
                .to_string(),
            )
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();

            });
        });
    */

    let onsubmit = Callback::from(move |event:SubmitEvent|{
        event.prevent_default();
    });

    let (store, dispatch) = use_store::<UserStore>();

    let username = store.user.clone();

    html! {
        <div class="login-box">
            <h1>{"Login"}</h1>
            <section>
                <div>
                    <form {onsubmit}>
                        <GenericInputField name = "username" label="Username" tipo = "text" handle_on_change = {username_changed} />
                        <GenericInputField name = "password" label="Password" tipo = "password" handle_on_change = {password_changed} />
                        <GenericButton text = "submit" onclick_event = {submit_clicked_example} />
                    </form>
                </div>
            </section>
        </div>
    }

}


