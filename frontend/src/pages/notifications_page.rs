use yew::prelude::*;
use crate::components::{indexed_button::IndexedButton, notification_thumbnail::NotificationThumbnail};


#[function_component(NotificationsPage)]
pub fn notifications_page() -> Html {

    let notification_list = use_state(|| vec!["".to_string(), "".to_string(), "".to_string()]);

    use_effect(
        // Traerme la lista de notificaciones del usuario
        || {}
    );

    let delete_notification = Callback::from(|index| {
        // Elimino la notificación con el indice recibido de IndexedButton y el dni del usuario del UserStore
    });

    html! {
        <div class="notifications-box">
            <h1 class="title">{"Notificaciones"}</h1>
            <ul>
                {
                    (&*notification_list).iter().enumerate().map(|(index, _notification)| {
                        html! {
                            <li>
                                <NotificationThumbnail id={1}/>
                                <IndexedButton text={"X".to_string()} onclick_event={delete_notification.clone()} index={index} />
                            </li>
                        }
                    }).collect::<Html>()
                }
            </ul>
        </div>
    }
}