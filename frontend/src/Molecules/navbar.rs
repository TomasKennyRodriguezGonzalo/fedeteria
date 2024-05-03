use yew::prelude::*;
use crate::Components::bb_link::BBLink;
use crate::router::Route;

#[function_component(Navbar)]
pub fn navbar() -> Html{
    html!{
        <header class="navbar">
            <div class="logo">
                <a href="/"><img src="https://github.com/lucagior22/fedeteria_img/blob/main/img/Fedeteria_Solo_Logo.png?raw=true" alt="fedeteria"/></a>
            </div>
            <nav>
                <ul class="option_list">
                // Todas los botones redirigen al HOME, cuando se creen las páginas respectivas podemos cambiar cada HREF
                 //   <BBLink text={"Todo".to_owned()} data_test={"logo".to_owned()} route={Route::Home} />
                    <li><a href="/mis-publicaciones">{"Mis publicaciones"}</a></li>
                    <li><a href="/perfil">{"Perfil"}</a></li>
                    <li><a href="/login-page">{"Iniciar Sesion"}</a></li>
                </ul>
            </nav>
        </header>
    }
}