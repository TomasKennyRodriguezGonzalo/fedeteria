use yew::prelude::*;
use yew_router::prelude::*;
use crate::Pages::{log_in_page::LogInPage, home::Home};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/hello-server")]
    HelloServer,
    #[at("/login-page")]
    LogInPage,
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home/> },
        Route::HelloServer => html! { <p>{"Hello Server"}</p> },
        Route::LogInPage => html! { <LogInPage /> },
        Route::NotFound => html! { <h1>{"Error 404 not found!"}</h1>}, 
    }
}
