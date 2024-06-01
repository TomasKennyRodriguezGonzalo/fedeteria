use web_sys::window;
use crate::components::{generic_button::GenericButton, indexed_button::IndexedButton, checked_input_field::CheckedInputField};
use crate::convenient_request::send_notification;
use crate::request_post;
use crate::{router::Route, store::UserStore};
use yew_router::hooks::use_navigator;
use yewdux::use_store;
use datos_comunes::{Publicacion, QueryCrearOferta, QueryEliminarPublicacion, QueryGetUserRole, QueryTasarPublicacion, QueryTogglePublicationPause, ResponseCrearOferta, ResponseEliminarPublicacion, ResponseGetUserRole, ResponsePublicacion, ResponseTasarPublicacion, ResponseTogglePublicationPause, RolDeUsuario, Trueque};
use reqwasm::http::Request;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_hooks::use_effect_once;
use crate::information_store::InformationStore;
use crate::molecules::confirm_prompt_button_molecule::ConfirmPromptButtonMolecule;
use crate::molecules::publication_selector_molecule::PublicationSelectorMolecule;

#[derive(Properties,PartialEq)]
pub struct Props {
    pub id: usize,
}

#[function_component(PublicationMolecule)]
pub fn publication_molecule(props : &Props) -> Html {

    let navigator = use_navigator().unwrap();

    let (_information_store, information_dispatch) = use_store::<InformationStore>();

    let (store, _dispatch) = use_store::<UserStore>();
    let dni = store.dni;
    
    let role_state: UseStateHandle<Option<RolDeUsuario>> = use_state(|| None);
    let cloned_role_state = role_state.clone();
    let cloned_dni = dni.clone();
    use_effect_once(move || {
        let cloned_role_state = cloned_role_state.clone();
        let cloned_dni = cloned_dni.clone();
        if cloned_dni.is_some() {
            let query = QueryGetUserRole { dni : cloned_dni.unwrap() };
            request_post("/api/obtener_rol", query, move |respuesta:ResponseGetUserRole|{
                cloned_role_state.set(Some(respuesta.rol));
            });
        }

        || {}
    });

    let id = props.id.clone();
    let datos_publicacion: UseStateHandle<Option<Publicacion>> = use_state(|| None);
    let datos_publicacion_setter = datos_publicacion.setter();

    let current_image_state = use_state(|| 0);

    let cloned_information_dispatch = information_dispatch.clone();
    let cloned_id = id.clone();
    use_effect_once(move || {
        if dni.is_none(){
            navigator.push(&Route::LogInPage);
            cloned_information_dispatch.reduce_mut(|store| store.messages.push("Para acceder a una publicación debes iniciar sesión".to_string()))
        } else {
            let id = cloned_id;
            spawn_local(async move {
                let respuesta = Request::get(&format!("/api/datos_publicacion?id={id}")).send().await;
                match respuesta{
                    Ok(respuesta) => {
                        let respuesta: Result<ResponsePublicacion, reqwasm::Error> = respuesta.json().await;
                        match respuesta{
                            Ok(respuesta) => {
                                match respuesta {
                                    Ok(publicacion) => {
                                        log::info!("Datos de publicacion!: {publicacion:?}");
                                        datos_publicacion_setter.set(Some(publicacion));
                                    },
                                    Err(error) => {
                                        log::error!("Error de publicacion: {error:?}. TODO INFORMAR BIEN?");
                                    }
                                }
                            }
                            Err(error)=>{
                                log::error!("Error en deserializacion: {}", error);
                            }
                        }
                    }
                    Err(error)=>{
                        log::error!("Error en llamada al backend: {}", error);
                    }
                }
            });
        }
        || {}
    });

    let (_information_store, information_dispatch) = use_store::<InformationStore>();
    let information_dispatch = information_dispatch.clone();
    let cloned_id = id.clone();
    let navigator = use_navigator().unwrap();
    let cloned_navigator = navigator.clone();
    let delete_publication = Callback::from(move |_e:MouseEvent|{
        let cloned_navigator = cloned_navigator.clone();
        let information_dispatch = information_dispatch.clone();
        let cloned_id = cloned_id.clone();
        let query = QueryEliminarPublicacion
        {
            id : cloned_id
        };
        request_post("/api/eliminar_publicacion", query, move |respuesta: ResponseEliminarPublicacion| {
            //si se elimino bien ok sera true
            let cloned_navigator = cloned_navigator.clone();
            let ok = respuesta.ok;
            let information_dispatch = information_dispatch.clone();
            information_dispatch.reduce_mut(|store| store.messages.push("La publicacion ha sido eliminada correctamente".to_string()));
            log::info!("resultado de eliminar publicacion : {ok}");
            cloned_navigator.push(&Route::Home);
        });
    });

    let cloned_datos_publicacion = datos_publicacion.clone();
    
    let (_information_store, information_dispatch) = use_store::<InformationStore>();
    let information_dispatch = information_dispatch.clone();
    let cloned_id = id.clone();
    let toggle_publication_pause = Callback::from(move |()| {
        let cloned_datos_publicacion = cloned_datos_publicacion.clone();
        let id = cloned_id.clone();
        let information_dispatch = information_dispatch.clone();
        spawn_local(async move{
            let cloned_datos_publicacion = cloned_datos_publicacion.clone();
            let information_dispatch = information_dispatch.clone();
            let query = QueryTogglePublicationPause{id : id.clone()};
            let response = Request::post("/api/alternar_pausa_publicacion").header("Content-Type", "application/json").body(serde_json::to_string(&query).unwrap()).send().await;
            match response{
            Ok(response) => {
                let cloned_datos_publicacion = cloned_datos_publicacion.clone();
                let response: Result<ResponseTogglePublicationPause, reqwasm::Error> = response.json().await;
                let information_dispatch = information_dispatch.clone();
                match response {
                    Ok(response) => {
                        let cloned_datos_publicacion = cloned_datos_publicacion.clone();
                        let information_dispatch = information_dispatch.clone();
                        if response.changed {
                            let nombre = (&*cloned_datos_publicacion).clone().unwrap().titulo.clone();
                            let publicacion_pausada = (&*cloned_datos_publicacion).clone().unwrap().pausada.clone();
                            let information_dispatch = information_dispatch.clone();
                            if (publicacion_pausada).clone() {
                                information_dispatch.reduce_mut(|store| store.messages.push(format!("La publicacion {} ha sido despasuada con exito",nombre.clone())));
                            } else {
                                information_dispatch.reduce_mut(|store| store.messages.push(format!("La publicacion {} ha sido pausada con exito",nombre.clone())));
                            }
                            // Refreshes to reset the first load states all over the code
                            if let Some(window) = window() {
                                window.location().reload().unwrap();
                            }
                        } else {
                            log::info!("No se cambió la publicación.")
                        }
                    }
                    Err(error) => {
                        log::error!("{:?}", error)
                    }
                }
            }
            Err(error)=>{
                log::error!("Error en llamada al backend: {}", error);
            }
        }
        });
    });

    let activate_delete_publication_state = use_state(||false);
    let cloned_activate_delete_publication_state = activate_delete_publication_state.clone();

    let activate_delete_publication = Callback::from(move |()|{
        let cloned_activate_delete_publication_state = cloned_activate_delete_publication_state.clone();
        cloned_activate_delete_publication_state.set(true);
    });

    let cloned_activate_delete_publication_state = activate_delete_publication_state.clone();
    let reject_func = Callback::from(move |_e:MouseEvent|{
        let cloned_activate_delete_publication_state = cloned_activate_delete_publication_state.clone();
        cloned_activate_delete_publication_state.set(false);
    });

    let cloned_current_image_state = current_image_state.clone();
    let change_current_image = Callback::from(move |index| {
        cloned_current_image_state.set(index);
    });

    let cloned_current_image_state = current_image_state.clone();
    
    //estado que mantiene las props que se enviaran a la publication grid
    let props_state: UseStateHandle<Option<u64>> = use_state(|| None);

    //estado boton de mostrar selector
    let show_selector_state = use_state(|| false);
    let show_selector_state_cloned = show_selector_state.clone();
    
    let show_selector = Callback::from(move |()| {
        show_selector_state_cloned.set(true);
    });
    
    let show_selector_state_cloned = show_selector_state.clone();
    let hide_selector = Callback::from(move |_input| {
        show_selector_state_cloned.set(false);
    });

    let props = *props_state.clone();
    log::info!("Las props tienen {:?}", props.clone());
    //este es el estado del input, el que va cambiando dinamicamente
    let input_publication_price_state = use_state(|| None);
    let cloned_input_publication_price_state = input_publication_price_state.clone();

    //recordar que html retorna un string aunque sea tipo number por eso hay que hacer la conversion
    let price_changed = Callback::from(move |precio:String|{
        match precio.parse::<u64>() {
            Ok(numero) => {
                log::info!("{:?}",numero.clone());
                let input_publication_price_state = cloned_input_publication_price_state.clone();
                input_publication_price_state.set(Some(numero));
            },
            Err(e) => log::error!("Error al convertir: {}", e),
        }
    });

    //este es el estado de la publicacion en si, el que cambia cuando se aprieta el boton "tasar publicacion"
    let cloned_input_publication_price_state = input_publication_price_state.clone();
    let publication_price_state:UseStateHandle<Option<u64>> = use_state(|| None);
    let cloned_publication_price_state = publication_price_state.clone();
    let cloned_id = id.clone();
    let cloned_datos_publicacion = datos_publicacion.clone();
    let assign_price = Callback::from(move |()|{
        let cloned_datos_publicacion = cloned_datos_publicacion.clone();
        let cloned_publication_price_state = cloned_publication_price_state.clone();
        let cloned_id = cloned_id.clone();
        let publication_price_state = cloned_publication_price_state.clone();
        let input_publication_price_state = cloned_input_publication_price_state.clone();
        if (&*input_publication_price_state).is_some(){
            let query = QueryTasarPublicacion{
                id : cloned_id,
                precio : (&*input_publication_price_state).clone(),
            };
            let input_publication_price_state = cloned_input_publication_price_state.clone();
            request_post("/api/tasar_publicacion", query, move |_respuesta:ResponseTasarPublicacion|{
                let input_publication_price_state = input_publication_price_state.clone();
                let cloned_datos_publicacion = cloned_datos_publicacion.clone();
                let dni_usuario = (&*cloned_datos_publicacion).clone().unwrap().dni_usuario;
                if let Some(window) = window() {
                match window.location().href() {
                    Ok(href) => {
                        log::info!("la href es {}",href);
                        send_notification("Publicación tasada!".to_string(), format!("tu publicación ha sido tasada en un valor de {} pesos!, entrá al link para despausarla y empezar a recibir ofertas de trueque!", (&*input_publication_price_state).clone().unwrap()), href, dni_usuario);
                    },
                    Err(err) => log::error!("Failed to get href: {:?}", err),
                };

                   window.location().reload().unwrap();
                }
            });
        }
        publication_price_state.set((&*input_publication_price_state).clone());
    });

    let cloned_id = id.clone();
    let cloned_datos_publicacion = datos_publicacion.clone();
    let created_offer_state = use_state(|| false);
    let cloned_created_offer_state = created_offer_state.clone();
    let create_offer = Callback::from( move |selected_publications| {
        // Creo el trueque en estado OFERTA
        let oferta = (dni.unwrap(), selected_publications);
        let receptor_dni = (cloned_datos_publicacion.as_ref()).unwrap().dni_usuario;
        let receptor = (receptor_dni, cloned_id);
        let created_offer_state = cloned_created_offer_state.clone();
        let query =  QueryCrearOferta {
            dni_ofertante : oferta.0,
            publicaciones_ofertadas : oferta.1,
            dni_receptor : receptor.0,
            publicacion_receptora : receptor.1,
        };
        request_post("/api/crear_oferta", query, move |respuesta:ResponseCrearOferta|{
            let created_offer_state = created_offer_state.clone();
            created_offer_state.set(respuesta.estado);
        });
        if let Some(window) = window() {
            let dni_receptor = receptor.0;
            send_notification("Nueva Oferta de Trueque!".to_string(), format!("Has recibido una oferta de trueque en tu {} cliquea aquí para verla!", ((&*cloned_datos_publicacion).clone().unwrap().titulo)), window.location().href().unwrap(), dni_receptor);
        }

        
    });

    let cloned_id = id.clone();
    let navigator = use_navigator().unwrap();
    let goto_trade_offers = Callback::from(move |_| {
        
        let _ = navigator.push_with_query(&Route::PublicationTradeOffers, &cloned_id);
        if let Some(window) = window() {
            window.location().reload().unwrap();
        }
    });

    html!{
        <div class="publication-box">
            if let Some(publicacion) = &*datos_publicacion {
                <div class="info">
                <div class="image">
                    <img src={
                        format!("/publication_images/{}", publicacion.imagenes[*cloned_current_image_state])
                    }/>
                    <div class="index-buttons">
                        {
                            publicacion.imagenes.iter().enumerate().map(|(index, _imagen)| {
                                if *cloned_current_image_state == index {
                                    html! {
                                        <button class="selected-button"></button>
                                    }
                                } else {
                                    html! {
                                        <IndexedButton index={index} text="" onclick_event={change_current_image.clone()}></IndexedButton>
                                    }
                                }
                            }).collect::<Html>()
                        }
                    </div> 
                </div> 
                    <div class="text">
                    <h3> {format!("DNI del dueño: {}", publicacion.dni_usuario) } </h3>
                    <h4 class="publication-name">{publicacion.titulo.clone()}</h4>
                    <h2 class="publication-price">{
                        if let Some(precio) = publicacion.precio {
                            if publicacion.pausada {
                                "Publicación Pausada".to_string()
                            } else {
                                precio.to_string()
                            }
                        } else {
                            "Sin Tasar".to_string()
                        }
                        }</h2>
                        <h5 class="description">{publicacion.descripcion.clone()}</h5>
                    </div>
                    </div>
                <div class="publication-selector-container">
                    if publicacion.dni_usuario != dni.clone().unwrap() {
                        <GenericButton text="Agregar publicacion a oferta" onclick_event={show_selector}/>
                        if *show_selector_state { 
                            <PublicationSelectorMolecule price={publicacion.precio.unwrap()} confirmed={create_offer} rejected={hide_selector}/>
                        }
                    }
                </div>
                if publicacion.dni_usuario == dni.clone().unwrap(){
                <div class="moderation-buttons">
                    <GenericButton text="Eliminar Publicación" onclick_event={activate_delete_publication}/>
                    if publicacion.precio.is_some() {
                        if publicacion.pausada {
                            <GenericButton text="Despausar Publicación" onclick_event={toggle_publication_pause}/>
                        } else {
                            <GenericButton text="Pausar Publicación" onclick_event={toggle_publication_pause}/>
                        }
                        <GenericButton text="Ver Ofertas de Trueque" onclick_event={goto_trade_offers}/>
                    }  
                </div>
                }
                {
                    if let Some(role) = &*role_state{
                        match role { 
                            RolDeUsuario::Dueño => {
                                if publicacion.precio.is_none(){
                                    html! {
                                        <>  
                                            <CheckedInputField name = "publication_price_assignment" label="Ingrese el precio de la publicación" tipo = "number" on_change = {price_changed} />
                                            <GenericButton text="Tasar Publicación" onclick_event={assign_price}/>
                                        </>
                                    }
                                } else {
                                    html! {
                                        <>  
                                            <div>{"Publicacion ya tasada"}</div>  
                                        </>
                                    }
                                }
                            },
                            RolDeUsuario::Empleado{sucursal : _} => {
                                if publicacion.precio.is_none(){
                                    html! {
                                        <>  
                                            <CheckedInputField name = "publication_price_assignment" label="Ingrese el precio de la publicación" tipo = "number" on_change = {price_changed} />
                                            <GenericButton text="Tasar Publicación" onclick_event={assign_price}/>
                                        </>
                                    }
                                } else{
                                    html! {
                                        <>  
                                            <div>{"Publicacion ya tasada"}</div>  
                                        </>
                                    }
                                }
                            },
                            RolDeUsuario::Normal => {
                                html!{}
                            }
                        }
                    } else {html!{}}
                }
                if (&*activate_delete_publication_state).clone(){
                    <ConfirmPromptButtonMolecule text="Seguro que quiere eliminar su publicacion?" confirm_func={delete_publication} reject_func={reject_func} />
                }
            } else {
                {"Cargando..."}
            }
            </div>
        }
}