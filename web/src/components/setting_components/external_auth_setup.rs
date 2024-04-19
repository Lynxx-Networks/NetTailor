use yew::prelude::*;
use wasm_bindgen::JsCast;
use yewdux::prelude::*;
use crate::components::context::AppState;
use yew::platform::spawn_local;
use web_sys::{window, Blob, Url, BlobPropertyBag};
use wasm_bindgen::JsValue;
use crate::requests::setting_reqs::{call_backup_user, ExternalAuthResponse, ExternalAuthRequest, call_add_external_auth, ExternalAuthSetting, call_get_all_external_auths};

#[function_component(ExternalAuthSetup)]
pub fn external_auth() -> Html {
    let (state, _dispatch) = use_store::<AppState>();
    let api_key = state.auth_details.as_ref().map(|ud| ud.api_key.clone());
    let user_id = state.user_details.as_ref().map(|ud| ud.UserID.clone());
    let server_name = state.auth_details.as_ref().map(|ud| ud.server_name.clone());
    let provider_values: UseStateHandle<Vec<ExternalAuthSetting>> = use_state(Vec::new);
    {
        let provider_values = provider_values.clone();
        use_effect_with((api_key.clone(), server_name.clone()), move |(api_key, server_name)| {
            let provider_values = provider_values.clone();
            let api_key = api_key.clone();
            let server_name = server_name.clone();
            let future = async move {
                if let (Some(api_key), Some(server_name)) = (api_key, server_name) {
                    let response = call_get_all_external_auths(&server_name, &api_key.unwrap()).await;
                    match response {
                        Ok(external_auth_info) => {
                            provider_values.set(external_auth_info);
                        },
                        Err(e) => web_sys::console::log_1(&format!("Error getting user info: {}", e).into()),
                    }
                }
            };
            spawn_local(future);
            // Return cleanup function
            || {}
        });
    }

    #[derive(Clone, PartialEq)]
    enum PageState {
        Hidden,
        Azure,
        Github,
    }

    let client_id = use_state(|| "".to_string());
    let tenant_id = use_state(|| "".to_string());
    let url_endpoint = use_state(|| "".to_string());
    let client_secret = use_state(|| "".to_string());

    // Define the initial state
    let page_state = use_state(|| PageState::Hidden);


    // Define the callback function for closing the modal
    let on_close_modal = {
        let page_state = page_state.clone();
        Callback::from(move |_| {
            page_state.set(PageState::Hidden);
        })
    };

    // Define the callback functions
    let configure_azure = {
        let page_state = page_state.clone();
        Callback::from(move |_| {
            page_state.set(PageState::Azure);
        })
    };

    let configure_github = {
        let page_state = page_state.clone();
        Callback::from(move |_| {
            page_state.set(PageState::Github);
        })
    };

    let login_endpoint = {
        let url_endpoint = url_endpoint.clone();
        Callback::from(move |e: InputEvent| {
            url_endpoint.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value());
        })
    };

    let on_client_id = {
        let client_id = client_id.clone();
        Callback::from(move |e: InputEvent| {
            client_id.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value());
        })
    };
    
    let on_client_secret = {
        let client_secret = client_secret.clone();
        Callback::from(move |e: InputEvent| {
            client_secret.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value());
        })
    };
    
    let on_tenant_id = {
        let tenant_id = tenant_id.clone();
        Callback::from(move |e: InputEvent| {
            tenant_id.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value());
        })
    };

    let azure_tenant_id = tenant_id.clone();
    let on_azure_submit = {
        let page_state = page_state.clone();
        let server_name = server_name.clone();
        let api_key = api_key.clone();
        let url_endpoint = url_endpoint.clone().to_string();
        let client_id = client_id.clone().to_string();
        let tenant_id = azure_tenant_id.clone();
        let client_secret = client_secret.clone().to_string();
        // let on_update_trigger = update_trigger.clone();
        Callback::from(move |e: MouseEvent| {
            // let error_container = error_container_create.clone();
            // let error_message_container = error_message_container_create.clone();
            // let update_trigger = on_update_trigger.clone();
            let tenant_id = tenant_id.clone();
            let call_server = server_name.clone();
            let call_api = api_key.clone();

            e.prevent_default();
            // Hash the password and generate a salt
            let tenant_id_clone = (*tenant_id).clone();

            let provider_settings = ExternalAuthRequest {
                provider: "Azure".to_string(),
                client_id: client_id.clone(),
                tenant_id: tenant_id_clone, 
                redirect_uri: url_endpoint.clone(),
                secret: client_secret.clone(),
            };
            let add_azure_request = Some(provider_settings);
            page_state.set(PageState::Hidden);
            wasm_bindgen_futures::spawn_local(async move {
                if let Some(add_azure_request_value) = add_azure_request {
                    match call_add_external_auth(call_server.unwrap(), call_api.unwrap().unwrap(), add_azure_request_value).await {
                        Ok(_success) => {
                            // on_update_trigger.set(!*update_trigger);
                            web_sys::console::log_1(&"provider added successfully".into());
                        },
                        Err(e) => {
                            web_sys::console::log_1(&format!("Error adding provider: {}", e).into());
                        },
                    }
                } else {
                    web_sys::console::log_1(&format!("Error adding provider").into());
                }
            });

        })
    };

    let on_github_submit = {
        let page_state = page_state.clone();
        let server_name = server_name.clone();
        let api_key = api_key.clone();
        let url_endpoint = url_endpoint.clone().to_string();
        let client_id = client_id.clone().to_string();
        let tenant_id = (*tenant_id).clone();
        // let on_update_trigger = update_trigger.clone();
        Callback::from(move |e: MouseEvent| {
            // let error_container = error_container_create.clone();
            // let error_message_container = error_message_container_create.clone();
            // let update_trigger = on_update_trigger.clone();
            let call_server = server_name.clone();
            let call_api = api_key.clone();

            e.prevent_default();
            // Hash the password and generate a salt

            let provider_settings = ExternalAuthRequest {
                provider: "Azure".to_string(),
                client_id: client_id.clone(),
                tenant_id: tenant_id.clone(),
                redirect_uri: url_endpoint.clone(),
                secret: "".to_string(),
            };
            let add_azure_request = Some(provider_settings);
            page_state.set(PageState::Hidden);
            wasm_bindgen_futures::spawn_local(async move {
                if let Some(add_azure_request_value) = add_azure_request {
                    match call_add_external_auth(call_server.unwrap(), call_api.unwrap().unwrap(), add_azure_request_value).await {
                        Ok(_success) => {
                            // on_update_trigger.set(!*update_trigger);
                            web_sys::console::log_1(&"provider added successfully".into());
                        },
                        Err(e) => {
                            web_sys::console::log_1(&format!("Error adding provider: {}", e).into());
                        },
                    }
                } else {
                    web_sys::console::log_1(&format!("Error adding provider").into());
                }
            });

        })
    };


    let setup_azure_modal = html! {
        <div id="create-user-modal" tabindex="-1" aria-hidden="true" class="fixed top-0 right-0 left-0 z-50 flex justify-center items-center w-full h-[calc(100%-1rem)] max-h-full bg-black bg-opacity-25">
            <div class="modal-container relative p-4 w-full max-w-md max-h-full rounded-lg shadow">
                <div class="modal-container relative rounded-lg shadow">
                    <div class="flex items-center justify-between p-4 md:p-5 border-b rounded-t">
                        <h3 class="text-xl font-semibold">
                            {"Setup Azure Auth"}
                        </h3>
                        <button onclick={on_close_modal.clone()} class="end-2.5 text-gray-400 bg-transparent hover:bg-gray-200 hover:text-gray-900 rounded-lg text-sm w-8 h-8 ms-auto inline-flex justify-center items-center dark:hover:bg-gray-600 dark:hover:text-white">
                            <svg class="w-3 h-3" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 14 14">
                                <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="m1 1 6 6m0 0 6 6M7 7l6-6M7 7l-6 6"/>
                            </svg>
                            <span class="sr-only">{"Close modal"}</span>
                        </button>
                    </div>
                    <div class="p-4 md:p-5">
                        <form class="space-y-4" action="#">
                            <div>
                                <label for="endpoint" class="block mb-2 text-sm font-medium">{"URL Endpoint"}</label>
                                <input oninput={login_endpoint.clone()} placeholder="pinepods_user1" type="text" id="username" name="username" class="search-bar-input border text-sm rounded-lg block w-full p-2.5" required=true />

                            </div>
                            <div>
                                <label for="client_id" class="block mb-2 text-sm font-medium">{"Client ID"}</label>
                                <input oninput={on_client_id.clone()} placeholder="Pinepods User" type="text" id="fullname" name="fullname" class="search-bar-input border text-sm rounded-lg block w-full p-2.5" required=true />
                            </div>
                            <div>
                                <label for="client_secret" class="block mb-2 text-sm font-medium">{"Client Secret"}</label>
                                <input oninput={on_client_secret.clone()} placeholder="user@pinepods.online" type="password" id="password" name="password" class="search-bar-input border text-sm rounded-lg block w-full p-2.5" required=true />

                            </div>
                            <div>
                                <label for="tenant_id" class="block mb-2 text-sm font-medium">{"Tenant ID"}</label>
                                <input oninput={on_tenant_id.clone()} placeholder="user@pinepods.online" type="email" id="email" name="email" class="search-bar-input border text-sm rounded-lg block w-full p-2.5" required=true />

                            </div>
                            <button onclick={on_azure_submit.clone()} class="mt-4 settings-button font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline">
                                {"Submit"}
                            </button>
                            // <button type="submit" onclick={on_create_submit} class="download-button w-full focus:ring-4 focus:outline-none font-medium rounded-lg text-sm px-5 py-2.5 text-center">{"Submit"}</button>
                        </form>
                    </div>
                </div>
            </div>
        </div>
    };

    let setup_github_modal = html! {
        <div id="create-user-modal" tabindex="-1" aria-hidden="true" class="fixed top-0 right-0 left-0 z-50 flex justify-center items-center w-full h-[calc(100%-1rem)] max-h-full bg-black bg-opacity-25">
            <div class="modal-container relative p-4 w-full max-w-md max-h-full rounded-lg shadow">
                <div class="modal-container relative rounded-lg shadow">
                    <div class="flex items-center justify-between p-4 md:p-5 border-b rounded-t">
                        <h3 class="text-xl font-semibold">
                            {"Create New User"}
                        </h3>
                        <button onclick={on_close_modal.clone()} class="end-2.5 text-gray-400 bg-transparent hover:bg-gray-200 hover:text-gray-900 rounded-lg text-sm w-8 h-8 ms-auto inline-flex justify-center items-center dark:hover:bg-gray-600 dark:hover:text-white">
                            <svg class="w-3 h-3" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 14 14">
                                <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="m1 1 6 6m0 0 6 6M7 7l6-6M7 7l-6 6"/>
                            </svg>
                            <span class="sr-only">{"Close modal"}</span>
                        </button>
                    </div>
                    <div class="p-4 md:p-5">
                        <form class="space-y-4" action="#">
                            <div>
                                <label for="endpoint" class="block mb-2 text-sm font-medium">{"URL Endpoint"}</label>
                                <input oninput={login_endpoint.clone()} placeholder="pinepods_user1" type="text" id="username" name="username" class="search-bar-input border text-sm rounded-lg block w-full p-2.5" required=true />

                            </div>
                            <div>
                                <label for="client_id" class="block mb-2 text-sm font-medium">{"Client ID"}</label>
                                <input oninput={on_client_id.clone()} placeholder="Pinepods User" type="text" id="fullname" name="fullname" class="search-bar-input border text-sm rounded-lg block w-full p-2.5" required=true />
                            </div>
                            <div>
                                <label for="tenant_id" class="block mb-2 text-sm font-medium">{"Tenant ID"}</label>
                                <input oninput={on_tenant_id.clone()} placeholder="user@pinepods.online" type="email" id="email" name="email" class="search-bar-input border text-sm rounded-lg block w-full p-2.5" required=true />

                            </div>
                            <button onclick={on_github_submit.clone()} class="mt-4 settings-button font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline">
                                {"Submit"}
                            </button>
                            // <button type="submit" onclick={on_create_submit} class="download-button w-full focus:ring-4 focus:outline-none font-medium rounded-lg text-sm px-5 py-2.5 text-center">{"Submit"}</button>
                        </form>
                    </div>
                </div>
            </div>
        </div>
    };

    html! {
        <>
            {
                match *page_state {
                // PageState::Hidden => create_user_modal,
                PageState::Azure => setup_azure_modal,
                PageState::Github => setup_github_modal,
                _ => html! {},
                }
            }
            <div class="p-4"> // You can adjust the padding as needed
                <p class="item_container-text text-lg font-bold mb-4">{"External Auth Settings:"}</p> // Styled paragraph
                <p class="item_container-text text-md mb-4">{"You can configure external login providers here. This will allow users to login utilizing external methods. "}</p> // Styled paragraph

                <p class="item_container-text text-lg font-bold mb-4">{"Currently Configured Auth Providers:"}</p>
                <div class="relative overflow-x-auto">
                    <table class="w-full text-sm text-left rtl:text-right text-gray-500 dark:text-gray-400">
                        <thead class="text-xs uppercase table-header">
                            <tr>
                                <th class="px-6 py-4">{"Provider"}</th>
                                <th class="px-6 py-4">{"Client ID"}</th>
                                <th class="px-6 py-4">{"Tenant ID"}</th>
                                <th class="px-6 py-4">{"Redirect URI"}</th>
                                <th class="px-6 py-4">{"Secret"}</th>

                            </tr>
                        </thead>
                        <tbody>
                        { 
                            for provider_values.iter().map(|setting| {
                                html! {
                                    <tr class="table-row border-b">
                                        <td class="px-6 py-4">{ &setting.provider }</td>
                                        <td class="px-6 py-4">{ &setting.client_id }</td>
                                        <td class="px-6 py-4">{ &setting.tenant_id }</td>
                                        <td class="px-6 py-4">{ &setting.redirect_uri }</td>
                                        <td class="px-6 py-4">{ &setting.secret }</td>
                                    </tr>
                                }
                            })
                        }
                        </tbody>
                    </table>
                    <div class="flex mt-4">
                    <button onclick={configure_azure} class="settings-button font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline mr-2">
                        {"Setup Azure AD Provider"}
                    </button>
                    <button onclick={configure_github} class="settings-button font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline mr-2">
                    {"Setup Github Provider"}
                </button>
                </div>
                </div>
                // <button onclick={configure_github} class="mt-4 settings-button font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline">
                //     {"Configure Github Provider"}
                // </button>
            </div>
        </>
    }
}

