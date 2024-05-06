use yewdux::use_store;
use crate::store::UserStore;
use yew_router::prelude::Link;
use yew::prelude::*;
use yew_hooks::use_local_storage;
use crate::router::Route::{self};

#[function_component(HomePage)]
pub fn home_page() -> Html {
    let auth = use_state(|| false);
    let auth_clone = auth.clone();
    
    let (store, dispatch) = use_store::<UserStore>();
    let mut dni = store.dni.clone();
    
    html!{
        <div class="home-page">
            <div class= "completed-trades">
                <h2 class="title">{"Próximamente..."}</h2>
            </div>
            <div class= "publication-list">
                <h1 class="title">{"Publicaciones..."}</h1>
                if !dni.is_none() {
                    <Link<Route> to={Route::CreatePublication}>{"Publicar"}</Link<Route>>
                } else {
                    <Link<Route> to={Route::LogInPage}>{"Publicar"}</Link<Route>>
                }
            </div>
        </div>
    }
}