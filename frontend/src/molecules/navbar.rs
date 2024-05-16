use yew_router::hooks::use_navigator;
use yewdux::use_store;
use yew_router::prelude::Link;
use yew::prelude::*;
use crate::information_store::InformationStore;
use crate::store::UserStore;
use crate::router::Route;
use crate::components::indexed_button::IndexedButton;


#[function_component(Navbar)]
pub fn navbar() -> Html{

    let navigator = use_navigator().unwrap();
    
    let (store, dispatch) = use_store::<UserStore>();
    let dni = store.dni;
    let username = store.nombre.clone();

    let logout = Callback::from(move|_event| {
        dispatch.reduce_mut(|store| store.dni = None);
        navigator.push(&Route::Home);
    });
    
    
    let (information_store, information_dispatch) = use_store::<InformationStore>();
    let messages = information_store.messages.clone();


    
    let onclick = Callback::from(move |button_index:usize|{
        information_dispatch.reduce_mut(|store| store.messages.remove(button_index));
    });


    let (information_store, information_dispatch) = use_store::<InformationStore>();

    html!{
        <>
            <header class="navbar">
                <div class="logo">
                    <Link<Route> to={Route::Home}><img src="assets/img/Fedeteria_Solo_Logo.svg" alt="fedeteria"/></Link<Route>>
                </div>
                if dni.is_some(){
                    <div>
                        <h2>{"Hola " }{username}{"!"}</h2>
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
                        <h2>{"No tienes tu sesión iniciada." }</h2>
                    </div>
                    <nav>
                        <ul class="option_list">
                            <li><Link<Route> to={Route::LogInPage}>{"Iniciar Sesion"}</Link<Route>></li>
                        </ul>
                    </nav>
                }
            </header>
            if !messages.is_empty() {
                <div class="information-message-list">
                    {   
                        messages.iter().enumerate().map(move |(index, message)| html! {
                            <div class="information-message">
                                <h2>{ message.clone() }</h2>
                                <IndexedButton text="Cerrar mensaje" index={index.clone()} onclick_event={onclick.clone()}/>
                               // <button onclick={Callback::from(move |event:MouseEvent| {onclick(index)})}>{"Cerrar mensaje"}</button>
                            </div>
                            }).collect::<Html>()
                    }
                </div>
            }
        </>
    }
}