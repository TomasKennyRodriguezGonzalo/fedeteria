use yew_router::prelude::Link;
use yew::prelude::*;
use crate::molecules::log_in_molecule::LogInMolecule;
use crate::router::Route;

#[function_component(LogInPage)]
pub fn log_in_page()-> Html{
    html!{
        <>
            <LogInMolecule />
            <div>
                <span> {"¿No tienes usuario? "} </span> <Link<Route> to={Route::Register}>{"Registrate"}</Link<Route>>
                <span> {"¿Olvidaste tu contraseña? "} </span> <Link<Route> to={Route::SendCodeToChangePassword}>{"Recuperar Contraseña"}</Link<Route>>
                <span> {"¿Ya tienes el código para recuperar tu contraseña? "} </span> <Link<Route> to={Route::ChangePasswordFromLogIn}>{"Cambiar Contraseña"}</Link<Route>>
            </div>
        </>
    }

}