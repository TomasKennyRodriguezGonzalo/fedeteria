use yew_router::prelude::Link;
use yew::prelude::*;

use crate::router::Route;



#[function_component(PublicationThumbnail)]
pub fn publication_thumbnail() -> Html {
    html! {
        <Link<Route> to={Route::Publication} class="publication-link">
            <div class="publication">
                <img src="/assets/img/Default_Imagen.jpg"/>
                <div class="info">
                    <h4 class="name">{"This is a default publication"}</h4>
                    <h2 class="price">{"$99999"}</h2>
                </div>
            </div>
        </Link<Route>>
    }
}
