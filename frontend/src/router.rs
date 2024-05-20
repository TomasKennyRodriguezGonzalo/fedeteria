use crate::pages::search_results_page::SearchResultsPage;
use crate::pages::unlock_account_page::UnlockAccountPage;
use crate::pages::change_user_rol_page::ChangeUserRolePage;
use datos_comunes::QueryPublicacionesFiltradas;
use yew::prelude::*;
use yew_router::prelude::*;
use crate::pages::{create_office_page::CreateOfficePage,
    create_publication_page::CreatePublicationPage,
    home_page::HomePage, log_in_page::LogInPage,
    profile_page::ProfilePage, 
    publication_page::PublicationPage, 
    register_page::RegisterPage, 
    delete_office_page::DeleteOffice, 
    edit_personal_info_page::EditPersonalInfoPage,
    privileged_actions_page::PrivilegedActionsPage,
    my_publications_page::MyPublicationsPage,
};

use crate::components::publication_thumbnail::PublicationThumbnail;
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
    #[at("/publicacion/:id")]
    Publication {id: usize},
    #[at("/register")]
    Register,
    #[at("/acciones-privilegiadas")]
    PrivilegedActions,
    #[at("/acciones-privilegiadas/agregar-sucursal")]
    CreateOffice,
    #[at("/acciones-privilegiadas/eliminar-sucursal")]
    DeleteOffice,
    #[at("/test/publication-thumbnail/:id")]
    PublicationThumbnail {id: usize},
    #[at("/acciones-privilegiadas/desbloquear-cuenta")]
    UnlockAccount,
    #[at("/acciones-privilegiadas/cambiar-rol")]
    ChangeUserRole,
    #[at("/resultados-busqueda")]
    SearchResults,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <HomePage/> },
        Route::LogInPage => html! { <LogInPage /> },
        Route::MyPublications => html! { < MyPublicationsPage /> },
        Route::Publication { id } => html! { <PublicationPage id={id}/>},
        Route::Profile => html! { <ProfilePage/> },
        Route::EditPersonalInfo => html! {<EditPersonalInfoPage/>},
        Route::Register => html! {<RegisterPage/>},
        Route::CreateOffice => html! { <CreateOfficePage/> },
        Route::CreatePublication => html! { <CreatePublicationPage/> },
        Route::DeleteOffice => html! {<DeleteOffice/>},
        Route::NotFound => html! { <h1>{"Error 404 página no existente!"}</h1>},
        Route::SavedPublications => html! {"Publicaciones guardadas"},
        Route::RecentlySeenPublications => html! {"Publicaciones vistas recientemente"},
        Route::MyPendingTrades => html! {"Trueques Pendientes"},
        Route::MyCompletedTrades => html! {"Trueques concretados"},
        Route::PrivilegedActions => html! {<PrivilegedActionsPage/>},
        Route::ChangeUserRole => html! {<ChangeUserRolePage/>},
        Route::UnlockAccount => html!(<UnlockAccountPage/>), 
        Route::PublicationThumbnail {id} => html! {<PublicationThumbnail id={id}/>},
        Route::SearchResults => html!(<SearchResultsPage/>),
    }
}