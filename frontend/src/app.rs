use yew::prelude::*;
use yew_router::prelude::*;
use crate::Molecules::navbar::Navbar;
use crate::router::{Route, switch};

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div>
            <BrowserRouter>
                <Navbar/>
                <Switch<Route> render={switch} />
            </BrowserRouter>
        </div>
    }
}