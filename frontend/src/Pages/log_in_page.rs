use yew_router::prelude::Link;
use yew::prelude::*;
use crate::Molecules::log_in_molecule::LogInMolecule;
use crate::router::Route;

#[function_component(LogInPage)]
pub fn log_in_page()-> Html{
    html!{
        <>
            <LogInMolecule />
            <div>
                <span> {"¿No tienes usuario? "} </span> <Link<Route> to={Route::Register}>{"Registrate"}</Link<Route>>
            </div>
        </>
    }

}