use yew_router::prelude::Link;
use yew::prelude::*;
use yew_hooks::use_local_storage;
use crate::store::UserStore;
use crate::router::Route;

#[function_component(Navbar)]
pub fn navbar() -> Html{
    let my_store = use_local_storage::<UserStore>("UserStore".to_string());
    let mut username = "".to_string();
    if !my_store.as_ref().is_none(){
        let user_store = my_store.as_ref().unwrap();
        username = user_store.user.clone();
    }

    html!{
        <header class="navbar">
            <div class="logo">
                <Link<Route> to={Route::Home}><img src="assets/img/Fedeteria_Solo_Logo.svg" alt="fedeteria"/></Link<Route>>
            </div>
            if !username.is_empty(){
                <div>
                    <h2>{"Estás loggeado como: " }{username}</h2>
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