use yew::prelude::*;
use crate::Components::generic_button::LogInButton;
use crate::Components::generic_text_input::GenericInputField;
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

    let login_response = use_state(|| "false".to_string());
    let login_response_c = login_response.clone();
    let cloned_username_state = username_state.clone();
    let cloned_password_state = password_state.clone();
    let submit_clicked = Callback::from(move |_| {
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
                        let resp = Request::get(&url).send().await.unwrap();
                        if resp.text().await.unwrap() == "true"{
                            login_response_c.set("true".to_string());
                        } else{
                            login_response_c.set("false".to_string());
                        }
                        
                    })
            }
        }
    });

    let onsubmit = Callback::from(|event:SubmitEvent|{
        event.prevent_default();
    });

    html! {
        <div class="login-box">
            <h1>{"Login"}</h1>
            <section>
                <div>
                    <form {onsubmit}>
                        <GenericInputField name = "username" label="Username" tipo = "text" handle_on_change = {username_changed} />
                        <GenericInputField name = "password" label="Password" tipo = "password" handle_on_change = {password_changed} />
                        <LogInButton text = "submit" onclick_event = {submit_clicked} />
                    </form>
                </div>
            </section>
            <p>{"your username is:"} {&*username_state}</p>
            <p>{"your password is:"} {&*password_state}</p>
            <p>{"login response:"} {&*login_response} </p>
        </div>
    }

}