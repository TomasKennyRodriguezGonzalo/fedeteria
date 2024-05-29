use datos_comunes::{Notificacion, ResponseNotificacion, QueryNotificacion};
use serde::de::Error;
use yew::prelude::*;
use yew_hooks::use_effect_once;
use crate::{components::generic_button::GenericButton, request_post};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id : usize,
    pub dni : u64,
}

#[function_component(NotificationThumbnail)]
pub fn notification_thumbnail(props : &Props) -> Html {
    let notification_state: UseStateHandle<Option<Notificacion>> = use_state(|| None);
    let dni = props.dni.clone();
    let id = props.id.clone();
    
    let cloned_notification_state = notification_state.clone();
    use_effect_once( move || {
        // Traigo la información de la notificación desde el backend con el id recibido
        let query = QueryNotificacion {
            dni : dni.clone(),
            index : id.clone(),
        };
        request_post(("/api/datos_notificacion?id={id}"), query, move |respuesta: ResponseNotificacion|{
            log::info!("{respuesta:?}");
            cloned_notification_state.set(respuesta.notificacion);
        });
        
        || {}
    });

    html! {
        <div class="notification-thumbnail">
            <a href="/">
                if let Some(notificacion) = &*notification_state {
                    <h1 class="notification-title">{notificacion.titulo.clone()}</h1>
                    <h4 class="notification-detail">{notificacion.detalle.clone()}</h4>
                } else {
                    {"Cargando..."}
                }
            </a>
        </div>
    }
}