use datos_comunes::QueryPublicacionesFiltradas;
use web_sys::console;
use yew::prelude::*;

use crate::molecules::publication_grid_molecule::PublicationGridMolecule;

#[derive(Properties,PartialEq)]
pub struct SearchResultsPageProps {
    pub search_query: QueryPublicacionesFiltradas,
}

#[function_component(SearchResultsPage)]
pub fn search_results_page (props: &SearchResultsPageProps) -> Html {
    //let props_deserialized: QueryPublicacionesFiltradas = serde_json::from_str(&props.search_query).unwrap();
    log::info!("{:?}", props_deserialized);

    html! (
        <div class="search-results-box">
            <h1 class="title">{"Resultados Busqueda"}</h1>
            <PublicationGridMolecule query={Some(props_deserialized)}/>
        </div>
    )
}