use crate::components::publication_thumbnail::PublicationThumbnail;
use datos_comunes::Publicacion;
use yew::prelude::*;

#[function_component(PublicationGridMolecule)]
pub fn publication_grid_molecule() -> Html {
    let publication_list_state: UseStateHandle<Option<Vec<Publicacion>>> = use_state(|| None);
    
    html!{
        <div class="publication-grid">
            if (&*publication_list_state).is_some() {
                <ul>
                    {
                        (&*publication_list_state).iter().map(move |publicacion| html! {
                            <li><PublicationThumbnail/></li>
                        }).collect::<Html>()
                    }
                </ul>
            }
        </div>
    }
}