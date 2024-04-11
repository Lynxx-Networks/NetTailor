use yew::prelude::*;
use wasm_bindgen::JsCast;
use yewdux::prelude::*;
use crate::components::context::AppState;
use web_sys::{window, Blob, Url, BlobPropertyBag};
use wasm_bindgen::JsValue;
use crate::requests::setting_reqs::{call_backup_user};

#[function_component(ExternalAuthSetup)]
pub fn external_auth() -> Html {
    let (state, _dispatch) = use_store::<AppState>();
    let api_key = state.auth_details.as_ref().map(|ud| ud.api_key.clone());
    let user_id = state.user_details.as_ref().map(|ud| ud.UserID.clone());
    let server_name = state.auth_details.as_ref().map(|ud| ud.server_name.clone());
        
    {
        let provider_values = email_values.clone();
        use_effect_with((api_key.clone(), server_name.clone()), move |(api_key, server_name)| {
            let email_values = email_values.clone();
            let api_key = api_key.clone();
            let server_name = server_name.clone();
            let future = async move {
                if let (Some(api_key), Some(server_name)) = (api_key, server_name) {
                    let response = call_get_email_settings(server_name, api_key.unwrap()).await;
                    match response {
                        Ok(email_info) => {
                            email_values.set(email_info);
                        },
                        Err(e) => console::log_1(&format!("Error getting user info: {}", e).into()),
                    }
                }
            };
            spawn_local(future);
            // Return cleanup function
            || {}
        });
    }


    let mut blob_property_bag = BlobPropertyBag::new();
    blob_property_bag.type_("text/xml");
    
    let configure_azure = {
        let blob_property_bag = blob_property_bag.clone();
        Callback::from(move |_| {
            let bloberty_bag = blob_property_bag.clone();
            let api_key = api_key.clone();
            let server_name = server_name.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match call_backup_user(&server_name.unwrap(), user_id.unwrap(), &api_key.unwrap().unwrap()).await {
                    Ok(opml_content) => {
                        // Wrap the OPML content in an array and convert to JsValue
                        let array = js_sys::Array::new();
                        array.push(&JsValue::from_str(&opml_content));
                        
                        // Create a new blob from the OPML content
                        let blob = Blob::new_with_str_sequence_and_options(&array, &bloberty_bag).unwrap();
                        let url = Url::create_object_url_with_blob(&blob).unwrap();
    
                        // Trigger the download
                        if let Some(window) = window() {
                            let document = window.document().unwrap();
                            let a = document.create_element("a").unwrap().dyn_into::<web_sys::HtmlAnchorElement>().unwrap();
                            a.set_href(&url);
                            a.set_download("podcasts.opml");
                            a.click();
    
                            // Revoke the object URL to free up resources
                            Url::revoke_object_url(&url).unwrap();
                        }
                    }
                    Err(e) => {
                        web_sys::console::log_1(&format!("Error exporting OPML: {:?}", e).into());
                    }
                }
            });
        })
    };



    html! {
        <div class="p-4"> // You can adjust the padding as needed
            <p class="item_container-text text-lg font-bold mb-4">{"External Auth Settings:"}</p> // Styled paragraph
            <p class="item_container-text text-md mb-4">{"You can configure external login providers here. This will allow users to login utilizing external methods. "}</p> // Styled paragraph

            <p class="item_container-text text-lg font-bold mb-4">{"Currently Configured Auth Providers:"}</p>
            <div class="relative overflow-x-auto">
                <table class="w-full text-sm text-left rtl:text-right text-gray-500 dark:text-gray-400">
                    <thead class="text-xs uppercase table-header">
                        <tr>
                            <th scope="col" class="px-6 py-3">{"External Provider"}</th>
                            <th scope="col" class="px-6 py-3">{"Setup"}</th>

                        </tr>
                    </thead>
                    <tbody>
                    {
                        // Access the state directly without let binding inside html!
                        html! {
                            <tr class="table-row border-b">
                                <td class="px-6 py-4">{ &provider_values.provider }</td>
                                <td class="px-6 py-4">{ &provider_values.setup }</td>
                            </tr>
                        }
                    }
                    </tbody>
                </table>
            </div>

            <button onclick={configure_azure} class="mt-4 settings-button font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline">
                {"Configure Azure AD Provider"}
            </button>
            // <button onclick={configure_github} class="mt-4 settings-button font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline">
            //     {"Configure Github Provider"}
            // </button>
        </div>
    }
}

