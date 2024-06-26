use datos_comunes::{QueryEliminarNotificacion, QueryGetNotificaciones, ResponseEliminarNotificacion, ResponseNotificaciones};
use yew::prelude::*;
use yew_hooks::use_effect_once;
use yewdux::use_store;
use crate::{components::{indexed_button::IndexedButton, notification_thumbnail::NotificationThumbnail}, request_post, store::UserStore};


#[function_component(NotificationsPage)]
pub fn notifications_page() -> Html {

    let notification_list:UseStateHandle<Vec<usize>> = use_state(|| vec![]);

    let (store, _store_dispatch) = use_store::<UserStore>();
    let dni = store.dni.unwrap();
    

    let cloned_notification_list = notification_list.clone();
    let cloned_dni = dni.clone();
    use_effect_once( move || {
        let dni = cloned_dni.clone();
        
        // Traerme la lista de notificaciones del usuario
        let notification_list = cloned_notification_list.clone();
        let query = QueryGetNotificaciones {
            dni,
        };
        request_post("/api/obtener_notificaciones", query, move |respuesta: ResponseNotificaciones|{
            let mut notificaciones = respuesta.notificaciones;
            notificaciones.reverse();
            notification_list.set(notificaciones);
        });
        
        || {}
    });

let cloned_dni = dni.clone();
let cloned_notification_list = notification_list.clone();
let delete_notification = Callback::from(move |index| {
    log::info!("el index a borrar es {}",index);
    // Elimino la notificación con el indice recibido de IndexedButton y el dni del usuario del UserStore
    let notification_list = cloned_notification_list.clone();
    let query = QueryEliminarNotificacion
    {
        dni : cloned_dni,
        index
    };
    request_post("/api/eliminar_notificacion", query, move |respuesta: ResponseEliminarNotificacion|{
        let notificaciones: ResponseEliminarNotificacion = respuesta;
        notification_list.set(notificaciones.notificaciones);
    });
});

    html! {
        <div class="notifications-box">
            <h1 class="title">{"Notificaciones"}</h1>
            if (&*notification_list).clone().is_empty(){
                <h1>{"Aún no tienes notificaciones"}</h1>
            }                
            <ul>
                {
                    (&*notification_list).iter().map(|index| {
                        html! {
                            <li>
                                <NotificationThumbnail id={index} dni={dni.clone()}/>
                                <IndexedButton text={"X".to_string()} onclick_event={delete_notification.clone()} index={index} />
                            </li>
                        }
                    }).collect::<Html>()
                }
            </ul>
        </div>
    }
}