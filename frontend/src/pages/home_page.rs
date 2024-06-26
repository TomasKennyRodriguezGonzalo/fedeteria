use yewdux::use_store;
use crate::{molecules::trade_grid_molecule::TradeGridMolecule, store::UserStore};
use yew_router::prelude::Link;
use yew::prelude::*;
use crate::router::Route::{self};
use wasm_bindgen_futures::spawn_local;
use datos_comunes::{QueryObtenerUsuario, ResponseObtenerUsuario};
use reqwasm::http::Request;
use crate::molecules::publication_grid_molecule::PublicationGridMolecule;

#[function_component(HomePage)]
pub fn home_page() -> Html {
    //TRAIGO DEL BACKEND LOS DATOS DEL USUARIO

    let (store, dispatch) = use_store::<UserStore>();
    let dni = store.dni;
    use_effect( move ||{
        let dispatch = dispatch.clone();
        if let Some(dni) = dni{
            spawn_local(async move {
                let query = QueryObtenerUsuario{dni};
                let dispatch = dispatch.clone();
                let respuesta = Request::post("/api/retornar_usuario").header("Content-Type", "application/json").body(serde_json::to_string(&query).unwrap()).send().await;
                match respuesta {
                    Ok(resp) => {
                        let response:Result<Option<ResponseObtenerUsuario>, reqwasm::Error> = resp.json().await;
                        match response{
                            Ok(resp) => {
                                if resp.is_some(){
                                    let username = resp.unwrap().nombre;
                                    dispatch.reduce_mut(|store|{
                                    store.nombre = username;
                                    });
                                } else{
                                    log::error!("username not found "); 
                                }

                            }
                            Err(error) => {
                                log::error!("Error en la deserializacion: {}",error); 
                            }
                        }
                    }
                    Err(error) => {
                        log::error!("Error en la respuesta del back: {}",error);
                    }
                };
            });
        } 
    });
    
    let (store, _dispatch) = use_store::<UserStore>();
    let dni = store.dni;

    html!{
        <div class="home-page">
            <div class= "completed-trades">
                <h2 class="title">{"Ultimos Trueques Concretados"}</h2>
                <TradeGridMolecule/>
            </div>
            <div class= "publication-list">
                <h1 class="title">{"Publicaciones"}</h1>
                if dni.is_some() {
                    <Link<Route> to={Route::CreatePublication}>{"Publicar"}</Link<Route>>
                }
                <PublicationGridMolecule/>
            </div>
        </div>
    }
}