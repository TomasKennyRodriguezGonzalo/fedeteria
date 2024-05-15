use yew_router::prelude::Link;
use yew::prelude::*;

use crate::{router::Route, molecules::register_molecule::RegisterMolecule};

#[function_component(RegisterPage)]
pub fn register_page()-> Html {
    html!{
        <>
            <RegisterMolecule/>
            <span> {"¿Ya tienes usuario? "} </span>
            <Link<Route> to={Route::LogInPage}>{"Iniciar Sesion"}</Link<Route>>
        </>
    }

}