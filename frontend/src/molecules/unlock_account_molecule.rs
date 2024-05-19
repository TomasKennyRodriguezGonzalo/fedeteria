use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use datos_comunes::{self, QueryUnlockAccount, ResponseUnlockAccount};
use reqwasm::http::Request;
use crate::components::indexed_button::IndexedButton;
use crate::components::generic_button::GenericButton;
use crate::molecules::confirm_prompt_button_molecule::ConfirmPromptButtonMolecule;
//use crate::components::indexed_button::IndexedButton;
use datos_comunes::ResponseGetBloquedAccounts;

#[function_component(UnlockAccountMolecule)]
pub fn unlock_account_molecule () -> Html {

    let state_bloqued_accounts = use_state(|| Vec::new());
    let state_bloqued_accounts_clone = state_bloqued_accounts.clone();

    let clicks = use_state(|| 0);
    let clicks_cloned = clicks.clone();

    let get_bloqued_users = Callback::from(move |()| {
        let state_bloqued_accounts_clone = state_bloqued_accounts_clone.clone();
        clicks_cloned.set(&*clicks_cloned + 1); 
        {
            spawn_local(async move {
                log::info!("entre al spawn local");
                let respuesta = Request::get("/api/obtener_cuentas_bloqueadas")
                                                        .header("Content-Type", "application/json")
                                                        .send()
                                                        .await;
                match respuesta{
                    Ok(respuesta) =>{
                        let response:Result<ResponseGetBloquedAccounts, reqwasm::Error> = respuesta.json().await;
                        log::info!("deserailice la respuesta {:?}",response);
                        match response{
                            Ok(respuesta) => {           
                                    state_bloqued_accounts_clone.set(respuesta.bloqued_users);
                            }
                            Err(error)=>{
                                log::error!("Error en deserializacion: {}", error);
                            }
                        }
                    }
                    Err(error)=>{
                        log::error!("Error en llamada al backend: {}", error);

                    }
                }

            });
        }
    });

    let state_bloqued_accounts_clone = state_bloqued_accounts.clone();

    let informe = use_state(|| "".to_string());
    let informe_cloned = informe.clone();

    let show_button_state = use_state(|| false);
    let cloned_show_button_state = show_button_state.clone();
    let reject_account_to_unlock = Callback::from(move |_e:MouseEvent|{
        let cloned_show_button_state = cloned_show_button_state.clone();
        cloned_show_button_state.set(false);
    });

    let cloned_show_button_state = show_button_state.clone();

    let state_index_account_to_unlock = use_state(|| 0);
    let state_index_account_to_unlock_clone = state_index_account_to_unlock.clone();
    let change_index_account_to_unlock = Callback::from(move |index: usize| {
        state_index_account_to_unlock_clone.set(index);
        cloned_show_button_state.set(true);
    });
    let state_index_account_to_unlock_clone = state_index_account_to_unlock.clone(); 
    let cloned_show_button_state = show_button_state.clone();

    let unlock_account = Callback::from(move |_e: MouseEvent| {
        cloned_show_button_state.set(false);
        let state_bloqued_accounts_clone = state_bloqued_accounts_clone.clone();
        let informe_cloned = informe_cloned.clone();
        let index = &*state_index_account_to_unlock_clone;
        let account_to_unlock = state_bloqued_accounts_clone.get(*index).unwrap().clone();
        {
            spawn_local(async move {
                let account_to_unlock = account_to_unlock.clone();
                let state_bloqued_accounts_clone = state_bloqued_accounts_clone.clone();
                log::info!("entre al spawn local");
                let query = QueryUnlockAccount {dni: account_to_unlock.dni.clone()};
                let respuesta = Request::post("/api/desbloquear_cuenta")
                                                                .header("Content-Type", "application/json")
                                                                .body(serde_json::to_string(&query).unwrap())
                                                                .send()
                                                                .await;
                match respuesta{
                    Ok(respuesta) =>{
                        let response:Result<ResponseUnlockAccount, reqwasm::Error> = respuesta.json().await;
                        log::info!("deserailice la respuesta {:?}",response);
                        match response{
                            Ok(respuesta) => {
                                state_bloqued_accounts_clone.set(respuesta.bloqued_users.clone());
                                informe_cloned.set("Sucursal Eliminada".to_string());
                                log::info!("{:?}", respuesta.bloqued_users.clone());
                            }
                            Err(error)=>{
                                log::error!("Error en deserializacion: {}", error);
                                informe_cloned.set("Ocurrio un error".to_string());
                            }
                        }
                    }
                    Err(error)=>{
                        log::error!("Error en llamada al backend: {}", error);
                        informe_cloned.set("Ocurrio un error".to_string());
                    }
                }
            });
        }
    });

    let state_bloqued_accounts_clone = &*state_bloqued_accounts.clone();
    log::info!("state bloqued users value: {:?}",*state_bloqued_accounts_clone);

    html!(
        <div class="unlock_account_box">
            <h1>{"Desbloquear Usuario"}</h1>
            <section>
                <GenericButton text="Obtener usuarios bloqueados" onclick_event={get_bloqued_users}/>
                <ul class="bloqued_account_list">
                if &*clicks != &0 {
                    if !state_bloqued_accounts_clone.is_empty() {
                        <div class="showing-bloqued-accounts">
                        {
                            state_bloqued_accounts_clone.iter().enumerate().map(|(index, account)| {
                                html!(
                                    <div class="show-bloqued-account">
                                        <h2>{ format!("DNI: {}, Nombre: {}", account.dni, account.nombre) }</h2>
                                        <IndexedButton text="Desbloquear Cuenta" index={index.clone()} onclick_event={change_index_account_to_unlock.clone()}/>
                                    </div>
                                )
                            }).collect::<Html>()
                        }
                        if (&*show_button_state).clone(){
                            <h2> {"¿Desea desbloquear la cuenta seleccionada?"}</h2>
                            <ConfirmPromptButtonMolecule text = "¿Desea desbloquear la cuenta seleccionada?" confirm_func = {unlock_account.clone()} reject_func = {reject_account_to_unlock.clone()}  />
                        }
                        </div>
                    } else{
                    <h1>{"No hay usuarios bloqueados"}</h1>
                    }
                }
                </ul>
            </section>
        </div>
    )
}