use crate::components::publication_thumbnail::PublicationThumbnail;
use datos_comunes::Publicacion;
use yew::prelude::*;

#[function_component(PublicationGridMolecule)]
pub fn publication_grid_molecule() -> Html {
    let publication_list_state: UseStateHandle<Option<Vec<String>>> = use_state(|| None);
    
    let first_load = use_state(|| true);
    
    let cloned_publication_list_state = publication_list_state.clone();
    let cloned_first_load = first_load.clone();
    use_effect(move || {
        if (&*cloned_first_load).clone() {
            // Logica de traer un vector de IDs 
            cloned_publication_list_state.set(Some(vec![
                "0".to_string(),
                "1".to_string(),
                "2".to_string(),
                "3".to_string(),
                "4".to_string(),
                "5".to_string(),
                "6".to_string(),
                "7".to_string(),
                "8".to_string(),
                "9".to_string(),
                "10".to_string(),
                "11".to_string(),
                "12".to_string(),
                "13".to_string(),
                "14".to_string(),
                "15".to_string(),
                "16".to_string(),
                "17".to_string(),
                "18".to_string(),
                "19".to_string(),
                "20".to_string(),
            ]));
            cloned_first_load.set(false)
        }
    });

    html!{
        <div class="publication-grid">
            if (&*publication_list_state).is_some() {
                <ul>
                    {
                        (publication_list_state).as_ref().unwrap().iter().map(|id| {
                            html! {
                                <li><PublicationThumbnail id={id.clone()}/></li>
                            }
                        }).collect::<Html>()
                    }
                </ul>
            }
        </div>
    }
}