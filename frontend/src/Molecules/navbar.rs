use yew::prelude::*;

#[function_component(Navbar)]
pub fn navbar() -> Html{
    html!{
        <header class="navbar">
            <div class="logo">
                <a href="/"><img src="/assets/img/Fedeteria_Solo_Logo.svg" alt="fedeteria"/></a>
            </div>
            <nav>
                <ul class="option_list">
                    <li><a href="/mis-publicaciones">{"Mis publicaciones"}</a></li>
                    <li><a href="/perfil">{"Perfil"}</a></li>
                    <li><a href="/login">{"Iniciar Sesion"}</a></li>
                </ul>
            </nav>
        </header>
    }
}