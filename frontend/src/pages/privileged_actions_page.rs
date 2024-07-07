use yew_hooks::use_effect_once;
use yewdux::use_store;
use yew_router::{components::Link, hooks::use_navigator};
use yew::prelude::*;
use datos_comunes::{EstadoTrueque, QueryGetOffice, QueryGetUserRole, QueryTruequesFiltrados, ResponseGetOffice, ResponseGetUserRole, RolDeUsuario};
use crate::{request_post, router::Route, store::UserStore};
use reqwasm::http::Request;
use wasm_bindgen_futures::spawn_local;
use crate::components::generic_button::GenericButton;

#[function_component(PrivilegedActionsPage)]
pub fn privileged_actions_page() -> Html {
    let navigator = use_navigator().unwrap();

    let (store, _dispatch) = use_store::<UserStore>();
    let dni = store.dni;

    let role_state = use_state(|| None);

    let cloned_role_state = role_state.clone();
    use_effect_once( move || {
        if let Some(dni) = dni {
            let query = QueryGetUserRole { dni };
            request_post("/api/obtener_rol", query, move |r: ResponseGetUserRole| {
                cloned_role_state.set(Some(r.rol));
            });
        }
        || {}
    });

    //pusheo a buscar trueques definidos para un empleado de una sucursal
    let cloned_role_state = role_state.clone();
    let navigator_cloned = navigator.clone();
    let search_defined_trades_office = Callback::from(move |()| {
        let cloned_role_state = cloned_role_state.clone();
        if let RolDeUsuario::Empleado { sucursal } = (&*cloned_role_state).as_ref().unwrap().clone() {
            let query = QueryTruequesFiltrados {
                filtro_codigo_ofertante: None,
                filtro_codigo_receptor: None,
                filtro_dni_integrantes: None,
                filtro_estado: Some(EstadoTrueque::Definido),
                filtro_fecha_pactada: None,
                filtro_fecha_trueque: None,
                filtro_id_publicacion: None,
                filtro_sucursal: Some(sucursal),
            };

            let _ = navigator_cloned.push_with_query(&Route::SearchTrueques, &query);
        }

    });

    html! {
        <div class="privileged-actions-box">
            if (&*role_state).clone().is_some(){
                <h1 class="title">{"Acciones privilegiadas"}</h1> 
                {
                    match (&*role_state).clone().unwrap() {
                        RolDeUsuario::Normal => {
                            navigator.push(&Route::Home);
                            html!()
                        },
                        RolDeUsuario::Dueño => {html!{
                            <ul class="option-list">
                                <li><Link<Route> to={Route::ChangeUserRole}>{"Cambiar Rol de Usuario"}</Link<Route>></li>
                                <li><Link<Route> to={Route::CreateOffice}>{"Agregar Sucursal"}</Link<Route>></li>
                                <li><Link<Route> to={Route::DeleteOffice}>{"Eliminar Sucursal"}</Link<Route>></li>
                                <li><Link<Route> to={Route::UnlockAccount}>{"Desbloquear Cuenta"}</Link<Route>></li>
                                <li><Link<Route> to={Route::AwaitingPricePublication}>{"Ver Publicaciones Esperando Tasación"}</Link<Route>></li>
                                <li><Link<Route> to={Route::FinishTrade}>{"Concretar Trueque"}</Link<Route>></li>
                                <li><Link<Route> to={Route::DefinedTrades}>{"Trueques Definidos"}</Link<Route>></li>
                                <li><Link<Route> to={Route::Estadisticas}>{"Ver Estadísticas"}</Link<Route>></li>
                            </ul>
                        }},
                        RolDeUsuario::Empleado { sucursal : _ } => {html! {
                            <ul class="option-list">
                                <li><Link<Route> to={Route::AwaitingPricePublication}>{"Ver Publicaciones Esperando Tasación"}</Link<Route>></li>
                                <li><Link<Route> to={Route::FinishTrade}>{"Concretar Trueque"}</Link<Route>></li>
                                <li><GenericButton text="Trueques Definidos" onclick_event={search_defined_trades_office}/></li>
                                <li><Link<Route> to={Route::Estadisticas}>{"Ver Estadísticas"}</Link<Route>></li>
                            </ul>
                        }},
                    }            
                }
            }
        </div>
    }
}