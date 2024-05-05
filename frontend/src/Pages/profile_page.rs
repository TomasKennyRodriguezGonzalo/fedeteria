use yew::prelude::*;
use yewdux::prelude::*;
use crate::store::UserStore;

#[function_component(ProfilePage)]
pub fn profile_page() -> Html {


    let (store, dispatch) = use_store::<UserStore>();
    let username = store.user.clone();

    html! (
        <>
            <h1>{"PROFILE"}</h1>
            <div>{"your username is: "} {username}</div>

        </>
    )
}