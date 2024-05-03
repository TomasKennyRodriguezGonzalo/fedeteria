use yew::prelude::*;

use crate::Molecules::registrar_molecule::RegistrarMolecule;

#[function_component(RegistrarPage)]
pub fn log_in_page()-> Html{
    html!{
        <>
            <RegistrarMolecule />
        </>
    }

}