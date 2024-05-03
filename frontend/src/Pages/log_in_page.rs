use std::clone;

use gloo::console::log;
use yew::prelude::*;
use crate::Components::generic_button::LogInButton;
use crate::Components::generic_text_input::GenericInputField;
use crate::Molecules::log_in_molecule::LogInMolecule;



#[function_component(LogInPage)]
pub fn log_in_page()-> Html{
    html!{
        <>
            <div>
                {"hola"}
            </div>

            <LogInMolecule />
        </>
    }

}