use yew::prelude::*;
use yew_router::prelude::*;
use crate::Pages::{home_page::HomePage, 
    log_in_page::LogInPage, 
    register_page::RegisterPage, 
    create_publication_page::CreatePublicationPage,
    publication_page::PublicationPage,
    profile_page::ProfilePage
};


#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/login")]
    LogInPage,
    #[at("/mis-publicaciones")]
    MyPublications,
    #[at("/publicar")]
    CreatePublication,
    #[at("/perfil")]
    Profile,
    #[at("/publicacion:id")]
    Publication {id: String},
    #[at("/register")]
    Register,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <HomePage/> },
        Route::LogInPage => html! { <LogInPage /> },
        Route::MyPublications => html! { <p>{"MIS PUBLICACIONES!!"}</p> },
        Route::Publication { id } => html! { <PublicationPage id={id}/>},
        Route::Profile => html! { <ProfilePage/> },
        Route::Register => html! {<RegisterPage/>},
        Route::CreatePublication => html! { <CreatePublicationPage/> },
        Route::NotFound => html! { <h1>{":( Error 404 página no existente!"}</h1>}, 
    }
}