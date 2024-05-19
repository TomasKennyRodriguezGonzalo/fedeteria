use datos_comunes::QueryPublicacionesFiltradas;
use web_sys::console;
use yew::prelude::*;
use yew_router::hooks::use_location;

use crate::molecules::publication_grid_molecule::PublicationGridMolecule;


#[function_component(SearchResultsPage)]
pub fn search_results_page () -> Html {
    let location = use_location().unwrap();
    let props = location.query::<QueryPublicacionesFiltradas>().unwrap();
    //let props_deserialized: QueryPublicacionesFiltradas = serde_json::from_str(&props.search_query).unwrap();
    log::info!("{:?}", props);

    html! (
        <div class="search-results-box">
            <h1 class="title">{"Resultados Busqueda"}</h1>
            <PublicationGridMolecule query={Some(props)}/>
        </div>
    )
}