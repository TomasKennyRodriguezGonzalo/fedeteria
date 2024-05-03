use yew::prelude::*;
use yew_router::prelude::*;
use crate::Molecules::{HelloServer::HelloServer, navbar::Navbar};
use crate::Components::boton_log_in;
use crate::Pages::log_in_page::LogInPage;
use crate::router::{Route, switch};

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div>
            <Navbar />
            <BrowserRouter>
                <Switch<Route> render={switch} />
            </BrowserRouter>
        </div>
    }
}