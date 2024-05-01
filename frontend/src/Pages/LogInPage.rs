use std::clone;

use gloo::console::log;
use yew::prelude::*;
use crate::Components::botonLogIn::LogInButton;
use crate::Components::textInputLogIn::LogInInputField;
use crate::Components::passwordTextInput::PasswordTextInput;


#[function_component(LogInPage)]
pub fn log_in_button()-> Html{
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