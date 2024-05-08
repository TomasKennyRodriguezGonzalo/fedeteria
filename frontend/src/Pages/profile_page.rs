use yew::prelude::*;
use yewdux::prelude::*;
use crate::store::UserStore;

#[function_component(ProfilePage)]
pub fn profile_page() -> Html {

    let (store, _dispatch) = use_store::<UserStore>();
    let dni = store.dni.clone();

    let (store, _dispatch) = use_store::<UserStore>();
    let username = store.nombre.clone();


    html! (
        <>
            <h1>{"PERFIL"}</h1>
            if dni.is_some(){
                <div>{"Hola "} {username.clone()}{" !"}</div>
                <div>{"listo para realizar unos trueques?"}</div>
            }
        </>
    )
}