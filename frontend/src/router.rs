use yew::prelude::*;
use yew_router::prelude::*;
use crate::pages::{create_office_page::CreateOfficePage, create_publication_page::CreatePublicationPage, home_page::HomePage, log_in_page::LogInPage, profile_page::ProfilePage, publication_page::PublicationPage, register_page::RegisterPage, delete_office_page::DeleteOffice
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
    #[at("/agregar-sucursal")]
    CreateOffice,
    #[at("/publicacion:id")]
    Publication {id: String},
    #[at("/register")]
    Register,
    #[at("/delete-office")]
    DeleteOffice,
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
        Route::CreateOffice => html! { <CreateOfficePage/> },
        Route::CreatePublication => html! { <CreatePublicationPage/> },
        Route::DeleteOffice => html! {<DeleteOffice/>},
        Route::NotFound => html! { <h1>{":( Error 404 página no existente!"}</h1>}, 
    }
}