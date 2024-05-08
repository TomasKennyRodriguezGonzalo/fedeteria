use datos_comunes::{QueryObtenerUsuario, ResponseObtenerUsuario};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use reqwasm::http::Request;
use yewdux::prelude::*;
use crate::store::{UserStore};

#[function_component(ProfilePage)]
pub fn profile_page() -> Html {

    let (store, dispatch) = use_store::<UserStore>();
    let dni = store.dni.clone();
    use_effect( move ||{
        let dispatch = dispatch.clone();
        let dni = dni.clone();
        if let Some(dni) = dni{
            spawn_local(async move {
                let query = QueryObtenerUsuario{dni:dni.clone()};
                let dispatch = dispatch.clone();
                let cloned_dni = dni.clone();
                let respuesta = Request::post("/api/retornar_usuario").header("Content-Type", "application/json").body(serde_json::to_string(&query).unwrap()).send().await;
                let response = match respuesta {
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
        } else{
            todo!()
        }
    });

    let (store, dispatch) = use_store::<UserStore>();
    let username = store.nombre.clone();


    html! (
        <>
            <h1>{"PERFIL"}</h1>
            if dni.is_some(){
                <div>{"tu nombre es: "} {username.clone()}</div>
            }
        </>
    )
}