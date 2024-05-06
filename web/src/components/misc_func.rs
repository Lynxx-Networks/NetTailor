use web_sys::{window, HtmlElement, HtmlInputElement, InputEvent};
use wasm_bindgen::JsCast;
use std::rc::Rc;
use yew::prelude::*;
use std::cell::RefCell;
use std::cell::Cell;

#[allow(dead_code)]
pub fn change_theme(theme: &str) {
    if let Some(window) = window() {
        if let Some(document) = window.document() {
            if let Some(root) = document.document_element() {
                if let Ok(html_element) = root.dyn_into::<HtmlElement>() {
                    let style = html_element.style();
                    match theme {
                        "light" => {
                            style.set_property("--button-color", "#new-dark-color").unwrap();
                            style.set_property("--background-color", "#new-dark-bg").unwrap();
                        },
                        "dark" => {
                            style.set_property("--button-color", "#new-light-color").unwrap();
                            style.set_property("--background-color", "#new-light-bg").unwrap();
                        },
                        "nordic" => {
                            style.set_property("--button-color", "#new-light-color").unwrap();
                            style.set_property("--background-color", "#new-light-bg").unwrap();
                        },
                        "abyss" => {
                            style.set_property("--button-color", "#new-light-color").unwrap();
                            style.set_property("--background-color", "#new-light-bg").unwrap();
                        },
                        "dracula" => {
                            style.set_property("--button-color", "#new-light-color").unwrap();
                            style.set_property("--background-color", "#new-light-bg").unwrap();
                        },
                        "kimbie" => {
                            style.set_property("--button-color", "#new-light-color").unwrap();
                            style.set_property("--background-color", "#new-light-bg").unwrap();
                        },
                        "hotdogstand - MY EYES" => {
                            style.set_property("--button-color", "#new-light-color").unwrap();
                            style.set_property("--background-color", "#new-light-bg").unwrap();
                        },
                        "neon" => {
                            style.set_property("--button-color", "#new-light-color").unwrap();
                            style.set_property("--background-color", "#new-light-bg").unwrap();
                        },
                        "wildberries" => {
                            style.set_property("--button-color", "#new-light-color").unwrap();
                            style.set_property("--background-color", "#new-light-bg").unwrap();
                        },
                        "greenie meanie" => {
                            style.set_property("--button-color", "#new-light-color").unwrap();
                            style.set_property("--background-color", "#new-light-bg").unwrap();
                        },
                        _ => {}
                    }
                }
            }
        }
    }
}

pub fn generate_config(
    os_version: &str,
    hostname: &str,
    tacacs_server: &str,
    ise_server: &str,
    timezone: &str,
    boot_bin: &str,
    switch_number: &str,
    model: &str,
    vtp_domain: &str,
    auvik_collector: &str,
    crypto_auth_block: &str,
    vlan_range: &str,
    encrypted_user_pass: &str,
    vlan_definitions: &str,
    class_map_definitions: &str,
    policy_map_definitions: &str,
    client_name: &str,
    device_type: &str,
    service_type: &str,
    devive_model: &str,
) -> String {
    format!("
version {}\n\
            service timestamps debug datetime msec\n\
            service timestamps log datetime msec\n\
            service call-home\n\
            platform punt-keepalive disable-kernel-core\n\
            hostname {}\n\
            vrf definition Mgmt-vrf\n \
             address-family ipv4\n \
             exit-address-family\n \
             address-family ipv6\n \
             exit-address-family\n\
            logging console emergencies\n\
            aaa new-model\n\
            aaa group server tacacs+ {}\n \
             server name {}\n \
             ip tacacs source-interface Loopback254\n\
            aaa authentication login default group {} local\n\
            aaa authentication enable default group {} enable\n\
            aaa session-id common\n\
            clock timezone {}\n\
            clock summer-time cdt recurring\n\
            boot system switch all flash:{}\n\
            switch {} provision {}\n\
            ip routing\n\
            no ip cef optimize neighbor resolution\n\
            login on-success log\n\
            udld enable\n\
            vtp domain {}\n\
            vtp mode transparent\n\
            flow record Record-FNF\n \
             description Flexible NetFlow Monitoring\n \
             match ipv4 tos\n \
             match ipv4 protocol\n \
             match ipv4 source address\n \
             match ipv4 destination address\n \
             match transport source-port\n \
             match transport destination-port\n \
             collect counter bytes long\n \
             collect counter packets long\n\
            flow exporter Export-FNF-Monitor-1\n \
             description FNFv9 NBAR2 with Auvik\n \
             destination {}\n \
             source Loopback254\n \
             transport udp 2055\n \
             template data timeout 60\n \
             option interface-table\n \
             option application-table\n\
            flow monitor Monitor-FNF\n \
             description IWAN Traffc Analysis\n \
             exporter Export-FNF-Monitor-1\n \
             cache timeout active 60\n \
             record Record-FNF\n\
            authentication mac-move permit\n\
            table-map AutoQos-4.0-Trust-Dscp-Table\n \
             default copy\n\
            table-map policed-dscp\n \
             map from  0 to 8\n \
             map from  10 to 8\n \
             map from  18 to 8\n \
             map from  24 to 8\n \
             map from  46 to 8\n \
             default copy\n\
            {}\n\
            license boot level network-essentials addon dna-essentials\n\
            memory free low-watermark processor 131696\n\
            diagnostic bootup level minimal\n\
            spanning-tree mode rapid-pvst\n\
            spanning-tree extend system-id\n\
            spanning-tree vlan {} priority 0\n\
            enable secret 9 {}\n\
            redundancy\n \
             mode sso\n\
            crypto engine compliance shield disable\n\
            transceiver type all\n \
             monitoring\n\
            {}\n\
            {}\n\
            {}\n",
            os_version, 
            hostname, 
            tacacs_server, 
            ise_server, 
            tacacs_server, 
            tacacs_server, 
            timezone, 
            boot_bin, 
            switch_number, 
            model, 
            vtp_domain, 
            auvik_collector, 
            crypto_auth_block, 
            vlan_range, 
            encrypted_user_pass, 
            vlan_definitions, 
            class_map_definitions, 
            policy_map_definitions,
        )
    }