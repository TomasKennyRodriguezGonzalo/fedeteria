use yew_router::prelude::Link;
use yew::prelude::*;
use yew_hooks::use_local_storage;
use crate::router::Route::{self};

#[function_component(HomePage)]
pub fn home_page() -> Html {
    let auth = use_state(|| false);
    let auth_clone = auth.clone();
    use_effect(move || {
        // Acá agregar la funcionalidad que detecta el login y cambia la página desplegada
    });

    html!{
        <div class="home-page">
            <div class= "completed-trades">
                <h2>{"Próximamente..."}</h2>
            </div>
            <div class= "publication-list">
                if *auth {
                    <Link<Route> to={Route::CreatePublication}>{"Publicar"}</Link<Route>>
                } else {
                    <Link<Route> to={Route::LogInPage}>{"Publicar"}</Link<Route>>
                }
            </div>
        </div>
    }
}