use std::clone;

use gloo::console::log;
use yew::prelude::*;
use crate::Components::boton_log_in::LogInButton;
use crate::Components::text_input_login::LogInInputField;
use crate::Components::password_input_login::PasswordTextInput;
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;



#[function_component(LogInMolecule)]
pub fn log_in_molecule()-> Html{
    let username_state = use_state(|| "no username yet".to_owned());
    let cloned_username_state = username_state.clone();
    let username_changed = Callback::from(move |username|{
        cloned_username_state.set(username);
    });

    
    let password_state = use_state(|| "no password yet".to_owned());
    let cloned_password_state = password_state.clone();
    let password_changed = Callback::from(move |password|{
        cloned_password_state.set(password);
    });

    let button_clicked = use_state(|| {
        let username = &*username_state;
        let password = &*password_state;
        {
            let username = username.clone();
            let password = password.clone();
            use_effect(move || {
                spawn_local(async move {
                    let resp = Request::get("/api/check_login").send().await.unwrap();
                    if resp.text().await.unwrap() == "true"{
                        html!{<div>{"login passed"}</div>}
                    } else{
                        html!{<div>{"login failed"}</div>}
                    }

                })
            })
        }
    });


    html! {
        <>
            <form>
                <LogInInputField name = "username" handle_on_change = {username_changed} />
                <PasswordTextInput name = "password" handle_on_change = {password_changed} />
                <LogInButton text = "submit" />
            </form>
            <p>{"your username is:"} {&*username_state}</p>
            <p>{"your password is:"} {&*password_state}</p>
        </>
    }

}