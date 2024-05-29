use datos_comunes::Notificacion;
use yew::prelude::*;
use crate::components::generic_button::GenericButton;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id : usize,
}

#[function_component(NotificationThumbnail)]
pub fn notification_thumbnail(props : &Props) -> Html {
    let notification_state: UseStateHandle<Option<Notificacion>> = use_state(|| None);

    use_effect(
        // Traigo la información de la notificación desde el backend con el id recibido

        || {}
    );

    html! {
        <div class="notification-thumbnail">
            <a href="/">
                if let Some(notificacion) = &*notification_state {
                    <h1 class="notification-title">{"Titulo "}</h1>
                    <h4 class="notification-detail">{{"Detalle Notificación"}}</h4>
                } else {
                    {"Cargando..."}
                }
            </a>
        </div>
    }
}