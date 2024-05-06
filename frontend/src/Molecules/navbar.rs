use yew_router::hooks::use_navigator;
use yew_router::navigator;
use yewdux::use_store;
use yew_router::prelude::Link;
use yew::prelude::*;
use crate::store::UserStore;
use crate::router::Route;


#[function_component(Navbar)]
pub fn navbar() -> Html{
    let auth = use_state(|| false);
    let auth_clone = auth.clone();

    let navigator = use_navigator().unwrap();
    
    let (store, dispatch) = use_store::<UserStore>();
    //let mut username = store.user.clone();
    let mut dni = store.dni.clone();

    let logout = Callback::from(move|_event| {
        dispatch.reduce_mut(|store| store.dni = None);
        navigator.push(&Route::Home);
    });

    html!{
        <header class="navbar">
            <div class="logo">
                <Link<Route> to={Route::Home}><img src="assets/img/Fedeteria_Solo_Logo.svg" alt="fedeteria"/></Link<Route>>
            </div>
            if !dni.is_none(){
                <div>
               //     <h2>{"Hola " }{username}{"!"} {"tu dni es: "} {dni.unwrap()}</h2>
                </div>
                <nav>
                    <ul class="option_list">
                        <li><Link<Route> to={Route::MyPublications}>{"Mis publicaciones"}</Link<Route>></li>
                        <li><Link<Route> to={Route::Profile}>{"Perfil"}</Link<Route>></li>
                        <li><a onclick={logout}>{"Cerrar Sesion"}</a></li>
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