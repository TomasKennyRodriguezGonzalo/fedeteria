use yew_router::prelude::Link;
use yew::prelude::*;
use yew_hooks::use_local_storage;
use crate::store::UserStore;
use crate::router::Route;

#[function_component(Navbar)]
pub fn navbar() -> Html{
    let auth = use_state(|| false);
    let auth_clone = auth.clone();
    
    let my_store = use_local_storage::<UserStore>("UserStore".to_string());
    let my_store_clone = my_store.clone();
    let mut username = "".to_string();


    use_effect(move || {
        // Acá agregar la funcionalidad que detecta el login y cambia la página desplegada
        
    });

    html!{
        <header class="navbar">
            <div class="logo">
                <Link<Route> to={Route::Home}><img src="assets/img/Fedeteria_Solo_Logo.svg" alt="fedeteria"/></Link<Route>>
            </div>
            if *auth{
                <div>
                    <h2>{"Hola " }{username}{"!"}</h2>
                </div>
                <nav>
                    <ul class="option_list">
                        <li><Link<Route> to={Route::MyPublications}>{"Mis publicaciones"}</Link<Route>></li>
                        <li><Link<Route> to={Route::Profile}>{"Mis publicaciones"}</Link<Route>></li>
                        <li><a>{"Cerrar Sesion"}</a></li>
                    </ul>
                </nav>
            } else {
                <div>
                    <h2>{"No estás loggeado." }</h2>
                </div>
                <nav>
                    <ul class="option_list">
                        <li><Link<Route> to={Route::LogInPage}>{"Iniciar Sesion"}</Link<Route>></li>
                    </ul>
                </nav>
            }
        </header>
    }
}