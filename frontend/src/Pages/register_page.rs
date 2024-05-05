use yew::prelude::*;

use crate::Molecules::register_molecule::RegisterMolecule;

#[function_component(RegisterPage)]
pub fn register_page()-> Html {
    html!{
        <>
            <RegisterMolecule/>
        </>
    }

}