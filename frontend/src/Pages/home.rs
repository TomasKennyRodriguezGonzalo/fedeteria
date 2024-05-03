use yew::prelude::*;
use yew_hooks::use_local_storage;
use crate::store::UserStore;

#[function_component(Home)]
pub fn home() -> Html {

    let my_store = use_local_storage::<UserStore>("UserStore".to_string());
    let mut username = "no estas logeado pa".to_string();
    if !my_store.as_ref().is_none(){
        let user_store = my_store.as_ref().unwrap();
        username = user_store.user.clone();
    }

    html!{
        <>
            <h1>{"ESTO ES EL HOME"}</h1>
            <div>{"tu username es: " }</div>
            <div>{username}</div>
        </>
    }
}