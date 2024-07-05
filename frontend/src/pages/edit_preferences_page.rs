use crate::molecules::edit_preferences_molecule::EditPreferencesMolecule;
use yew::prelude::*;

#[function_component(EditPreferencesPage)]
pub fn edit_preferences() -> Html{
    html! {
        <EditPreferencesMolecule/>
    }
}