use std::default;

use gloo::history::Location;
use yew::prelude::*;
use yewdux::use_store;
use super::app_drawer::App_drawer;
use super::search_nav::Search_nav;
use yew_router::history::BrowserHistory;
use crate::components::context::{AppState, UIState};
use crate::components::misc_func::generate_config;
use web_sys::{HtmlSelectElement, HtmlInputElement};
use crate::components::settings::AccordionItem;
use crate::components::settings::AccordionItemPosition;
use web_sys::HtmlTextAreaElement;
use wasm_bindgen_futures::spawn_local;
use crate::requests::net_requests::{send_config_to_server, add_config_db, DeviceConfig};

#[function_component(CreateConfig)]
pub fn create_config() -> Html {
    let (state, dispatch) = use_store::<AppState>();
    let effect_dispatch = dispatch.clone();
    let history = BrowserHistory::new();


    let (post_state, _post_dispatch) = use_store::<AppState>();
    let (audio_state, audio_dispatch) = use_store::<UIState>();
    let error_message = audio_state.error_message.clone();
    let info_message = audio_state.info_message.clone();
    let api_key = state.auth_details.as_ref().map(|ud| ud.api_key.clone());
    let server_name = state.auth_details.as_ref().map(|ud| ud.server_name.clone());
    let cloud_storage = state.use_cloud_storage.clone();
    let _user_id = state.user_details.as_ref().map(|ud| ud.UserID.clone());
    let loading = use_state(|| true);


    let os_version = use_state(|| String::from(""));
    let hostname = use_state(|| String::from(""));
    let tacacs_server = use_state(|| String::from(""));
    let ise_server = use_state(|| String::from(""));
    let timezone = use_state(|| String::from(""));
    let boot_bin = use_state(|| String::from(""));
    let switch_number = use_state(|| String::from(""));
    let model = use_state(|| String::from(""));
    let vtp_domain = use_state(|| String::from(""));
    let auvik_collector = use_state(|| String::from(""));
    let crypto_auth_block = use_state(|| String::from(""));
    let vlan_range = use_state(|| String::from(""));
    let encrypted_user_pass = use_state(|| String::from(""));
    let vlan_definitions = use_state(|| String::from(""));
    let class_map_definitions = use_state(|| String::from(""));
    let policy_map_definitions = use_state(|| String::from(""));
    let client_name = use_state(|| String::from(""));
    let device_type = use_state(|| String::from(""));
    let service_type = use_state(|| String::from(""));
    let device_model = use_state(|| String::from(""));
    let switch_config_block = use_state(|| String::from(""));
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

    let generate_config_click = {
        let server_name = server_name.clone(); // URL to your backend server
        let api_key = api_key.clone(); // API key for your backend server
        let use_cloud_storage = cloud_storage.clone(); // State that determines whether to use cloud or local storage
        let call_config = (*configuration).clone();
        let call_hostname = (*hostname).clone();
        let call_location = (*location).clone();
        Callback::from(move |_: MouseEvent| {
            let server_name = server_name.clone();
            let server_name = server_name.clone();
            let use_cloud_storage = use_cloud_storage.clone();
    
            // Example device config, replace this with actual data collection logic
            let device_config = DeviceConfig {
                hostname: call_hostname.clone(),
                config: call_config.clone(),
                location: call_location.clone(), // This could be dynamically determined based on `use_cloud_storage`
            };
    
            let future = async move {
                match add_config_db(&server_name.clone().unwrap(), &device_config).await {
                    Ok(_) => {
                        match send_config_to_server(&server_name.unwrap(), &device_config, use_cloud_storage.unwrap()).await {
                            Ok(message) => web_sys::console::log_1(&format!("Success: {}", message).into()),
                            Err(e) => web_sys::console::log_1(&format!("Error sending config to server: {}", e).into()),
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
        os_version: String,
        hostname: String,
        tacacs_server: String,
        ise_server: String,
        timezone: String,
        boot_bin: String,
        switch_number: String,
        model: String,
        vtp_domain: String,
        auvik_collector: String,
        crypto_auth_block: String,
        vlan_range: String,
        encrypted_user_pass: String,
        vlan_definitions: String,
        class_map_definitions: String,
        policy_map_definitions: String,
        client_name: String,
        device_type: String,
        service_type: String,
        device_model: String,
        switch_config_block: String,
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
        os_version: (*os_version).clone(),
        hostname: (*hostname).clone(),
        tacacs_server: (*tacacs_server).clone(),
        ise_server: (*ise_server).clone(),
        timezone: (*timezone).clone(),
        boot_bin: (*boot_bin).clone(),
        switch_number: (*switch_number).clone(),
        model: (*model).clone(),
        vtp_domain: (*vtp_domain).clone(),
        auvik_collector: (*auvik_collector).clone(),
        crypto_auth_block: (*crypto_auth_block).clone(),
        vlan_range: (*vlan_range).clone(),
        encrypted_user_pass: (*encrypted_user_pass).clone(),
        vlan_definitions: (*vlan_definitions).clone(),
        class_map_definitions: (*class_map_definitions).clone(),
        policy_map_definitions: (*policy_map_definitions).clone(),
        client_name: (*client_name).clone(),
        device_type: (*device_type).clone(),
        service_type: (*service_type).clone(),
        device_model: (*device_model).clone(),
        switch_config_block: (*switch_config_block).clone(),
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
            &deps.os_version, 
            &deps.hostname, 
            &deps.tacacs_server,
            &deps.ise_server,
            &deps.timezone,
            &deps.boot_bin,
            &deps.switch_number,
            &deps.model,
            &deps.vtp_domain,
            &deps.auvik_collector,
            &deps.crypto_auth_block,
            &deps.vlan_range,
            &deps.encrypted_user_pass,
            &deps.vlan_definitions,
            &deps.class_map_definitions,
            &deps.policy_map_definitions,
            &deps.client_name,
            &deps.device_type,
            &deps.service_type,
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

    let os_version_block = html! {
        <div class="config-form">
            <div class="input-field" style="display: flex; align-items: center; justify-content: flex-start;">
                <label style="margin-right: 10px;">{"OS Version:"}</label>
                <input
                    type="text"
                    class="email-input border p-2 ml-2 rounded"
                    placeholder="OS Version"
                    value={(*os_version).clone()}
                    oninput={{
                        let os_version = os_version.clone();
                        Callback::from(move |e: InputEvent| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            os_version.set(input.value());
                        })
                    }}
                />
            </div>
        </div>
    };

    let interface_definitions_block = html! {
        <div class="config-form">
            <div class="input-field" style="display: flex; align-items: center; justify-content: flex-start;">
                <label style="margin-right: 10px;">{"Interface Definitions:"}</label>
                <input
                    type="text"
                    class="email-input border p-2 ml-2 rounded"
                    placeholder="Interface Definitions"
                    value={(*interface_definitions).clone()}
                    oninput={{
                        let interface_definitions = interface_definitions.clone();
                        Callback::from(move |e: InputEvent| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            interface_definitions.set(input.value());
                        })
                    }}
                />
            </div>
        </div>
    };

    let client_domain_block = html! {
        <div class="config-form">
            <div class="input-field" style="display: flex; align-items: center; justify-content: flex-start;">
                <label style="margin-right: 10px;">{"Client Domain:"}</label>
                <input
                    type="text"
                    class="email-input border p-2 ml-2 rounded"
                    placeholder="Client Domain"
                    value={(*client_domain).clone()}
                    oninput={{
                        let client_domain = client_domain.clone();
                        Callback::from(move |e: InputEvent| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            client_domain.set(input.value());
                        })
                    }}
                />
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
                <label style="margin-right: 10px;">{"Encrypted Enable Pass:"}</label>
                <input
                    type="text"
                    class="email-input border p-2 ml-2 rounded"
                    placeholder="Encrypted Enable Pass"
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
                    placeholder="Device IP"
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

    let ise_server_block = html! {
        <div class="config-form">
            <div class="input-field" style="display: flex; align-items: center; justify-content: flex-start;">
                <label style="margin-right: 10px;">{"ISE Server:"}</label>
                <input
                    type="text"
                    class="email-input border p-2 ml-2 rounded"
                    placeholder="ISE Server"
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
                    placeholder="Tacacs Server"
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
                <label style="margin-right: 10px;">{"ISE Server:"}</label>
                <input
                    type="text"
                    class="email-input border p-2 ml-2 rounded"
                    placeholder="ISE Server"
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

    let timezone_block = html! {
        <div class="config-form">
            <div class="input-field" style="display: flex; align-items: center; justify-content: flex-start;">
                <label style="margin-right: 10px;">{"Timezone:"}</label>
                <input
                    type="text"
                    class="email-input border p-2 ml-2 rounded"
                    placeholder="Timezone"
                    value={(*timezone).clone()}
                    oninput={{
                        let timezone = timezone.clone();
                        Callback::from(move |e: InputEvent| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            timezone.set(input.value());
                        })
                    }}
                />
            </div>
        </div>
    };

    let boot_bin_block = html! {
        <div class="config-form">
            <div class="input-field" style="display: flex; align-items: center; justify-content: flex-start;">
                <label style="margin-right: 10px;">{"Boot Bin:"}</label>
                <input
                    type="text"
                    class="email-input border p-2 ml-2 rounded"
                    placeholder="Boot Bin"
                    value={(*boot_bin).clone()}
                    oninput={{
                        let boot_bin = boot_bin.clone();
                        Callback::from(move |e: InputEvent| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            boot_bin.set(input.value());
                        })
                    }}
                />
            </div>
        </div>
    };

    let switch_number_block = html! {
        <div class="config-form">
            <div class="input-field" style="display: flex; align-items: center; justify-content: flex-start;">
                <label style="margin-right: 10px;">{"Switch Number:"}</label>
                <input
                    type="text"
                    class="email-input border p-2 ml-2 rounded"
                    placeholder="Switch Number"
                    value={(*switch_number).clone()}
                    oninput={{
                        let switch_number = switch_number.clone();
                        Callback::from(move |e: InputEvent| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            switch_number.set(input.value());
                        })
                    }}
                />
            </div>
        </div>
    };

    let model_block = html! {
        <div class="config-form">
            <div class="input-field" style="display: flex; align-items: center; justify-content: flex-start;">
                <label style="margin-right: 10px;">{"Model:"}</label>
                <input
                    type="text"
                    class="email-input border p-2 ml-2 rounded"
                    placeholder="Model"
                    value={(*model).clone()}
                    oninput={{
                        let model = model.clone();
                        Callback::from(move |e: InputEvent| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            model.set(input.value());
                        })
                    }}
                />
            </div>
        </div>
    };

    let vtp_domain_block = html! {
        <div class="config-form">
            <div class="input-field" style="display: flex; align-items: center; justify-content: flex-start;">
                <label style="margin-right: 10px;">{"VTP Domain:"}</label>
                <input
                    type="text"
                    class="email-input border p-2 ml-2 rounded"
                    placeholder="VTP Domain"
                    value={(*vtp_domain).clone()}
                    oninput={{
                        let vtp_domain = vtp_domain.clone();
                        Callback::from(move |e: InputEvent| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            vtp_domain.set(input.value());
                        })
                    }}
                />
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
                    placeholder="Auvik Collector"
                    value={(*auvik_collector).clone()}
                    oninput={{
                        let auvik_collector = auvik_collector.clone();
                        Callback::from(move |e: InputEvent| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            auvik_collector.set(input.value());
                        })
                    }}
                />
            </div>
        </div>
    };

    let crypto_auth_block = html! {
        <div class="config-form">
            <div class="input-field" style="display: flex; align-items: center; justify-content: flex-start;">
                <label style="margin-right: 10px;">{"Crypto Auth Block:"}</label>
                <input
                    type="text"
                    class="email-input border p-2 ml-2 rounded"
                    placeholder="Crypto Auth Block"
                    value={(*crypto_auth_block).clone()}
                    oninput={{
                        let crypto_auth_block = crypto_auth_block.clone();
                        Callback::from(move |e: InputEvent| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            crypto_auth_block.set(input.value());
                        })
                    }}
                />
            </div>
        </div>
    };

    let vlan_range_block = html! {
        <div class="config-form">
            <div class="input-field" style="display: flex; align-items: center; justify-content: flex-start;">
                <label style="margin-right: 10px;">{"VLAN Range:"}</label>
                <input
                    type="text"
                    class="email-input border p-2 ml-2 rounded"
                    placeholder="VLAN Range"
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
                <label style="margin-right: 10px;">{"Encrypted User Pass:"}</label>
                <input
                    type="text"
                    class="email-input border p-2 ml-2 rounded"
                    placeholder="Encrypted User Pass"
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

    let ip_routes_block = html! {
        <div class="config-form">
            <div class="input-field" style="display: flex; align-items: center; justify-content: flex-start;">
                <label style="margin-right: 10px;">{"IP Routes:"}</label>
                <input
                    type="text"
                    class="email-input border p-2 ml-2 rounded"
                    placeholder="IP Routes"
                    value={(*ip_routes).clone()}
                    oninput={{
                        let ip_routes = ip_routes.clone();
                        Callback::from(move |e: InputEvent| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            ip_routes.set(input.value());
                        })
                    }}
                />
            </div>
        </div>
    };

    let custom_snmp_config_block = html! {
        <div class="config-form">
            <div class="input-field" style="display: flex; align-items: center; justify-content: flex-start;">
                <label style="margin-right: 10px;">{"Custom SNMP Config:"}</label>
                <textarea
                    class="email-input border p-2 ml-2 rounded"
                    placeholder="Custom SNMP Config"
                    value={(*custom_snmp_config).clone()}
                    oninput={{
                        let custom_snmp_config = custom_snmp_config.clone();
                        Callback::from(move |e: InputEvent| {
                            let textarea: HtmlTextAreaElement = e.target_unchecked_into();
                            custom_snmp_config.set(textarea.value());
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
                    placeholder="Location"
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
                    placeholder="Tacacs Key"
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

    let vlan_definitions_block = html! {
        <div class="config-form">
            <div class="input-field" style="display: flex; align-items: center; justify-content: flex-start;">
                <label style="margin-right: 10px;">{"VLAN Definitions:"}</label>
                <input
                    type="text"
                    class="email-input border p-2 ml-2 rounded"
                    placeholder="VLAN Definitions"
                    value={(*vlan_definitions).clone()}
                    oninput={{
                        let vlan_definitions = vlan_definitions.clone();
                        Callback::from(move |e: InputEvent| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            vlan_definitions.set(input.value());
                        })
                    }}
                />
            </div>
        </div>
    };


    let on_client_name_change = {
        let client_name = client_name.clone();
        Callback::from(move |e: InputEvent| {
            let select: web_sys::HtmlSelectElement = e.target_unchecked_into();
            client_name.set(select.value());
        })
    };

    let client_name_block = html! {
        <div class="config-form">
            <div class="input-field" style="display: flex; align-items: center; justify-content: flex-start;">
                <label style="margin-right: 10px;">{"Client Name:"}</label>
                <select
                    class="email-input border p-2 ml-2 rounded"
                    value={(*client_name).clone()}
                    oninput={on_client_name_change.clone()}
                >
                    <option value="Option1">{"Option1"}</option>
                    <option value="Option2">{"Option2"}</option>
                    <option value="Option3">{"Option3"}</option>
                    // Add more options as needed
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
                                    <option value="access">{"Access"}</option>
                                    <option value="distribution">{"Distribution"}</option>
                                    <option value="core">{"Core"}</option>
                                </>
                            }
                        } else if *device_type == "Router" {
                            html! {
                                <>
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
                                    <option value="access">{"model1"}</option>
                                    <option value="distribution">{"model2"}</option>
                                    <option value="core">{"model3"}</option>
                                </>
                            }
                        } else if *device_type == "Router" {
                            html! {
                                <>
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
                                                {custom_snmp_config_block}
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
                                            {ip_routes_block}
                                        </div>
                                        }} position={AccordionItemPosition::Middle}/>
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
                            <button onclick={generate_config_click}>{"Generate Config"}</button>
                        </div>

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

