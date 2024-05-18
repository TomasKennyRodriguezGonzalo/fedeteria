use yew::prelude::*;
use yew_router::prelude::*;
use crate::pages::{privileged_actions_page::PrivilegedActionsPage, create_office_page::CreateOfficePage, create_publication_page::CreatePublicationPage, home_page::HomePage, log_in_page::LogInPage, profile_page::ProfilePage, publication_page::PublicationPage, register_page::RegisterPage, delete_office_page::DeleteOffice
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
    #[at("/perfil/publicaciones-guardadas")]
    SavedPublications,
    #[at("/perfil/visto-recientemente")]
    RecentlySeenPublications,
    #[at("/perfil/trueques-pendientes")]
    MyPendingTrades,
    #[at("/perfil/trueques-concretados")]
    MyCompletedTrades,
    #[at("/perfil/editar-informacion-personal")]
    EditPersonalInfo,
    #[at("/perfil/editar-preferencias")]
    EditPreferences,
    #[at("/publicacion:id")]
    Publication {id: String},
    #[at("/register")]
    Register,
    #[at("/acciones-privilegiadas")]
    PrivilegedActions,
    #[at("/acciones-privilegiadas/agregar-sucursal")]
    CreateOffice,
    #[at("/acciones-privilegiadas/eliminar-sucursal")]
    DeleteOffice,
    #[at("/acciones-priviligiadas/desbloquear-cuenta")]
    UnlockAccount,
    #[at("/acciones-privilegiadas/cambiar-rol")]
    ChangeUserRole,
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
        Route::EditPersonalInfo => html! {"Editar Informacion personal"},
        Route::Register => html! {<RegisterPage/>},
        Route::CreateOffice => html! { <CreateOfficePage/> },
        Route::CreatePublication => html! { <CreatePublicationPage/> },
        Route::DeleteOffice => html! {<DeleteOffice/>},
        Route::NotFound => html! { <h1>{"Error 404 página no existente!"}</h1>},
        Route::SavedPublications => html! {"Publicaciones guardadas"},
        Route::RecentlySeenPublications => html! {"Publicaciones vistas recientemente"},
        Route::MyPendingTrades => html! {"Trueques Pendientes"},
        Route::MyCompletedTrades => html! {"Trueques concretados"},
        Route::EditPreferences => html! {"Editar preferencias"}, 
        Route::PrivilegedActions => html! {<PrivilegedActionsPage/>},
        Route::UnlockAccount => html! {"Desbloquear Cuenta"},
        Route::ChangeUserRole => html! {"Cambiar Rol de Usuario"},
    }
}