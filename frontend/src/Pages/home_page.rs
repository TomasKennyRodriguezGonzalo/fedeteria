use yew_router::prelude::Link;
use yew::prelude::*;
use yew_hooks::use_local_storage;
use crate::router::Route::{self};

#[function_component(HomePage)]
pub fn home_page() -> Html {

    html!{
        <div class="home-page">
            <div class= "completed-trades">
                <h2>{"Próximamente..."}</h2>
            </div>
            <div class= "publication-list">
                <Link<Route> to={Route::CreatePublication}>{"Publicar"}</Link<Route>>
            </div>
        </div>
    }
}