use datos_comunes::QueryTruequesFiltrados;
use yew::prelude::*;
use yew_router::hooks::use_location;

use crate::molecules::trade_grid_molecule::TradeGridMolecule;


#[function_component(SearchTruequesPage)]
pub fn search_trueques_page () -> Html {
    let location = use_location().unwrap();
    let props = location.query::<QueryTruequesFiltrados>().unwrap();
    log::info!("{:?}", props);

    html! (
        <div class="search-trueques-box">
            <TradeGridMolecule query={Some(props)}/>
        </div>
    )
}