use yew::prelude::*;

#[derive(Properties,PartialEq)]
pub struct Props {
    pub id : String
}

#[function_component(PublicationPage)]
pub fn publication_page(_props : &Props) -> Html {
    html!(
        <>
            <h1>{"Publicacion"}</h1>
        </>
    )
}