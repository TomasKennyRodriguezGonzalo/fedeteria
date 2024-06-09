use yew::prelude::*;
use yew_router::hooks::use_location;
use crate::molecules::finish_trade_molecule::FinishTradeMolecule;

#[function_component(FinishTradePage)]
pub fn finish_trade_page () -> Html{
    html! {
        <FinishTradeMolecule/>
    }
}