use yew::prelude::*;
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