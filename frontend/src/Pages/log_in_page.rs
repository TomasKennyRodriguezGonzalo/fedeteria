use yew::prelude::*;
use crate::Molecules::log_in_molecule::LogInMolecule;

#[function_component(LogInPage)]
pub fn log_in_page()-> Html{
    html!{
        <>
            <LogInMolecule />
            <div>
                <span> {"¿No tienes usuario? "} </span> <a href="/register" value="Redirect"> {"Create una cuenta."} </a>
            </div>
        </>
    }

}