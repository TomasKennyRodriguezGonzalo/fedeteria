use yew::prelude::*;
use yew_router::prelude::*;
use crate::pages:: {change_user_rol_page::ChangeUserRolePage, create_office_page::CreateOfficePage, create_publication_page::CreatePublicationPage, delete_office_page::DeleteOffice, edit_personal_info_page::EditPersonalInfoPage, home_page::HomePage, log_in_page::LogInPage, privileged_actions_page::PrivilegedActionsPage, profile_page::ProfilePage, publication_page::PublicationPage, register_page::RegisterPage, unlock_account_page::UnlockAccountPage,
    privileged_actions_page::PrivilegedActionsPage
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
    }
}