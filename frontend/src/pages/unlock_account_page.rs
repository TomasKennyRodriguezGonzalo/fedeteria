use yew::prelude::*;
use crate::molecules::unlock_account_molecule::UnlockAccountMolecule;

#[function_component(UnlockAccountPage)]
pub fn unlock_account_page() -> Html {

    html!(
        <>
            <UnlockAccountMolecule/>
        </>
    )
}