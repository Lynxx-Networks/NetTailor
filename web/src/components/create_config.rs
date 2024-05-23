use wasm_bindgen::JsValue;
use serde::{Deserialize, Serialize};
use yew::prelude::*;
use wasm_bindgen::JsCast;
use yewdux::use_store;
use crate::components::gen_funcs::{validate_config_input, get_base_url};
use web_sys::window;
use super::app_drawer::App_drawer;
use super::search_nav::Search_nav;
use crate::components::context::{AppState, UIState};
use crate::components::misc_func::generate_config;
use web_sys::{HtmlSelectElement, HtmlInputElement};
use crate::components::settings::AccordionItem;
use crate::components::settings::AccordionItemPosition;
use wasm_bindgen_futures::spawn_local;
use regex::Regex;
use crate::requests::net_requests::{DeviceInfo, send_config_to_server, add_config_db, DeviceConfig};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Client {
    name: String,
}


async fn get_clients_list() -> Result<Vec<Client>, JsValue> {
    let window = window().ok_or_else(|| JsValue::from_str("No global `window` exists"))?;
    let session_storage = window.session_storage()?.ok_or_else(|| JsValue::from_str("Failed to get session storage"))?;

    let clients_json = session_storage.get_item("clients")?.ok_or_else(|| JsValue::from_str("No clients found in session storage"))?;

    serde_json::from_str(&clients_json).map_err(|e| JsValue::from_str(&e.to_string()))
}


#[function_component(CreateConfig)]
pub fn create_config() -> Html {
    let (state, _dispatch) = use_store::<AppState>();


    let (_post_state, _post_dispatch) = use_store::<AppState>();
    let (audio_state, _audio_dispatch) = use_store::<UIState>();
    let error_message = audio_state.error_message.clone();
    let info_message = audio_state.info_message.clone();
    let api_key = state.auth_details.as_ref().map(|ud| ud.api_key.clone());
    let server_name = state.auth_details.as_ref().map(|ud| ud.server_name.clone());
    let _user_id = state.user_details.as_ref().map(|ud| ud.UserID.clone());
    let banked_configs = use_state(|| vec![]);
    let banked_vlans = use_state(|| vec![]);
    let banked_routes = use_state(|| vec![]);
    let client_names = use_state(|| Vec::new());
    let shared_link = use_state(|| String::new());

    let hostname = use_state(|| String::from(""));
    let tacacs_server = use_state(|| String::from(""));
    let ise_server = use_state(|| String::from(""));
    let timezone = use_state(|| String::from(""));
    let switch_number = use_state(|| String::from(""));
    let auvik_collector = use_state(|| String::from(""));
    let vlan_range = use_state(|| String::from(""));
    let encrypted_user_pass = use_state(|| String::from(""));
    let vlan_definitions = use_state(|| String::from(""));
    let client_name = use_state(|| String::from(""));
    let device_type = use_state(|| String::from(""));
    let service_type = use_state(|| String::from(""));
    let device_model = use_state(|| String::from(""));

    let client_domain = use_state(|| String::from(""));
    let device_ip = use_state(|| String::from(""));
    let configuration = use_state(|| String::new());
    let default_admin = use_state(|| String::from(""));
    let encrypted_enable_pass = use_state(|| String::from(""));
    let interface_definitions = use_state(|| String::from(""));
    let router_definitions = use_state(|| String::from(""));
    let ip_routes = use_state(|| String::from(""));
    let custom_snmp_config = use_state(|| String::from(""));
    let location = use_state(|| String::from(""));
    let snmp_community_string = use_state(|| String::from(""));
    let tacacs_key = use_state(|| String::from(""));
    let dns_server1 = use_state(|| String::from(""));
    let dns_server2 = use_state(|| String::from(""));

    let effect_names = client_names.clone();
    use_effect_with((), move |_| {
        let client_names = effect_names.clone();
        wasm_bindgen_futures::spawn_local(async move {
            match get_clients_list().await {
                Ok(clients) => {
                    client_names.set(clients);
                },
                Err(err) => {
                    eprintln!("Failed to fetch clients: {:?}", err);
                }
            }
        });

        || () // Cleanup logic if necessary
    });

    #[derive(Clone, PartialEq)]
    enum AuvikErrorNotice {
        Hidden,
        Shown(String),
    }

    #[derive(Clone, PartialEq)]
    enum ClientDomainErrorNotice {
        Hidden,
        Shown(String),
    }

    #[derive(Clone, PartialEq)]
    enum TacacsErrorNotice {
        Hidden,
        Shown(String),
    }

    #[derive(Clone, PartialEq)]
    enum LocationErrorNotice {
        Hidden,
        Shown(String),
    }
    #[derive(Clone, PartialEq)]
    enum TacacsKeyErrorNotice {
        Hidden,
        Shown(String),
    }
    #[derive(Clone, PartialEq)]
    enum SnmpComErrorNotice {
        Hidden,
        Shown(String),
    }
    #[derive(Clone, PartialEq)]
    enum AuthenticationErrorNotice {
        Hidden,
        Shown(String),
    }
    #[derive(Clone, PartialEq)]
    enum IseServerErrorNotice {
        Hidden,
        Shown(String),
    }
    
    let auvik_error = use_state(|| AuvikErrorNotice::Hidden);
    let client_domain_error = use_state(|| ClientDomainErrorNotice::Hidden);
    let tacacs_error = use_state(|| TacacsErrorNotice::Hidden);
    let location_error = use_state(|| LocationErrorNotice::Hidden);
    let tacacs_key_error = use_state(|| TacacsKeyErrorNotice::Hidden);
    let snmp_com_error = use_state(|| SnmpComErrorNotice::Hidden);
    let authentication_error = use_state(|| AuthenticationErrorNotice::Hidden);
    let ise_server_error = use_state(|| IseServerErrorNotice::Hidden);

    let generate_config_click = {

        let auvik_collector = auvik_collector.clone();
        let client_domain = client_domain.clone();
        let tacacs_server = tacacs_server.clone();
        let location = location.clone();
        let tacacs_key = tacacs_key.clone();
        let snmp_com = snmp_community_string.clone();
        let ise_server = ise_server.clone();

        let auvik_error = auvik_error.clone();
        let client_domain_error = client_domain_error.clone();
        let tacacs_error = tacacs_error.clone();
        let location_error = location_error.clone();
        let tacacs_key_error = tacacs_key_error.clone();
        let snmp_com_error = snmp_com_error.clone();
        let ise_server_error = ise_server_error.clone();

        let user_id = _user_id.clone();
        web_sys::console::log_1(&format!("User ID: {:?}", user_id).into());
        let server_name = server_name.clone(); // URL to your backend server
        let api_key = api_key.clone(); // API key for your backend server
        let call_config = (*configuration).clone();
        let call_hostname = (*hostname).clone();
        let call_location = (*location).clone();
        let client_name = (*client_name).clone();
        let device_type = (*device_type).clone();
        let shared_link = shared_link.clone(); 
        Callback::from(move |_: MouseEvent| {
            let auvik_collector_value = (*auvik_collector).clone();
            let client_domain_value = (*client_domain).clone();
            let tacacs_server_value = (*tacacs_server).clone();
            let location_value = (*location).clone();
            let tacacs_key_value = (*tacacs_key).clone();
            let snmp_com_value = (*snmp_com).clone();
            let ise_server_value = (*ise_server).clone();

            // Log the values to ensure they are captured correctly
            web_sys::console::log_1(&format!("Auvik Collector: {:?}", auvik_collector_value).into());
            web_sys::console::log_1(&format!("Client Domain: {:?}", client_domain_value).into());
            // ... Add logs for other fields similarly

            web_sys::console::log_1(&format!("User ID: {:?}", user_id).into());
            let api_key = api_key.clone();
            let server_name = server_name.clone();
            let server_name = server_name.clone();
            let shared_link = shared_link.clone();
            let mut base_url = String::new();
            match get_base_url() {
                Ok(url) => {
                    base_url = url;
                }
                Err(e) => {
                    web_sys::console::log_1(&format!("Error getting base URL: {}", e).into());
                }
            }

            let validation_result = validate_config_input(
                &auvik_collector_value,
                &client_domain_value,
                &tacacs_server_value,
                &location_value,
                &tacacs_key_value,
                &snmp_com_value,
                &ise_server_value,
            );
    
            match validation_result {
                Ok(_) => {
                    auvik_error.set(AuvikErrorNotice::Hidden);
                    client_domain_error.set(ClientDomainErrorNotice::Hidden);
                    tacacs_error.set(TacacsErrorNotice::Hidden);
                    location_error.set(LocationErrorNotice::Hidden);
                    tacacs_key_error.set(TacacsKeyErrorNotice::Hidden);
                    snmp_com_error.set(SnmpComErrorNotice::Hidden);
                    ise_server_error.set(IseServerErrorNotice::Hidden);
                }
                Err(errors) => {
                    // Log the errors to ensure they are captured
                    web_sys::console::log_1(&format!("Validation Errors: {:?}", errors).into());
    
                    for error in errors {
                        if error.contains("Auvik Collector") {
                            auvik_error.set(AuvikErrorNotice::Shown(error.clone()));
                        }
    
                        if error.contains("Client domain") {
                            client_domain_error.set(ClientDomainErrorNotice::Shown(error.clone()));
                        }
    
                        if error.contains("TACACS server") {
                            tacacs_error.set(TacacsErrorNotice::Shown(error.clone()));
                        }
    
                        if error.contains("Location") {
                            location_error.set(LocationErrorNotice::Shown(error.clone()));
                        }
    
                        if error.contains("TACACS key") {
                            tacacs_key_error.set(TacacsKeyErrorNotice::Shown(error.clone()));
                        }
    
                        if error.contains("SNMP community string") {
                            snmp_com_error.set(SnmpComErrorNotice::Shown(error.clone()));
                        }
    
                        if error.contains("ISE server") {
                            ise_server_error.set(IseServerErrorNotice::Shown(error.clone()));
                        }
                    }
                    return;
                }
            }

            // if errors.contains(&ValidationError::InvalidEmail) {
            //     email_error.set(email_error_notice::Shown);
            // } else {
            //     email_error.set(email_error_notice::Hidden);
            // }

            // Example device config, replace this with actual data collection logic
            let device_config = DeviceConfig {
                user_id: user_id.clone().unwrap(),
                hostname: call_hostname.clone(),
                config: call_config.clone(),
            };



            let device_info = DeviceInfo {
                user_id: user_id.clone().unwrap(),
                device_hostname: call_hostname.clone(),
                location: call_location.clone(),
                client_name: client_name.clone(),
                device_type: device_type.clone(),
                config_name: call_hostname.clone(),
                url: base_url.clone(),
            };
    
            let future = async move {
                match add_config_db(&server_name.clone().unwrap(), &device_info, &api_key.clone().unwrap()).await {
                    Ok(config_response) => {
                        shared_link.set(config_response.shared_link.clone());  // Set the shared link
                        // web_sys::console::log_1(&format!("accesslink here: {}", config_response.shared_link).into());
                        let result = send_config_to_server(
                            &server_name.unwrap(),
                            config_response.config_id,
                            &device_config,
                            &config_response.storage_location,
                            &api_key.unwrap(),
                        )
                        .await;
        
                        match result {
                            Ok(message) => {
                                web_sys::console::log_1(&format!("Success: {}", message).into());
                            }
                            Err(e) => {
                                web_sys::console::log_1(&format!("Error sending config to server: {}", e).into());
                            }
                        }
                    },
                    Err(e) => {
                        web_sys::console::log_1(&format!("Error adding config to DB: {}", e).into());
                    }
                }
            };
    
            spawn_local(future);
        })
    };


    #[derive(PartialEq, Clone)]
    struct ConfigDependencies {
        hostname: String,
        tacacs_server: String,
        ise_server: String,
        timezone: String,
        switch_number: String,
        auvik_collector: String,
        vlan_range: String,
        encrypted_user_pass: String,
        vlan_definitions: String,
        client_name: String,
        device_type: String,
        device_model: String,
        client_domain: String,
        device_ip: String,
        default_admin: String,
        encrypted_enable_pass: String,
        interface_definitions: String,
        router_definitions: String,
        ip_routes: String,
        custom_snmp_config: String,
        location: String,
        snmp_community_string: String,
        tacacs_key: String,
        dns_server1: String,
        dns_server2: String,
    }

    let dependencies = ConfigDependencies {
        hostname: (*hostname).clone(),
        tacacs_server: (*tacacs_server).clone(),
        ise_server: (*ise_server).clone(),
        timezone: (*timezone).clone(),
        switch_number: (*switch_number).clone(),
        auvik_collector: (*auvik_collector).clone(),
        vlan_range: (*vlan_range).clone(),
        encrypted_user_pass: (*encrypted_user_pass).clone(),
        vlan_definitions: (*vlan_definitions).clone(),
        client_name: (*client_name).clone(),
        device_type: (*device_type).clone(),
        device_model: (*device_model).clone(),
        client_domain: (*client_domain).clone(),
        device_ip: (*device_ip).clone(),
        default_admin: (*default_admin).clone(),
        encrypted_enable_pass: (*encrypted_enable_pass).clone(),
        interface_definitions: (*interface_definitions).clone(),
        router_definitions: (*router_definitions).clone(),
        ip_routes: (*ip_routes).clone(),
        custom_snmp_config: (*custom_snmp_config).clone(),
        location: (*location).clone(),
        snmp_community_string: (*snmp_community_string).clone(),
        tacacs_key: (*tacacs_key).clone(),
        dns_server1: (*dns_server1).clone(),
        dns_server2: (*dns_server2).clone(),

    };

    let effect_configuration = configuration.clone();

    use_effect_with(dependencies, move |deps| {
        let config = generate_config(
            &deps.hostname, 
            &deps.tacacs_server,
            &deps.ise_server,
            &deps.timezone,
            &deps.switch_number,
            &deps.auvik_collector,
            &deps.vlan_range,
            &deps.encrypted_user_pass,
            &deps.vlan_definitions,
            &deps.client_name,
            &deps.device_type,
            &deps.device_model,
            &deps.client_domain,
            &deps.device_ip,
            &deps.default_admin,
            &deps.encrypted_enable_pass,
            &deps.interface_definitions,
            &deps.router_definitions,
            &deps.ip_routes,
            &deps.custom_snmp_config,
            &deps.location,
            &deps.snmp_community_string,
            &deps.tacacs_key,
            &deps.dns_server1,
            &deps.dns_server2,

        );

        effect_configuration.set(config);
        || ()  // Cleanup, if necessary
    });

    let client_domain_block = html! {
        <div class="config-form">
            <div class="input-field" style="display: flex; align-items: center; justify-content: flex-start;">
                <label style="margin-right: 10px;">{"Client Domain:"}</label>
                <input
                    type="text"
                    class="email-input border p-2 ml-2 rounded"
                    placeholder="myclient.com"
                    value={(*client_domain).clone()}
                    oninput={{
                        let client_domain = client_domain.clone();
                        Callback::from(move |e: InputEvent| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            client_domain.set(input.value());
                        })
                    }}
                />
                <div>
                    {
                        match &*client_domain_error {
                            ClientDomainErrorNotice::Hidden => html! {},
                            ClientDomainErrorNotice::Shown(msg) => html! {<p class="text-red-500 text-xs italic">{ msg }</p>},
                        }
                    }
                </div>
            </div>
        </div>
    };
    
    let default_admin_block = html! {
        <div class="config-form">
            <div class="input-field" style="display: flex; align-items: center; justify-content: flex-start;">
                <label style="margin-right: 10px;">{"Default Admin:"}</label>
                <input
                    type="text"
                    class="email-input border p-2 ml-2 rounded"
                    placeholder="Default Admin"
                    value={(*default_admin).clone()}
                    oninput={{
                        let default_admin = default_admin.clone();
                        Callback::from(move |e: InputEvent| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            default_admin.set(input.value());
                        })
                    }}
                />
            </div>
        </div>
    };

    let encrypted_enable_pass_block = html! {
        <div class="config-form">
            <div class="input-field" style="display: flex; align-items: center; justify-content: flex-start;">
                <label style="margin-right: 10px;">{"Enable Pass:"}</label>
                <input
                    type="text"
                    class="email-input border p-2 ml-2 rounded"
                    placeholder="MySecurePassword!"
                    value={(*encrypted_enable_pass).clone()}
                    oninput={{
                        let encrypted_enable_pass = encrypted_enable_pass.clone();
                        Callback::from(move |e: InputEvent| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            encrypted_enable_pass.set(input.value());
                        })
                    }}
                />
            </div>
        </div>
    };

    let device_ip_block = html! {
        <div class="config-form">
            <div class="input-field" style="display: flex; align-items: center; justify-content: flex-start;">
                <label style="margin-right: 10px;">{"Device IP:"}</label>
                <input
                    type="text"
                    class="email-input border p-2 ml-2 rounded"
                    placeholder="Loopback IP"
                    value={(*device_ip).clone()}
                    oninput={{
                        let device_ip = device_ip.clone();
                        Callback::from(move |e: InputEvent| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            device_ip.set(input.value());
                        })
                    }}
                />
            </div>
        </div>
    };

    let hostname_block = html! {
        <div class="config-form">
            <div class="input-field" style="display: flex; align-items: center; justify-content: flex-start;">
                <label style="margin-right: 10px;">{"Hostname:"}</label>
                <input
                    type="text"
                    class="email-input border p-2 ml-2 rounded"
                    placeholder="Hostname"
                    value={(*hostname).clone()}
                    oninput={{
                        let hostname = hostname.clone();
                        Callback::from(move |e: InputEvent| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            hostname.set(input.value());
                        })
                    }}
                />
            </div>
        </div>
    };


    let tacacs_server_block = html! {
        <div class="config-form">
            <div class="input-field" style="display: flex; align-items: center; justify-content: flex-start;">
                <label style="margin-right: 10px;">{"Tacacs Server:"}</label>
                <input
                    type="text"
                    class="email-input border p-2 ml-2 rounded"
                    placeholder="tacacs.hoth.com"
                    value={(*tacacs_server).clone()}
                    oninput={{
                        let tacacs_server = tacacs_server.clone();
                        Callback::from(move |e: InputEvent| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            tacacs_server.set(input.value());
                        })
                    }}
                />
            </div>
        </div>
    };

    let ise_server_block = html! {
        <div class="config-form">
            <div class="input-field" style="display: flex; align-items: center; justify-content: flex-start;">
                <label style="margin-right: 10px;">{"Authentication Server:"}</label>
                <input
                    type="text"
                    class="email-input border p-2 ml-2 rounded"
                    placeholder="1.2.3.4"
                    value={(*ise_server).clone()}
                    oninput={{
                        let ise_server = ise_server.clone();
                        Callback::from(move |e: InputEvent| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            ise_server.set(input.value());
                        })
                    }}
                />
            </div>
        </div>
    };

    let timezones = vec![
        ("Eastern", "EST -5 0"),
        ("Central", "CST -6 0"),
        ("Mountain", "MST -7 0"),
        ("Pacific", "PST -8 0"),
        ("Alaska", "AKST -9 0"),
        ("Hawaii", "HST -10 0"),
    ];
    
    let timezone_block = html! {
        <div class="config-form">
            <div class="input-field" style="display: flex; align-items: center; justify-content: flex-start;">
                <label style="margin-right: 10px;">{"Timezone:"}</label>
                <select
                    class="email-input border p-2 ml-2 rounded"
                    value={(*timezone).clone()}
                    onchange={{
                        let timezone = timezone.clone();
                        Callback::from(move |e: Event| {
                            let select: HtmlSelectElement = e.target_unchecked_into();
                            timezone.set(select.value());
                        })
                    }}
                >
                    <option value="" disabled=true selected=true hidden=true>{"Select Timezone"}</option>
                    { for timezones.iter().map(|(name, value)| html! {
                        <option value={value.clone()}>{name}</option>
                    }) }
                </select>
            </div>
        </div>
    };
    

    let auvik_collector_block = html! {
        <div class="config-form">
            <div class="input-field" style="display: flex; align-items: center; justify-content: flex-start;">
                <label style="margin-right: 10px;">{"Auvik Collector:"}</label>
                <input
                    type="text"
                    class="email-input border p-2 ml-2 rounded"
                    placeholder="1.2.3.4"
                    value={(*auvik_collector).clone()}
                    oninput={{
                        let auvik_collector = auvik_collector.clone();
                        Callback::from(move |e: InputEvent| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            auvik_collector.set(input.value());
                        })
                    }}
                />
                    <div>
                        {
                            match &*auvik_error {
                                AuvikErrorNotice::Hidden => html! {},
                                AuvikErrorNotice::Shown(msg) => html! {<p class="text-red-500 text-xs italic">{ msg }</p>},
                            }
                        }
                    </div>
                
            </div>
        </div>
    };

    let vlan_range_block = html! {
        <div class="config-form">
            <div class="input-field" style="display: flex; align-items: center; justify-content: flex-start;">
                <label style="margin-right: 10px;">{"Spanning-tree VLAN Range:"}</label>
                <input
                    type="text"
                    class="email-input border p-2 ml-2 rounded"
                    placeholder="100-105"
                    value={(*vlan_range).clone()}
                    oninput={{
                        let vlan_range = vlan_range.clone();
                        Callback::from(move |e: InputEvent| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            vlan_range.set(input.value());
                        })
                    }}
                />
            </div>
        </div>
    };

    let encrypted_user_pass_block = html! {
        <div class="config-form">
            <div class="input-field" style="display: flex; align-items: center; justify-content: flex-start;">
                <label style="margin-right: 10px;">{"User Pass:"}</label>
                <input
                    type="text"
                    class="email-input border p-2 ml-2 rounded"
                    placeholder="ExtraSecurePassword!"
                    value={(*encrypted_user_pass).clone()}
                    oninput={{
                        let encrypted_user_pass = encrypted_user_pass.clone();
                        Callback::from(move |e: InputEvent| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            encrypted_user_pass.set(input.value());
                        })
                    }}
                />
            </div>
        </div>
    };

    let location_block = html! {
        <div class="config-form">
            <div class="input-field" style="display: flex; align-items: center; justify-content: flex-start;">
                <label style="margin-right: 10px;">{"Location:"}</label>
                <input
                    type="text"
                    class="email-input border p-2 ml-2 rounded"
                    placeholder="West Wing Death Star"
                    value={(*location).clone()}
                    oninput={{
                        let location = location.clone();
                        Callback::from(move |e: InputEvent| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            location.set(input.value());
                        })
                    }}
                />
            </div>
        </div>
    };

    let snmp_community_string_block = html! {
        <div class="config-form">
            <div class="input-field" style="display: flex; align-items: center; justify-content: flex-start;">
                <label style="margin-right: 10px;">{"SNMP Community String:"}</label>
                <input
                    type="text"
                    class="email-input border p-2 ml-2 rounded"
                    placeholder="SNMP Community String"
                    value={(*snmp_community_string).clone()}
                    oninput={{
                        let snmp_community_string = snmp_community_string.clone();
                        Callback::from(move |e: InputEvent| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            snmp_community_string.set(input.value());
                        })
                    }}
                />
            </div>
        </div>
    };

    let tacacs_key_block = html! {
        <div class="config-form">
            <div class="input-field" style="display: flex; align-items: center; justify-content: flex-start;">
                <label style="margin-right: 10px;">{"Tacacs Key:"}</label>
                <input
                    type="text"
                    class="email-input border p-2 ml-2 rounded"
                    placeholder="123456789"
                    value={(*tacacs_key).clone()}
                    oninput={{
                        let tacacs_key = tacacs_key.clone();
                        Callback::from(move |e: InputEvent| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            tacacs_key.set(input.value());
                        })
                    }}
                />
            </div>
        </div>
    };

    let dns_server1_block = html! {
        <div class="config-form">
            <div class="input-field" style="display: flex; align-items: center; justify-content: flex-start;">
                <label style="margin-right: 10px;">{"DNS Server 1:"}</label>
                <input
                    type="text"
                    class="email-input border p-2 ml-2 rounded"
                    placeholder="DNS Server 1"
                    value={(*dns_server1).clone()}
                    oninput={{
                        let dns_server1 = dns_server1.clone();
                        Callback::from(move |e: InputEvent| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            dns_server1.set(input.value());
                        })
                    }}
                />
            </div>
        </div>
    };

    let dns_server2_block = html! {
        <div class="config-form">
            <div class="input-field" style="display: flex; align-items: center; justify-content: flex-start;">
                <label style="margin-right: 10px;">{"DNS Server 2:"}</label>
                <input
                    type="text"
                    class="email-input border p-2 ml-2 rounded"
                    placeholder="DNS Server 2"
                    value={(*dns_server2).clone()}
                    oninput={{
                        let dns_server2 = dns_server2.clone();
                        Callback::from(move |e: InputEvent| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            dns_server2.set(input.value());
                        })
                    }}
                />
            </div>
        </div>
    };

    let on_client_name_change = {
        let client_name = client_name.clone();
        Callback::from(move |e: InputEvent| {
            let select: HtmlSelectElement = e.target_unchecked_into();
            client_name.set(select.value());
        })
    };

    let client_name_block = html! {
        <div class="config-form">
            <div class="input-field" style="display: flex; align-items: center; justify-content: flex-start;">
                <label style="margin-right: 10px;">{"Client Name:"}</label>
                <select class="email-input border p-2 ml-2 rounded" oninput={on_client_name_change}>
                    <option value="" disabled=true selected=true hidden=true>{"Select Client"}</option>
                    {for client_names.iter().map(|client| html! {
                        <option value={client.name.clone()}>{&client.name}</option>
                    })}
                </select>
            </div>
        </div>
    };
    

    let on_device_type_change = {
        let device_type = device_type.clone();
        Callback::from(move |e: InputEvent| {
            let select: HtmlSelectElement = e.target_unchecked_into();
            device_type.set(select.value());
        })
    };

    let device_type_block = html! {
        <div class="config-form">
            <div class="input-field" style="display: flex; align-items: center; justify-content: flex-start;">
                <label style="margin-right: 10px;">{"Device Type:"}</label>
                <select
                    class="email-input border p-2 ml-2 rounded"
                    value={(*device_type).clone()}
                    oninput={on_device_type_change.clone()}
                >
                    <option value="" disabled=true selected=true hidden=true>{"Select Device Type"}</option>
                    <option value="Switch">{"Switch"}</option>
                    <option value="Router">{"Router"}</option>
                    // Add more options as needed
                </select>
            </div>
        </div>
    };

    let on_service_type_change = {
        let service_type = service_type.clone();
        Callback::from(move |e: InputEvent| {
            let select: HtmlSelectElement = e.target_unchecked_into();
            service_type.set(select.value());
        })
    };

    let service_type_block = html! {
        <div class="config-form">
            <div class="input-field" style="display: flex; align-items: center; justify-content: flex-start;">
                <label style="margin-right: 10px;">{"Service Type:"}</label>
                <select
                    class="email-input border p-2 ml-2 rounded"
                    value={(*service_type).clone()}
                    oninput={on_service_type_change.clone()}
                    >
                    {
                        if *device_type == "Switch" {
                            html! {
                                <>
                                    <option value="" disabled=true selected=true hidden=true>{"Select Service Type"}</option>
                                    <option value="access">{"Access"}</option>
                                    <option value="distribution">{"Distribution"}</option>
                                    <option value="core">{"Core"}</option>
                                </>
                            }
                        } else if *device_type == "Router" {
                            html! {
                                <>
                                    <option value="" disabled=true selected=true hidden=true>{"Select Service Type"}</option>
                                    <option value="voip">{"VoIP"}</option>
                                    <option value="dmvpn">{"DMVPN"}</option>
                                    <option value="none">{"None (Edge)"}</option>
                                </>
                            }
                        } else {
                            html! {
                                <option value="">{"Select Service Type"}</option>
                            }
                        }
                    }
                </select>
            </div>
        </div>
    };

    let on_device_model_change = {
        let device_model = device_model.clone();
        Callback::from(move |e: InputEvent| {
            let select: HtmlSelectElement = e.target_unchecked_into();
            device_model.set(select.value());
        })
    };

    let device_model_block = html! {
        <div class="config-form">
            <div class="input-field" style="display: flex; align-items: center; justify-content: flex-start;">
                <label style="margin-right: 10px;">{"Service Type:"}</label>
                <select
                    class="email-input border p-2 ml-2 rounded"
                    value={(*device_model).clone()}
                    oninput={on_device_model_change.clone()}
                    >
                    {
                        if *device_type == "Switch" {
                            html! {
                                <>
                                    <option value="" disabled=true selected=true hidden=true>{"Select Device Model"}</option>
                                    <option value="access">{"3750"}</option>
                                </>
                            }
                        } else if *device_type == "Router" {
                            html! {
                                <>
                                    <option value="" disabled=true selected=true hidden=true>{"Select Device Model"}</option>
                                    <option value="voip">{"rmodel1"}</option>
                                    <option value="dmvpn">{"rmodel2"}</option>
                                    <option value="none">{"rmodel3"}</option>
                                </>
                            }
                        } else {
                            html! {
                                <option value="">{"Select Model Type"}</option>
                            }
                        }
                    }
                </select>
            </div>
        </div>
    };


    let apply_configuration = {
        let banked_configs = banked_configs.clone();
        Callback::from(move |_: yew::MouseEvent| {
            let window = web_sys::window().expect("no global `window` exists");
            let document = window.document().expect("should have a document on window");
    
            let start_port = document
                .get_element_by_id("start-port")
                .expect("should have #start-port on the page")
                .dyn_into::<HtmlInputElement>()
                .unwrap()
                .value_as_number() as usize;
    
            let end_port = document
                .get_element_by_id("end-port")
                .expect("should have #end-port on the page")
                .dyn_into::<HtmlInputElement>()
                .unwrap()
                .value_as_number() as usize;
    
            let description = document
                .get_element_by_id("description")
                .expect("should have #description on the page")
                .dyn_into::<HtmlInputElement>()
                .unwrap()
                .value();
    
            let mode = document
                .get_element_by_id("mode")
                .expect("should have #mode on the page")
                .dyn_into::<HtmlSelectElement>()
                .unwrap()
                .value();
    
            let access_vlan = document
                .get_element_by_id("access-vlan")
                .expect("should have #access-vlan on the page")
                .dyn_into::<HtmlInputElement>()
                .unwrap()
                .value_as_number();
    
            let voice_vlan = document
                .get_element_by_id("voice-vlan")
                .expect("should have #voice-vlan on the page")
                .dyn_into::<HtmlInputElement>()
                .unwrap()
                .value_as_number();
    
            let portfast = document
                .get_element_by_id("portfast")
                .expect("should have #portfast on the page")
                .dyn_into::<HtmlInputElement>()
                .unwrap()
                .checked();
    
            let bpduguard = document
                .get_element_by_id("bpduguard")
                .expect("should have #bpduguard on the page")
                .dyn_into::<HtmlInputElement>()
                .unwrap()
                .checked();
    
            let nonegotiate = document
                .get_element_by_id("nonegotiate")
                .expect("should have #nonegotiate on the page")
                .dyn_into::<HtmlInputElement>()
                .unwrap()
                .checked();
    
            let config = generate_interface_configuration(start_port, end_port, &description, &mode, access_vlan, voice_vlan, portfast, bpduguard, nonegotiate);
            
            banked_configs.set({
                let mut configs = (*banked_configs).clone();
                configs.push(config);
                configs
            });
        })
    };

    fn generate_interface_configuration(start_port: usize, end_port: usize, description: &str, mode: &str, access_vlan: f64, voice_vlan: f64, portfast: bool, bpduguard: bool, nonegotiate: bool) -> String {
        let mut config = format!(
            "interface range GigabitEthernet1/0/{}-{}\n description {}\n switchport mode {}\n",
            start_port, end_port, description, mode
        );
    
        if mode == "access" {
            config.push_str(&format!(" switchport access vlan {}\n", access_vlan));
            if voice_vlan > 0.0 {
                config.push_str(&format!(" switchport voice vlan {}\n", voice_vlan));
            }
        }
    
        if portfast {
            config.push_str(" spanning-tree portfast\n");
        }
        if bpduguard {
            config.push_str(" spanning-tree bpduguard enable\n");
        }
        if nonegotiate {
            config.push_str(" switchport nonegotiate\n");
        }
        config.push_str("!\n");
    
        config
    }
    // let apply_configuration = Callback::from(|event: yew::MouseEvent| apply_configuration_to_ports(event));

    // let apply_configuration = {
    //     let interface_definitions = interface_definitions.clone();
    //     Callback::from(move |event: yew::MouseEvent| apply_configuration_to_ports(event, interface_definitions.clone()))
    // };
    
    let add_to_config = {
        let banked_configs = banked_configs.clone();
        let interface_definitions = interface_definitions.clone();
        Callback::from(move |_| {
            let final_config = (*banked_configs).join("\n");
            interface_definitions.set(final_config.clone());
            // Add code to use final_config in your final configuration
            web_sys::console::log_1(&final_config.into());
            // Clear banked configurations after adding to the final config
            banked_configs.set(vec![]);
        })
    };
    let remove_config = {
        let banked_configs = banked_configs.clone();
        Callback::from(move |index: usize| {
            banked_configs.set({
                let mut configs = (*banked_configs).clone();
                configs.remove(index);
                configs
            });
        })
    };

    let interface_configuration_block = html! {
        <div class="config-form">
            <div class="input-field">
                <label>{"Start Port:"}</label>
                <input type="number" class="email-input border p-2 ml-2 rounded" id="start-port" placeholder="1"/>
            </div>
            <div class="input-field">
                <label>{"End Port:"}</label>
                <input type="number" class="email-input border p-2 ml-2 rounded" id="end-port" placeholder="48"/>
            </div>
            <div class="input-field">
                <label>{"Description:"}</label>
                <input type="text" class="email-input border p-2 ml-2 rounded" id="description" placeholder="Access Port Only!"/>
            </div>
            <div class="input-field">
                <label>{"Mode:"}</label>
                <select class="email-input border p-2 ml-2 rounded" id="mode">
                    <option value="access">{"Access"}</option>
                    <option value="trunk">{"Trunk"}</option>
                    <option value="dynamic auto">{"Dynamic Auto"}</option>
                    <option value="dynamic desirable">{"Dynamic Desirable"}</option>
                    <option value="nonegotiate">{"Nonegotiate"}</option>
                </select>
            </div>
            <div class="input-field">
                <label>{"Access VLAN:"}</label>
                <input type="number" class="email-input border p-2 ml-2 rounded" id="access-vlan" placeholder="100"/>
            </div>
            <div class="input-field">
                <label>{"Voice VLAN:"}</label>
                <input type="number" class="email-input border p-2 ml-2 rounded" id="voice-vlan" placeholder="102"/>
            </div>
            <div class="input-field">
                <input type="checkbox" id="portfast" />
                <label for="portfast">{"Spanning-tree Portfast"}</label>
            </div>
            <div class="input-field">
                <input type="checkbox" id="bpduguard" />
                <label for="bpduguard">{"Spanning-tree BPDU Guard"}</label>
            </div>
            <div class="input-field">
                <input type="checkbox" id="nonegotiate" />
                <label for="nonegotiate">{"Switchport Nonegotiate"}</label>
            </div>
            <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-1 px-2 mt-3 rounded" onclick={apply_configuration}>{"Apply to bank"}</button>
        </div>
    };

    let banked_config_list = banked_configs.iter().enumerate().map(|(index, config)| {
        html! {
            <div class="code-block">
                <h3>{"Banked Interface Configuration:"}</h3>
                <pre>{config}</pre>
                <button class="bg-red-500 hover:bg-red-700 text-white font-bold py-1 px-2 mt-3 rounded" onclick={remove_config.reform(move |_| index)}>{"Remove"}</button>
            </div>
        }
    }).collect::<Html>();
    
    
    let interface_setup_accordion_item = if *device_type == "Switch" || *device_type == "Router" {
        html! {
            <AccordionItem title="Interface Setup" content={html!{
                <div class="config-form">
                    {interface_configuration_block}
                    <div class="config-list">
                        {banked_config_list}
                        <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 mt-6 rounded" onclick={add_to_config}>{"Add Current Bank to Config"}</button>
                    </div>
                </div>
            }} position={AccordionItemPosition::Middle}/>
        }
    } else {
        html! {}
    };

    let apply_vlan_configuration = {
        let banked_vlans = banked_vlans.clone();
        Callback::from(move |_: yew::MouseEvent| {
            let window = web_sys::window().expect("no global `window` exists");
            let document = window.document().expect("should have a document on window");
    
            let vlan_number = document
                .get_element_by_id("vlan-number")
                .expect("should have #vlan-number on the page")
                .dyn_into::<HtmlInputElement>()
                .unwrap()
                .value();
    
            let vlan_description = document
                .get_element_by_id("vlan-description")
                .expect("should have #vlan-description on the page")
                .dyn_into::<HtmlInputElement>()
                .unwrap()
                .value();
    
            let vlan_ip = document
                .get_element_by_id("vlan-ip")
                .expect("should have #vlan-ip on the page")
                .dyn_into::<HtmlInputElement>()
                .unwrap()
                .value();
    
            let vlan_subnet = document
                .get_element_by_id("vlan-subnet")
                .expect("should have #vlan-subnet on the page")
                .dyn_into::<HtmlInputElement>()
                .unwrap()
                .value();
    
            let dhcp_server1 = document
                .get_element_by_id("dhcp-server1")
                .expect("should have #dhcp-server1 on the page")
                .dyn_into::<HtmlInputElement>()
                .unwrap()
                .value();
    
            let dhcp_server2 = document
                .get_element_by_id("dhcp-server2")
                .expect("should have #dhcp-server2 on the page")
                .dyn_into::<HtmlInputElement>()
                .unwrap()
                .value();
    
            let config = generate_vlan_configuration(&vlan_number, &vlan_description, &vlan_ip, &vlan_subnet, &dhcp_server1, &dhcp_server2);
    
            banked_vlans.set({
                let mut vlans = (*banked_vlans).clone();
                vlans.push(config);
                vlans
            });
        })
    };
    
    fn generate_vlan_configuration(vlan_number: &str, vlan_description: &str, vlan_ip: &str, vlan_subnet: &str, dhcp_server1: &str, dhcp_server2: &str) -> String {
        format!(
            "interface Vlan{}\n description {}\n ip address {} {}\n ip helper-address {}\n ip helper-address {}\n!",
            vlan_number, vlan_description, vlan_ip, vlan_subnet, dhcp_server1, dhcp_server2
        )
    }
    
    let add_vlans_to_config = {
        let banked_vlans = banked_vlans.clone();
        let vlan_definitions = vlan_definitions.clone();
        Callback::from(move |_| {
            let final_vlan_config = (*banked_vlans).join("\n");
            vlan_definitions.set(final_vlan_config.clone());
            // Add code to use final_vlan_config in your final configuration
            web_sys::console::log_1(&final_vlan_config.into());
            // Clear banked VLAN configurations after adding to the final config
            banked_vlans.set(vec![]);
        })
    };
    
    let remove_vlan_config = {
        let banked_vlans = banked_vlans.clone();
        Callback::from(move |index: usize| {
            banked_vlans.set({
                let mut vlans = (*banked_vlans).clone();
                vlans.remove(index);
                vlans
            });
        })
    };

    let vlan_configuration_block = html! {
        <div class="config-form">
            <div class="input-field">
                <label>{"VLAN Number:"}</label>
                <input type="text" class="email-input border p-2 ml-2 rounded" id="vlan-number" placeholder="10"/>
            </div>
            <div class="input-field">
                <label>{"Description:"}</label>
                <input type="text" class="email-input border p-2 ml-2 rounded" id="vlan-description" placeholder="VLAN Description"/>
            </div>
            <div class="input-field">
                <label>{"IP Address:"}</label>
                <input type="text" class="email-input border p-2 ml-2 rounded" id="vlan-ip" placeholder="192.168.1.1"/>
            </div>
            <div class="input-field">
                <label>{"Subnet Mask:"}</label>
                <input type="text" class="email-input border p-2 ml-2 rounded" id="vlan-subnet" placeholder="255.255.255.0"/>
            </div>
            <div class="input-field">
                <label>{"DHCP Server 1:"}</label>
                <input type="text" class="email-input border p-2 ml-2 rounded" id="dhcp-server1" placeholder="192.168.1.2"/>
            </div>
            <div class="input-field">
                <label>{"DHCP Server 2:"}</label>
                <input type="text" class="email-input border p-2 ml-2 rounded" id="dhcp-server2" placeholder="192.168.1.3"/>
            </div>
            <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-1 px-2 mt-3 rounded" onclick={apply_vlan_configuration}>{"Apply to bank"}</button>
        </div>
    };
    
    let banked_vlan_list = banked_vlans.iter().enumerate().map(|(index, config)| {
        html! {
            <div class="code-block">
                <h3>{"Banked VLAN Configuration:"}</h3>
                <pre>{config}</pre>
                <button class="bg-red-500 hover:bg-red-700 text-white font-bold py-1 px-2 mt-3 rounded" onclick={remove_vlan_config.reform(move |_| index)}>{"Remove"}</button>
            </div>
        }
    }).collect::<Html>();

    let vlan_setup_accordion_item = if *device_type == "Switch"{ 
            html! {
            <AccordionItem title="VLAN Setup" content={html!{
                <div>
                    {vlan_configuration_block}
                    <div class="config-list">
                        {banked_vlan_list}
                        <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 mt-6 rounded" onclick={add_vlans_to_config}>{"Add to Config"}</button>
                    </div>
                </div>
            }} position={AccordionItemPosition::Middle}/>
        }
    } else {
        html! {}
    };

    let apply_route_configuration = {
        let banked_routes = banked_routes.clone();
        Callback::from(move |_: yew::MouseEvent| {
            let window = web_sys::window().expect("no global `window` exists");
            let document = window.document().expect("should have a document on window");
    
            let network = document
                .get_element_by_id("network")
                .expect("should have #network on the page")
                .dyn_into::<HtmlInputElement>()
                .unwrap()
                .value();
    
            let subnet = document
                .get_element_by_id("subnet")
                .expect("should have #subnet on the page")
                .dyn_into::<HtmlInputElement>()
                .unwrap()
                .value();
    
            let ip_address = document
                .get_element_by_id("ip-address")
                .expect("should have #ip-address on the page")
                .dyn_into::<HtmlInputElement>()
                .unwrap()
                .value();
    
            let config = generate_ip_route_configuration(&network, &subnet, &ip_address);
    
            banked_routes.set({
                let mut routes = (*banked_routes).clone();
                routes.push(config);
                routes
            });
        })
    };
    
    fn generate_ip_route_configuration(network: &str, subnet: &str, ip_address: &str) -> String {
        format!(
            "ip route {} {} {}",
            network, subnet, ip_address
        )
    }
    
    let add_routes_to_config = {
        let banked_routes = banked_routes.clone();
        let ip_routes = ip_routes.clone();
        Callback::from(move |_| {
            let final_route_config = (*banked_routes).join("\n");
            ip_routes.set(final_route_config.clone());
            // Add code to use final_route_config in your final configuration
            web_sys::console::log_1(&final_route_config.into());
            // Clear banked route configurations after adding to the final config
            banked_routes.set(vec![]);
        })
    };
    
    let remove_route_config = {
        let banked_routes = banked_routes.clone();
        Callback::from(move |index: usize| {
            banked_routes.set({
                let mut routes = (*banked_routes).clone();
                routes.remove(index);
                routes
            });
        })
    };

    let route_configuration_block = html! {
        <div class="config-form">
            <div class="input-field">
                <label>{"Network:"}</label>
                <input type="text" class="email-input border p-2 ml-2 rounded" id="network" placeholder="172.20.102.0"/>
            </div>
            <div class="input-field">
                <label>{"Subnet:"}</label>
                <input type="text" class="email-input border p-2 ml-2 rounded" id="subnet" placeholder="255.255.255.0"/>
            </div>
            <div class="input-field">
                <label>{"IP Address:"}</label>
                <input type="text" class="email-input border p-2 ml-2 rounded" id="ip-address" placeholder="172.20.0.49"/>
            </div>
            <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-1 px-2 mt-3 rounded" onclick={apply_route_configuration}>{"Apply to bank"}</button>
        </div>
    };
    
    let banked_route_list = banked_routes.iter().enumerate().map(|(index, config)| {
        html! {
            <div class="code-block">
                <h3>{"Banked IP Route Configuration:"}</h3>
                <pre>{config}</pre>
                <button class="bg-red-500 hover:bg-red-700 text-white font-bold py-1 px-2 mt-3 rounded" onclick={remove_route_config.reform(move |_| index)}>{"Remove"}</button>
            </div>
        }
    }).collect::<Html>();

    let route_setup_accordion_item = html! {
        <AccordionItem title="IP Route Setup" content={html!{
            <div>
                {route_configuration_block}
                <div class="config-list">
                    {banked_route_list}
                    <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 mt-6 rounded" onclick={add_routes_to_config}>{"Add to Config"}</button>
                </div>
            </div>
        }} position={AccordionItemPosition::Middle}/>
    };


    let link_display = if !shared_link.is_empty() {
        html! {
            <div class="link-display">
                <p class="text-lg font-bold mb-2">{"Command to install config on switch:"}</p>
                <div class="border border-gray-300 p-2 rounded">
                    <p class="font-mono text-sm">{format!("copy {} running-config", *shared_link)}</p>
                </div>
            </div>
        }
    } else {
        html! {}
    };
    
    html! {
        <>
        <div class="main-container">
            <Search_nav />
            {
                html! {
                    // <div class="flex flex-col space-y-4 w-full max-w-xs p-8 border border-gray-300 rounded-lg shadow-lg bg-gray-600">

                        <div class="container mx-auto text-center p-10">
                            <div class="flex justify-center items-center">
                                <img class="object-scale-down h-20 w-66" src="static/assets/favicon.png" alt="NetTailor Logo" />
                            </div>
                            <h1 class="item_container-text text-4xl font-bold mb-5">{"Generate New Configuration"}</h1>
                            <div class="grid grid-cols-2 gap-4">
                                <div class="config-form">


                                // Continue with other inputs...

                                <div id="accordion-collapse" data-accordion="collapse" class="bg-custom-light item_container-text">
                                    <AccordionItem title="Client Configuration" content={html!{ 
                                        <div class="config-form">
                                            <div class="input-field">
                                                {auvik_collector_block}
                                                {client_name_block}
                                                {client_domain_block}
                                                {location_block}
                                                {tacacs_server_block}
                                                {tacacs_key_block}
                                                {snmp_community_string_block}

                                                {ise_server_block}
                                            </div>
                                        </div>
                                        }} position={AccordionItemPosition::First}/>
                                    <AccordionItem title="Device Type" content={html!{ 
                                        <div class="config-form">
                                            <div class="input-field">
                                                {device_type_block}
                                            </div>
                                        </div>
                                        }} position={AccordionItemPosition::Middle}/>
                                    <AccordionItem title="Service Configuration" content={html!{ 
                                        <div class="config-form">
                                            <div class="input-field">
                                                {service_type_block}
                                            </div>
                                        </div>
                                        }} position={AccordionItemPosition::Middle}/>
                                        <AccordionItem title="Device Model" content={html!{ 
                                            <div class="config-form">
                                                <div class="input-field">
                                                    {device_model_block}
                                                </div>
                                            </div>
                                            }} position={AccordionItemPosition::Middle}/>
                                    <AccordionItem title="Device Configuration" content={html!{ 
                                        <div class="config-form">                                     
                                            {hostname_block}
                                            {device_ip_block}
                                            {dns_server1_block}
                                            {dns_server2_block}
                                            {default_admin_block}
                                            {encrypted_enable_pass_block}
                                            {encrypted_user_pass_block}
                                            {timezone_block}
                                            {vlan_range_block}
                                        </div>
                                        }} position={AccordionItemPosition::Middle}/>
                                        {vlan_setup_accordion_item}
                                        {interface_setup_accordion_item}
                                        {route_setup_accordion_item}

                                </div>
                            </div>

                            <div class="relative container item_container-text">
                                <div>
                                    <textarea class="config-output peer h-full min-h-[500px] w-full resize-vertical rounded-[7px] border border-blue-gray-200 border-t-transparent bg-transparent px-3 py-2.5 font-sans text-sm font-normal text-blue-gray-700 outline outline-0 transition-all placeholder-shown:border placeholder-shown:border-blue-gray-200 placeholder-shown:border-t-blue-gray-200 focus:border-2 focus:border-gray-900 focus:border-t-transparent focus:outline-0 disabled:border-0 disabled:bg-blue-gray-50" value={(*configuration).clone()} />
                                    <label
                                        class="before:content[' '] after:content[' '] pointer-events-none absolute left-0 -top-1.5 flex h-full w-full select-none text-[11px] font-normal leading-tight text-blue-gray-400 transition-all before:pointer-events-none before:mt-[6.5px] before:mr-1 before:box-border before:block before:h-1.5 before:w-2.5 before:rounded-tl-md before:border-t before:border-l before:border-blue-gray-200 before:transition-all after:pointer-events-none after:mt-[6.5px] after:ml-1 after:box-border after:block after:h-1.5 after:w-2.5 after:flex-grow after:rounded-tr-md after:border-t after:border-r after:border-blue-gray-200 after:transition-all peer-placeholder-shown:text-sm peer-placeholder-shown:leading-[3.75] peer-placeholder-shown:text-blue-gray-500 peer-placeholder-shown:before:border-transparent peer-placeholder-shown:after:border-transparent peer-focus:text-[11px] peer-focus:leading-tight peer-focus:text-gray-900 peer-focus:before:border-t-2 peer-focus:before:border-l-2 peer-focus:before:border-gray-900 peer-focus:after:border-t-2 peer-focus:after:border-r-2 peer-focus:after:border-gray-900 peer-disabled:text-transparent peer-disabled:before:border-transparent peer-disabled:after:border-transparent peer-disabled:peer-placeholder-shown:text-blue-gray-500">
                                        {"Current Configuration"}
                                    </label>
                                </div>
                            </div>
                            </div>
                        <div>
                            <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 mt-3 rounded" onclick={generate_config_click}>{"Generate Config"}</button>
                        </div>
                        {link_display}
                        </div>
                    // </div>
                }
            }
        // Conditional rendering for the error banner
        if let Some(error) = error_message {
            <div class="error-snackbar">{ error }</div>
        }
        if let Some(info) = info_message {
            <div class="info-snackbar">{ info }</div>
        }
        </div>
        <App_drawer />
        </>
    }

}

