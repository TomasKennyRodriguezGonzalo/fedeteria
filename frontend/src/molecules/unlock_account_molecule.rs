use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use datos_comunes::{self, DuringBlockError, QueryUnlockAccount, ResponseUnlockAccount};
use reqwasm::http::Request;
use yewdux::use_store;
use crate::components::dni_input_field::DniInputField;
use crate::components::generic_button::GenericButton;
use crate::information_store::InformationStore;
use crate::molecules::confirm_prompt_button_molecule::ConfirmPromptButtonMolecule;
use datos_comunes::ResponseGetBlockedAccounts;

#[function_component(UnlockAccountMolecule)]
pub fn unlock_account_molecule () -> Html {

    let (information_store, information_dispatch) = use_store::<InformationStore>();

    let dni_state:UseStateHandle<Option<u64>> = use_state(|| None);
    let cloned_dni_state = dni_state.clone();
    let dni_changed = Callback::from(move |dni:String|{
        let dni = Some(dni.parse().unwrap());
        let cloned_dni_state = cloned_dni_state.clone();
        cloned_dni_state.set(dni);
    });

    let state_blocked_accounts: UseStateHandle<Option<Vec<datos_comunes::BlockedUser> >> = use_state(|| None);
    let state_blocked_accounts_clone = state_blocked_accounts.clone();

    let get_blocked_users = Callback::from(move |()| {
        let state_blocked_accounts_clone = state_blocked_accounts_clone.clone();
        {
            spawn_local(async move {
                log::info!("entre al spawn local");
                let respuesta = Request::get("/api/obtener_cuentas_bloqueadas")
                                                        .header("Content-Type", "application/json")
                                                        .send()
                                                        .await;
                match respuesta{
                    Ok(respuesta) =>{
                        let response:Result<ResponseGetBlockedAccounts, reqwasm::Error> = respuesta.json().await;
                        log::info!("deserialice la respuesta {:?}",response);
                        match response{
                            Ok(respuesta) => {           
                                    state_blocked_accounts_clone.set(Some(respuesta.blocked_users));
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

    let state_blocked_accounts_clone = state_blocked_accounts.clone();

    let informe = use_state(|| "".to_string());
    let informe_cloned = informe.clone();

    let show_button_state = use_state(|| false);
    let cloned_show_button_state = show_button_state.clone();
    let reject_account_to_unlock = Callback::from(move |_e:MouseEvent|{
        let cloned_show_button_state = cloned_show_button_state.clone();
        cloned_show_button_state.set(false);
    });

    let cloned_show_button_state = show_button_state.clone();
    let change_show_button_state = Callback::from(move |_| {
        cloned_show_button_state.set(true);
    });


    let user_not_found_state = use_state(||false);
    let cloned_user_not_found_state = user_not_found_state.clone();

    let cloned_information_dispatch = information_dispatch.clone();
    let cloned_show_button_state = show_button_state.clone();
    let cloned_dni_state = dni_state.clone();
    let unlock_account = Callback::from(move |_e: MouseEvent| {
        let cloned_user_not_found_state = cloned_user_not_found_state.clone();

        let cloned_show_button_state = cloned_show_button_state.clone();
        let cloned_dni_state = cloned_dni_state.clone();
        cloned_show_button_state.set(false);
        let state_blocked_accounts_clone = state_blocked_accounts_clone.clone();
        let informe_cloned = informe_cloned.clone();
        let information_dispatch = cloned_information_dispatch.clone();
        {
            spawn_local(async move {
                let cloned_user_not_found_state = cloned_user_not_found_state.clone();
                let cloned_dni_state = cloned_dni_state.clone();
                let account_to_unlock = (&*cloned_dni_state).clone().unwrap();
                let state_blocked_accounts_clone = state_blocked_accounts_clone.clone();
                log::info!("entre al spawn local");
                let query = QueryUnlockAccount {dni: account_to_unlock};
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
                                    match respuesta{
                                        Ok(respuesta)=> {
                                            state_blocked_accounts_clone.set(Some(respuesta.clone()));
                                            information_dispatch.reduce_mut( |store | store.messages.push(format!("Se desbloqueó la cuenta del DNI {}", (&*cloned_dni_state).clone().unwrap())));
                                            log::info!("{:?}", respuesta.clone());
                                        }
                                        Err(error)=>{
                                            cloned_user_not_found_state.set(true);
                                        }
                                    }
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

    let state_blocked_accounts_clone= (*state_blocked_accounts).clone();
    log::info!("state blocked users value: {:?}",state_blocked_accounts_clone);


    html!(
        <div class="unlock-account-box">
            <h1 class="title">{"Desbloquear Usuario"}</h1>
            <section>
                <DniInputField dni = "dni" label="Dni a desbloquear:" tipo = "number" handle_on_change = {dni_changed} />
                if (&*dni_state).clone().is_some() {
                    <GenericButton text="Desbloquear Usuario" onclick_event={change_show_button_state.clone()}/>
                } else {
                    <button class="disabled-dyn-element">{"Desbloquear Usuario"}</button>
                }
                <GenericButton text="Mostrar usuarios bloqueados" onclick_event={get_blocked_users}/>
                if state_blocked_accounts_clone.clone().is_some() {
                    <ul class="blocked-account-list">
                    if !state_blocked_accounts_clone.clone().unwrap().is_empty() {
                        {
                            state_blocked_accounts_clone.clone().unwrap().iter().enumerate().map(|(_index, account)| {
                                html!(
                                    <li class="blocked-account">
                                        <h2>{ format!("DNI: {}, Nombre: {}", account.dni, account.nombre) }</h2>
                                    </li>
                                )
                            }).collect::<Html>()
                        }
                    } else{
                        <h1>{"No hay usuarios bloqueados"}</h1>
                    }
                    </ul>
                }
                if (&*user_not_found_state).clone(){
                    <h1 class="error-text">{"El dni ingresado no pertenece a un usuario bloqueado"}</h1>
                }
            </section>
            if (&*show_button_state).clone(){
                <ConfirmPromptButtonMolecule text = "¿Desea desbloquear la cuenta seleccionada?" confirm_func = {unlock_account.clone()} reject_func = {reject_account_to_unlock.clone()}  />
            }
        </div>
    )
}