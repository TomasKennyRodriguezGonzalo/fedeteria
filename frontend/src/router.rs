use yew::prelude::*;
use yew_router::prelude::*;
use crate::Pages::{log_in_page::LogInPage, home::Home};
use crate::Components::button_storage::Prueba;


#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/hello-server")]
    HelloServer,
    #[at("/login-page")]
    LogInPage,
    #[at("/mis-publicaciones")]
    MyPublications,
    #[at("/perfil")]
    Profile,
<<<<<<< HEAD
    #[at("/boton")]
    Button,
=======
>>>>>>> 7c0ba59a9f1606f5fa8c40d762ca8cad1fc4d335
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home/> },
        Route::HelloServer => html! { <p>{"Hello Server"}</p> },
        Route::LogInPage => html! { <LogInPage /> },
        Route::MyPublications => html! { <p>{"MIS PUBLICACIONES!!"}</p> },
        Route::Profile => html! { <p>{"MI PERFIL"}</p> },
<<<<<<< HEAD
        Route::Button => html! { <Prueba/> },
=======
>>>>>>> 7c0ba59a9f1606f5fa8c40d762ca8cad1fc4d335
        Route::NotFound => html! { <h1>{"Error 404 not found!"}</h1>}, 
    }
}