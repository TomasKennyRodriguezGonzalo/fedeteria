use crate::pages::{notifications_page::NotificationsPage, trueque_page::TruequePage};
use crate::pages::search_results_page::SearchResultsPage;
use crate::pages::unlock_account_page::UnlockAccountPage;
use crate::pages::change_user_rol_page::ChangeUserRolePage;
use crate::request_post;
use crate::store::UserStore;
use datos_comunes::{QueryGetUserRole, ResponseGetUserRole};
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::use_store;
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
    awaiting_price_publication::AwaitingPricePublicationPage,
    publication_trade_offers_page::PublicationTradeOffersPage,
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
    #[at("/publicacion/:id")]
    Publication {id: usize},
    #[at("/trueque/:id")]
    Trueque {id: usize},
    #[at("/register")]
    Register,
    #[at("/acciones-privilegiadas")]
    PrivilegedActions,
    #[at("/acciones-privilegiadas/agregar-sucursal")]
    CreateOffice,
    #[at("/acciones-privilegiadas/eliminar-sucursal")]
    DeleteOffice,
    #[at("/acciones-privilegiadas/desbloquear-cuenta")]
    UnlockAccount,
    #[at("/acciones-privilegiadas/cambiar-rol")]
    ChangeUserRole,
    #[at("/acciones-privilegiadas/esperando-tasacion")]
    AwaitingPricePublication,
    #[at("/resultados-busqueda")]
    SearchResults,
    #[at("/notificaciones")]
    Notifications,
    #[at("/ofertas-recibidas")]
    PublicationTradeOffers,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    html! {<>
            <RouteCheckPage route={routes.clone()}/>
            {match routes {
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
                Route::SearchResults => html!(<SearchResultsPage/>),
                Route::Notifications => html!(<NotificationsPage/>),
                Route::AwaitingPricePublication => html!(<AwaitingPricePublicationPage/>),
                Route::PublicationTradeOffers => html!(<PublicationTradeOffersPage/>),
                Route::Trueque { id } => html! { <TruequePage id={id}/>},
            }}
    </>}
}


#[derive(Properties,PartialEq)]
pub struct RouteCheckPageProps {
    route: Route,
}

#[function_component(RouteCheckPage)]
pub fn privileged_actions_page(props: &RouteCheckPageProps) -> Html {
    
    let (store, _dispatch) = use_store::<UserStore>();
    let route = props.route.clone();
    
    // [sin login, normal, empleado, dueño]
    let allowed_roles = match route {
        Route::Home => [true, true, true, true],
        Route::LogInPage => [true, false, false, false],
        Route::MyPublications => [false, true, true, true],
        Route::CreatePublication => [false, true, true, true],
        Route::Profile => [false, true, true, true],
        Route::SavedPublications => [false, true, true, true],
        Route::RecentlySeenPublications => [false, true, true, true],
        Route::MyPendingTrades => [false, true, true, true],
        Route::MyCompletedTrades => [false, true, true, true],
        Route::EditPersonalInfo => [false, true, true, true],
        // No registrado "puede acceder" pero la propia página lo redirige a la página de login con un mensaje especial
        // estaría bueno que todo se maneje acá de alguna manera, pero complicaría esta tabla
        // o, de alguna manera obligar a cada página a que implemente sus propias reglas de permisos
        Route::Publication { id : _ } => [true, true, true, true],
        Route::Register => [true, false, false, false],
        Route::PrivilegedActions => [false, false, true, true],
        Route::CreateOffice => [false, false, false, true],
        Route::DeleteOffice => [false, false, false, true],
        Route::UnlockAccount => [false, false, false, true],
        Route::ChangeUserRole => [false, false, false, true],
        Route::SearchResults => [true, true, true, true],
        Route::NotFound => [true, true, true, true],
        Route::Notifications => [false, true, true, true],
        Route::AwaitingPricePublication => [false, false, true, true],
        Route::PublicationTradeOffers => [false, true, true, true],
        Route::Trueque { id : _ } => [false, true, true, true],
    };
    let navigator = use_navigator().unwrap();
    use_effect(move || {
        if let Some(dni) = store.dni {
            let query = QueryGetUserRole {dni};
            request_post("/api/obtener_rol",
                query, move |response: ResponseGetUserRole| {
                    let rol = response.rol;
                    let num = match rol {
                        datos_comunes::RolDeUsuario::Normal => 1,
                        datos_comunes::RolDeUsuario::Empleado { sucursal : _ } => 2,
                        datos_comunes::RolDeUsuario::Dueño => 3,
                    };
                    if !allowed_roles[num] {
                        navigator.push(&Route::Home);
                    }
            });
        } else {
            // usuario no login
            if !allowed_roles[0] {
                navigator.push(&Route::Home);
            }
        }
        || {}
    });
    html! {
        <> </>
    }
}