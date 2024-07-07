use web_sys::window;
use crate::components::{generic_button::GenericButton, indexed_button::IndexedButton, checked_input_field::CheckedInputField};
use crate::request_post;
use crate::{router::Route, store::UserStore};
use yew_router::hooks::use_navigator;
use yewdux::use_store;
use datos_comunes::*;
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
    
    let publicacion_en_trueque_state = use_state(|| false);

    let question_text_state = use_state(||"".to_string());
    let question_text_state_cloned = question_text_state.clone();
    let question_text_changed = Callback::from(move|question|{
        question_text_state_cloned.set(question);
    });

    let answer_text_state = use_state(||"".to_string());
    let answer_text_state_cloned = answer_text_state.clone();
    let answer_text_changed = Callback::from(move|answer|{
        answer_text_state_cloned.set(answer);
    });
    
    let id = props.id.clone();
    let cloned_id = id.clone();
    let is_in_saved_state = use_state(||false);
    let cloned_is_in_saved_state = is_in_saved_state.clone();

    let role_state: UseStateHandle<Option<RolDeUsuario>> = use_state(|| None);
    let cloned_role_state = role_state.clone();
    let cloned_dni = dni.clone();
    use_effect_once(move || {
        if let Some(dni) = cloned_dni {
            let query = QueryGetUserRole { dni };
            request_post("/api/obtener_rol", query, move |respuesta:ResponseGetUserRole|{
                cloned_role_state.set(Some(respuesta.rol));
            });
            let query = QueryPublicacionGuardada{ dni:dni , id_publicacion:cloned_id};
            request_post("/api/publicacion_guardada", query, move|respuesta:ResponsePublicacionGuardada|{
                cloned_is_in_saved_state.set(respuesta.guardada);
            });
        }

        || {}
    });

    let datos_publicacion: UseStateHandle<Option<Publicacion>> = use_state(|| None);
    let datos_publicacion_setter = datos_publicacion.setter();

    let current_image_state = use_state(|| 0);

    let activate_assign_price_state = use_state(|| false);

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


    let cloned_information_dispatch = information_dispatch.clone();
    let cloned_id = id.clone();
    let navigator = use_navigator().unwrap();
    let cloned_navigator = navigator.clone();
    let delete_publication = Callback::from(move |_e:MouseEvent|{
        let cloned_navigator = cloned_navigator.clone();
        let information_dispatch = cloned_information_dispatch.clone();
        let cloned_id = cloned_id.clone();
        let query = QueryEliminarPublicacion
        {
            id : cloned_id
        };
        request_post("/api/eliminar_publicacion", query, move |respuesta: ResponseEliminarPublicacion| {
            //si se elimino bien ok sera true
            let cloned_navigator = cloned_navigator.clone();
            if respuesta.ok {
                let information_dispatch = information_dispatch.clone();
                information_dispatch.reduce_mut(|store| store.messages.push("La publicacion ha sido eliminada correctamente".to_string()));
                cloned_navigator.push(&Route::Home);
            }
            else {
                let information_dispatch = information_dispatch.clone();
                information_dispatch.reduce_mut(|store| store.messages.push("No es posible eliminar la publicacion, ya que cuenta con ofertas. Rechazelas si desea eliminar la publicación".to_string()));
            }
            log::info!("resultado de eliminar publicacion : {}", respuesta.ok);
        });
    });

    
    let cloned_datos_publicacion = datos_publicacion.clone();
    let cloned_information_dispatch = information_dispatch.clone();
    let cloned_id = id.clone();
    let toggle_publication_pause = Callback::from(move |()| {
        let cloned_datos_publicacion = cloned_datos_publicacion.clone();
        let id = cloned_id.clone();
        let information_dispatch = cloned_information_dispatch.clone();
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
                        let nombre = (&*cloned_datos_publicacion).clone().unwrap().titulo.clone();
                        if response.changed {
                            //let nombre = (&*cloned_datos_publicacion).clone().unwrap().titulo.clone();
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
                            information_dispatch.reduce_mut(|store| store.messages.push(format!("La publicacion {} no ha sido despasuada debido a que tienes trueques pendientes, definidos o finalizados",nombre.clone())));
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
    let cloned_information_dispatch = information_dispatch.clone();
    let cloned_input_publication_price_state = input_publication_price_state.clone();
    let publication_price_state:UseStateHandle<Option<u64>> = use_state(|| None);
    let cloned_publication_price_state = publication_price_state.clone();
    let cloned_id = id.clone();
    let cloned_datos_publicacion = datos_publicacion.clone();
    let assign_price = Callback::from(move |_event|{
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
                   window.location().reload().unwrap();
                }
            });
        }
        cloned_information_dispatch.reduce_mut(|store| store.messages.push(format!("Publicación tasada en ${}.", (input_publication_price_state.clone()).unwrap())));
        publication_price_state.set((&*input_publication_price_state).clone());
    });


    let cloned_information_dispatch = information_dispatch.clone();
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
            window.location().reload().unwrap();
        }

    });

    let cloned_id = id.clone();
    let navigator = use_navigator().unwrap();
    let goto_trade_offers = Callback::from(move |_| {
        
        let query = QueryTruequesFiltrados{
            filtro_codigo_ofertante: None,
            filtro_codigo_receptor: None,
            //filtro_ofertante: None,
            //filtro_receptor: None,
            filtro_dni_integrantes: None,
            filtro_estado: Some(EstadoTrueque::Oferta),
            filtro_fecha_pactada: None,
            filtro_fecha_trueque: None,
            filtro_id_publicacion: Some(cloned_id),
            filtro_sucursal: None,
        };
        let _ = navigator.push_with_query(&Route::SearchTrueques, &query);
        if let Some(window) = window() {
            window.location().reload().unwrap();
        }
    });

    let cloned_activate_assign_price_state = activate_assign_price_state.clone();
    let ask_assign_price_confirmation = Callback::from(move|_event| {
        cloned_activate_assign_price_state.set(true);
    });

    let cloned_activate_assign_price_state = activate_assign_price_state.clone();
    let reject_assign_price_confirmation = Callback::from(move|_event| {
        cloned_activate_assign_price_state.set(false);
    });

    let cloned_id = id.clone();

    let show_question_state = use_state(||false);

    let cloned_show_question_state = show_question_state.clone();
    let show_question_prompt = Callback::from(move|_|{
        cloned_show_question_state.set(true);
    });

    let cloned_show_question_state = show_question_state.clone();
    let hide_show_question_state = Callback::from(move|_:MouseEvent|{
        cloned_show_question_state.set(false);
    });
    
    let cloned_show_question_state = show_question_state.clone();
    let cloned_dni = dni.clone();
    let question_text_state_cloned = question_text_state.clone();
    let cloned_datos_publicacion = datos_publicacion.clone();
    let ask_question = Callback::from(move|_:MouseEvent|{
        if (&*question_text_state_cloned).split_whitespace().count() >= 2{
            if let Some(dni) = cloned_dni {
                if let Some(_publicacion) = &*cloned_datos_publicacion{
                    let query = QueryAskQuestion{ dni_preguntante:dni, pregunta:(&*question_text_state_cloned).clone(), id_publicacion:cloned_id};
                    request_post("/api/preguntar",query, move |_respuesta:ResponseAskQuestion|{});
                }
            }
            if let Some(window) = window() {
                window.location().reload().unwrap();
            }
        } else{
            //notificar que la pregunta no puede estar vacia
        }
        cloned_show_question_state.set(false);
    });

    let cloned_dni = dni.clone();
    let add_pub_to_saved = Callback::from(move|()|{
        if let Some(dni) = cloned_dni {
            let query = QueryAgregarAGuardados {dni:dni, id_publicacion:cloned_id};
            request_post("/api/guardar_publicacion",query,move |_respuesta:ResponseAgregarAGuardados|{

            });
        }
        if let Some(window) = window() {
            window.location().reload().unwrap();
        }
    });


    let cloned_dni = dni.clone();
    let remove_saved_pub = Callback::from(move|()|{
        if let Some(dni) = cloned_dni {
            let query = QueryEliminarGuardados {dni:dni, id_publicacion:cloned_id};
            request_post("/api/eliminar_publicacion_guardados",query,move |_respuesta:ResponseEliminarGuardados|{

            });
        }
        if let Some(window) = window() {
            window.location().reload().unwrap();
        }
    });

  



    let cloned_dni = dni.clone();
    let add_pub_to_saved = Callback::from(move|_|{
        if let Some(dni) = cloned_dni {
            let query = QueryAgregarAGuardados {dni:dni, id_publicacion:cloned_id};
            request_post("/api/guardar_publicacion",query,move |_respuesta:ResponseAgregarAGuardados|{

            });
        }
        if let Some(window) = window() {
            window.location().reload().unwrap();
        }
    });


    let cloned_dni = dni.clone();
    let remove_saved_pub = Callback::from(move|_|{
        if let Some(dni) = cloned_dni {
            let query = QueryEliminarGuardados {dni:dni, id_publicacion:cloned_id};
            request_post("/api/eliminar_publicacion_guardados",query,move |_respuesta:ResponseEliminarGuardados|{

            });
        }
        if let Some(window) = window() {
            window.location().reload().unwrap();
        }
    });

  




    let cloned_id = id.clone();
    let cloned_datos_publicacion = datos_publicacion.clone();
    let answer_text_state_cloned = answer_text_state.clone();
    let answer_question = Callback::from(move |index|{
        if !(&*answer_text_state_cloned).is_empty(){
                if let Some(_publicacion) = &*cloned_datos_publicacion{
                    let query = QueryAnswerQuestion{indice_pregunta : index, id_publicacion:cloned_id, respuesta:(*answer_text_state_cloned).clone()};
                    request_post("/api/responder",query, move |_respuesta:ResponseAnswerQuestion|{

                    });
            }
        }
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
                // Seccion de Titulo, Precio y descripcion
                </div> 
                    <div class="text">
                    <h3> {format!("DNI del dueño: {}", publicacion.dni_usuario) } </h3>
                    <h4 class="publication-name">{publicacion.titulo.clone()}</h4>
                    <h2 class="publication-price">{
                        if let Some(precio) = publicacion.precio {
                            if publicacion.pausada && !publicacion.intercambiada {
                                "Publicación Pausada".to_string()
                            } 
                            else if publicacion.eliminada {
                                "Publicación Eliminada".to_string()
                            }
                            else if publicacion.intercambiada {
                                "Publicación Intercambiada".to_string()
                            }
                            else {
                                let mut incluir = false;
                                if let Some(dni) = dni {
                                    if publicacion.dni_usuario == dni {
                                        incluir = true;
                                    }
                                    if let Some(role) = &*role_state {
                                        match role { 
                                            RolDeUsuario::Dueño | RolDeUsuario::Empleado{sucursal : _} => {
                                                incluir = true;
                                            },
                                            _ => {}
                                        }
                                    }
                                }
                                get_string_de_rango(precio, incluir)
                            }
                        } else {
                            "Sin Tasar".to_string()
                        }
                        }</h2>
                        <h5 class="description">{publicacion.descripcion.clone()}</h5>
                    </div>
                    </div>
                if !publicacion.eliminada {
                    // Seccion de propuesta de oferta
                    <div class="publication-selector-container">
                        if publicacion.dni_usuario != dni.clone().unwrap() {
                            if publicacion.precio.is_some() && !publicacion.intercambiada {
                                <GenericButton text="Proponer Trueque" onclick_event={show_selector}/>
                            }
                            if *show_selector_state { 
                                <GenericButton text="X" onclick_event={hide_selector.clone()}/>
                                <PublicationSelectorMolecule price={publicacion.precio.unwrap()} confirmed={create_offer} rejected={hide_selector}/>
                            }
                        }
                    </div>
                    // Seccion de moderacion de publicacion propia
                    if publicacion.dni_usuario == dni.clone().unwrap(){
                    <div class="moderation-buttons">
                        if !publicacion.en_trueque || publicacion.intercambiada {
                            <GenericButton text="Eliminar Publicación" onclick_event={activate_delete_publication}/>
                        }
                        if (publicacion.precio.is_some()) && (!publicacion.en_trueque) {
                            if publicacion.pausada {
                                <GenericButton text="Despausar Publicación" onclick_event={toggle_publication_pause}/>
                            } else {
                                <GenericButton text="Pausar Publicación" onclick_event={toggle_publication_pause}/>
                            }
                            <GenericButton text="Ver Ofertas de Trueque" onclick_event={goto_trade_offers}/>
                        }  
                    </div>
                    }
                    // Seccion de tasacion de publicacion
                    {
                        if let Some(role) = &*role_state{
                            match role { 
                                RolDeUsuario::Dueño | RolDeUsuario::Empleado{sucursal : _} => {
                                    if publicacion.precio.is_none(){
                                        html! {
                                            <div class="assign-price-box">  
                                                <h1 class="title">{"Tasar"}</h1>
                                                <CheckedInputField name = "publication_price_assignment" label="Ingrese el precio de la publicación" tipo = "number" on_change={price_changed} />
                                                if let Some(input) = &*input_publication_price_state {
                                                    if input != &(0 as u64) {
                                                        <GenericButton text="Tasar Publicación" onclick_event={ask_assign_price_confirmation}/>
                                                    } else {
                                                        <button class="disabled-dyn-element">{"Tasar Publicación"}</button>
                                                    }
                                                } else {
                                                    <button class="disabled-dyn-element">{"Tasar Publicación"}</button>
                                                }
                                            </div>
                                        }
                                    } else {
                                        html! {}
                                    }
                                },
                                RolDeUsuario::Normal => {
                                    html!{}
                                }
                            }
                        } else {html!{}}
                    }
                }

                if !publicacion.pausada{
                    //seccion preguntas y respuestas
                    {
                        html!{
                            <div class="question-answer-box">
                            <h1 class="title">{"Preguntas y respuestas"}</h1>
                            <div class="question-input">
                                    if (publicacion.dni_usuario != dni.clone().unwrap()) && (!publicacion.eliminada) {
                                        <CheckedInputField name = "question-field" placeholder="Escriba su pregunta" tipo = "text" on_change={question_text_changed}/>
                                        <GenericButton text="Enviar pregunta" onclick_event={show_question_prompt}/>
                                    }
                                        if (&*show_question_state).clone(){
                                        <ConfirmPromptButtonMolecule text = "¿Seguro que quiere realizar esta pregunta?" confirm_func = {ask_question} reject_func = {hide_show_question_state}  />
                                    }
                                </div>
                            <ul class="question-answer-list">    
                                    {
                                        publicacion.preguntas.iter().enumerate().map(|(index,pregunta)|{
                                            html!{
                                                <li class="question-answer-item">
                                                    <h2 class="question">{""}{(pregunta.pregunta).clone()}</h2>
                                                    if publicacion.dni_usuario == dni.clone().unwrap() && pregunta.respuesta.is_none() && !publicacion.eliminada {
                                                    <div class="answer-input">
                                                            <CheckedInputField name = "answer-field" placeholder="Escriba su Respuesta" tipo = "text" on_change={(answer_text_changed).clone()}/>
                                                            <IndexedButton text="Enviar" index={index} onclick_event={(answer_question).clone()}/>
                                                    </div>
                                                    }
                                                    
                                                    if let Some(respuesta) = (pregunta.respuesta).clone(){
                                                        <h4 class="answer">{""}{(respuesta).clone()}</h4>
                                                    } else {
                                                    if publicacion.dni_usuario != dni.clone().unwrap(){
                                                            <h4 class="unanswered">{"el dueño de la publicación aún no ha respondido esta pregunta."}</h4>
                                                        }
                                                    }
                                                </li>    
                                        }
                                        }).collect::<Html>()
                                    }
                                </ul>
                        </div>
                        }
    
                    }
    
                    //guardar publicacion
                    if publicacion.dni_usuario != dni.unwrap(){
                        if !*is_in_saved_state{
                            //entonces agregar publicacion a guardados  
                            <GenericButton text="Agregar publicacion a guardados" onclick_event={add_pub_to_saved}/>
                        } else{
                            <GenericButton text="Eliminar publicacion de guardados" onclick_event={remove_saved_pub}/>
                        }
                    }
                }
    
            if (&*activate_assign_price_state).clone(){
                <ConfirmPromptButtonMolecule text="¿Confirma la tasación?" confirm_func={assign_price} reject_func={reject_assign_price_confirmation} />
            }

                if (&*activate_delete_publication_state).clone() {
                    <ConfirmPromptButtonMolecule text="¿Desea eliminar la publicación de forma permanente?" confirm_func={delete_publication} reject_func={reject_func} />
                }

            } else {
                {"Cargando..."}
            }
            </div>
        }
}