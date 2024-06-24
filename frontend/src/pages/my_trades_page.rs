use yew::prelude::*;

use crate::pages::{my_completed_trades_page::MyCompletedTradesPage, my_defined_trades_page::MyDefinedTradesPage, my_pending_trades_page::MyPendingTradesPage, my_trades_offers_page::MyTradesOffersPage};

#[function_component(MyTradesPage)]
pub fn my_trades_page () -> Html {

    html!( 
        <ul class="option-list">
            <MyTradesOffersPage/>
            <MyPendingTradesPage/>
            <MyDefinedTradesPage/>
            <MyCompletedTradesPage/>
        </ul>
    )
}