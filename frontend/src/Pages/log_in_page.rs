use std::clone;

use gloo::console::log;
use yew::prelude::*;
use crate::Components::boton_log_in::LogInButton;
use crate::Components::text_input_login::LogInInputField;
use crate::Components::password_input_login::PasswordTextInput;
use crate::Molecules::log_in_molecule::LogInMolecule;



#[function_component(LogInPage)]
pub fn log_in_Page()-> Html{
    html!{
        <>
            <div>
                {"hola"}
            </div>

            <LogInMolecule />
        </>
    }

}