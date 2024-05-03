use yew::prelude::*;
use yew_router::prelude::*;
use crate::Pages::{home::Home, log_in_page::LogInPage, register_page::RegisterPage, create_publication_page::CreatePublicationPage};


#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/hello-server")]
    HelloServer,
    #[at("/login")]
    LogInPage,
    #[at("/mis-publicaciones")]
    MyPublications,
    #[at("/publicar")]
    CreatePublication,
    #[at("/perfil")]
    Profile,
    #[at("/registrar")]
    Register,
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
        Route::Register => html! {<RegisterPage/>},
        Route::CreatePublication => html! { <CreatePublicationPage/> },
        Route::NotFound => html! { <h1>{"Error 404 not found!"}</h1>}, 
    }
}