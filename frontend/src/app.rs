use yew::prelude::*;
use yew_router::prelude::*;
use crate::Molecules::HelloServer::HelloServer;
use crate::Components::botonLogIn;
use crate::Pages::LogInPage::LogInPage;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/hello-server")]
    HelloServer,
    #[at("/login-page")]
    LogInPage,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <h1>{ "Hello Frontend" }</h1> },
        Route::HelloServer => html! { <HelloServer /> },
        Route::LogInPage => html! { <LogInPage /> },
    }
}


#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}