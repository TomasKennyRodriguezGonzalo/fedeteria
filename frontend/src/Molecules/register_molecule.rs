use yew::prelude::*;


#[function_component(RegisterMolecule)]
pub fn register_molecule()-> Html{
    html! {
        <>
            <h1> {"Registrarse"} </h1>
            <span> {"¿Ya tienes usuario? "} </span> <a href="/login" value="Redirect"> {"Ingresa a tu cuenta."} </a>
        </>
    }
}