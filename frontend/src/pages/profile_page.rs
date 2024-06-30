use serde::{Deserialize, Serialize};
use yew::prelude::*;
use yew_hooks::use_effect_once;
use yew_router::hooks::{use_location, use_navigator};
use yewdux::prelude::*;
extern crate chrono;
use chrono::prelude::*;
use crate::{request_post, router::Route, store::UserStore};
use datos_comunes::{self, QueryGetUserInfo, QueryPublicacionesFiltradas, ResponseGetUserInfo};
use yew_router::prelude::Link;

#[derive(Clone, Default)]
pub struct DatosDeUsuario {
    pub nombre_completo: String,
    pub email: String,
    pub nacimiento: DateTime<Local>,
    pub puntos: i64,
}

impl DatosDeUsuario {
    pub fn new(nombre_completo: String, email: String, nacimiento: DateTime<Local>, puntos: i64) -> DatosDeUsuario {
        DatosDeUsuario {nombre_completo, email, nacimiento, puntos}
    }
}


// navigator_cloned.push_with_query(&Route::ProfilePage, &dni_query);
#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct DniURLQuery {
    pub dni: u64,
}

#[function_component(ProfilePage)]
pub fn profile_page() -> Html {

    let location = use_location().unwrap();
    let (store, _dispatch) = use_store::<UserStore>();
    let dni = store.dni;


    let navigator = use_navigator().unwrap();
    use_effect(move || {
        if dni.is_none() {
            navigator.push(&Route::LogInPage)
        }
    });

    let dni = match dni {
        Some(dni) => dni,
        None => {return html!(<>{"Error!"}</>);}
    };

    // query hecha con /profile?dni=algo
    let dni_query = location.query::<DniURLQuery>().ok().map(|d| d.dni);
    // true si estoy viendo mi propio perfil
    let informacion_propia = dni_query.is_none() || (dni == dni_query.unwrap());
    let dni_query = dni_query.unwrap_or(dni);

    let user_state: UseStateHandle<DatosDeUsuario> = use_state(|| Default::default());
    let cloned_user_state = user_state.clone();

    
    use_effect_once(move || {
        let cloned_user_state = cloned_user_state.clone();
        let query = QueryGetUserInfo { dni : dni_query };
        request_post("/api/get_user_info", query, move |respuesta:Option<ResponseGetUserInfo>| {
            
            if let Some(respuesta) = respuesta {
                let user_info = DatosDeUsuario::new(
                    respuesta.nombre_y_ap,
                    respuesta.email,
                    respuesta.nacimiento,
                    respuesta.puntos);
                cloned_user_state.set(user_info);
            } else {
                log::error!("Usuario no encontrado!");
            }
        });

        ||{}
    });
        
    
    html! (
        <div class="profile-page-box">
            <div class="profile-information-box">
                <h1 class="title"> {
                    if informacion_propia {"Tu información"} else {"Información"}
                }</h1>
                <h2 class="information-text">{"Nombre y apellido: "} {&*user_state.nombre_completo}</h2>
                <h2 class="information-text">{"Email: "} {&*user_state.email}</h2>
                <h2 class="information-text">{"DNI: "} {dni_query}</h2>
                <h2 class="information-text">{"Fecha de nacimiento: "} {user_state.nacimiento.format("%Y-%m-%d")}</h2>
                if informacion_propia {
                    <h2 class="information-text">{"Puntos: "} {user_state.puntos}</h2>
                }
            </div>
        <div class="profile-actions-box">
            <h1 class="title">{"Acciones"}</h1>
            <ul>
                // <li><Link<Route> to={Route::SavedPublications}>{"Articulos Guardados"}</Link<Route>></li>
                //  <li><Link<Route> to={Route::RecentlySeenPublications}>{"Vistos Recientemente"}</Link<Route>></li>
                if informacion_propia {
                    <li><Link<Route> to={Route::MyPublications}>{"Tus Publicaciones"}</Link<Route>></li>
                } else {
                    <li><Link<Route, QueryPublicacionesFiltradas> to={Route::SearchResults} query={
                        QueryPublicacionesFiltradas {
                            filtro_dni: Some(dni_query),
                            ..Default::default()
                        }
                    }>{"Ver Publicaciones"}</Link<Route, QueryPublicacionesFiltradas>></li>
                }
                if informacion_propia {
                    <li><Link<Route> to={Route::MyTradesOffers}>{"Ofertas de Trueque"}</Link<Route>></li>
                    <li><Link<Route> to={Route::MyPendingTrades}>{"Trueques Pendientes"}</Link<Route>></li>
                    <li><Link<Route> to={Route::MyDefinedTrades}>{"Trueques Definidos"}</Link<Route>></li>
                    <li><Link<Route> to={Route::MyCompletedTrades}>{"Trueques Concretados"}</Link<Route>></li>
                    <li><Link<Route> to={Route::EditPersonalInfo}>{"Editar Información Personal"}</Link<Route>></li>
                    <li><Link<Route> to={Route::SavedPublications}>{"Publicaciones Guardadas"}</Link<Route>></li>
                }
            </ul>
        </div>
    </div>
    )
}