use web_sys::{window, HtmlElement, HtmlInputElement, InputEvent};
use wasm_bindgen::JsCast;
use std::rc::Rc;
use yew::prelude::*;
use std::cell::RefCell;
use std::cell::Cell;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;


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
    device_model: &str,
    client_domain: &str,
    device_ip: &str,
    default_admin: &str,
    encrypted_enable_pass: &str,
    interface_definitions: &str,
    router_definitions: &str,
    ip_routes: &str,
    custom_snmp_config: &str,
    location: &str,
    snmp_community_string: &str,
    tacacs_key: &str,
    dns_server1: &str,
    dns_server2: &str,
) -> String {

    fn generate_random_key() -> String {
        let random_key: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(12)
            .map(char::from)
            .collect();
        random_key
    }
    let random_key = generate_random_key();

    let class_map_definitions = format!("
class-map match-any system-cpp-police-ewlc-control
  description EWLC Control 
class-map match-any AutoQos-4.0-Output-Multimedia-Conf-Queue
 match dscp af41  af42  af43 
 match cos  4 
class-map match-any system-cpp-police-topology-control
  description Topology control
class-map match-any system-cpp-police-sw-forward
  description Sw forwarding, L2 LVX data packets, LOGGING, Transit Traffic
class-map match-any AutoQos-4.0-Output-Bulk-Data-Queue
 match dscp af11  af12  af13 
 match cos  1 
class-map match-any system-cpp-default
  description EWLC Data, Inter FED Traffic 
class-map match-any system-cpp-police-sys-data
  description Openflow, Exception, EGR Exception, NFL Sampled Data, RPF Failed
class-map match-any AutoQos-4.0-Output-Priority-Queue
 match dscp cs4  cs5  ef 
 match cos  5 
class-map match-any system-cpp-police-punt-webauth
  description Punt Webauth
class-map match-any AutoQos-4.0-Output-Multimedia-Strm-Queue
 match dscp af31  af32  af33 
class-map match-any system-cpp-police-l2lvx-control
  description L2 LVX control packets
class-map match-any system-cpp-police-forus
  description Forus Address resolution and Forus traffic
class-map match-any system-cpp-police-multicast-end-station
  description MCAST END STATION
class-map match-any AutoQos-4.0-Voip-Data-CiscoPhone-Class
 match cos  5 
class-map match-any system-cpp-police-high-rate-app
  description High Rate Applications 
class-map match-any system-cpp-police-multicast
  description MCAST Data
class-map match-any AutoQos-4.0-Voip-Signal-CiscoPhone-Class
 match cos  3 
class-map match-any system-cpp-police-l2-control
  description L2 control
class-map match-any system-cpp-police-dot1x-auth
  description DOT1X Auth
class-map match-any system-cpp-police-data
  description ICMP redirect, ICMP_GEN and BROADCAST
class-map match-all AutoQoS-VoIP-RTP-Trust
 match ip dscp ef 
class-map match-any system-cpp-police-stackwise-virt-control
  description Stackwise Virtual OOB
class-map match-all AutoQoS-VoIP-Control-Trust
 match ip dscp cs3  af31 
class-map match-any non-client-nrt-class
class-map match-any AutoQos-4.0-Default-Class
 match access-group name AutoQos-4.0-Acl-Default
class-map match-any system-cpp-police-routing-control
  description Routing control and Low Latency
class-map match-any system-cpp-police-protocol-snooping
  description Protocol snooping
class-map match-any AutoQos-4.0-Output-Trans-Data-Queue
 match dscp af21  af22  af23 
 match cos  2 
class-map match-any system-cpp-police-dhcp-snooping
  description DHCP snooping
class-map match-any system-cpp-police-ios-routing
  description L2 control, Topology control, Routing control, Low Latency
class-map match-any system-cpp-police-system-critical
  description System Critical and Gold Pkt
class-map match-any AutoQos-4.0-Output-Scavenger-Queue
 match dscp cs1 
class-map match-any system-cpp-police-ios-feature
  description ICMPGEN,BROADCAST,ICMP,L2LVXCntrl,ProtoSnoop,PuntWebauth,MCASTData,Transit,DOT1XAuth,Swfwd,LOGGING,L2LVXData,ForusTraffic,ForusARP,McastEndStn,Openflow,Exception,EGRExcption,NflSampled,RpfFailed
class-map match-any AutoQos-4.0-Output-Control-Mgmt-Queue
 match dscp cs2  cs3  cs6  cs7 
 match cos  3 ");

    let policy_map_definitions = format!("
policy-map AutoQos-4.0-Output-Policy
    class AutoQos-4.0-Output-Priority-Queue
     priority level 1 percent 30
    class AutoQos-4.0-Output-Control-Mgmt-Queue
     bandwidth remaining percent 10 
     queue-limit dscp cs2 percent 80
     queue-limit dscp cs3 percent 90
     queue-limit dscp cs6 percent 100
     queue-limit dscp cs7 percent 100
     queue-buffers ratio 10
    class AutoQos-4.0-Output-Multimedia-Conf-Queue
     bandwidth remaining percent 10 
     queue-buffers ratio 10
    class AutoQos-4.0-Output-Trans-Data-Queue
     bandwidth remaining percent 10 
     queue-buffers ratio 10
    class AutoQos-4.0-Output-Bulk-Data-Queue
     bandwidth remaining percent 4 
     queue-buffers ratio 10
    class AutoQos-4.0-Output-Scavenger-Queue
     bandwidth remaining percent 1 
     queue-buffers ratio 10
    class AutoQos-4.0-Output-Multimedia-Strm-Queue
     bandwidth remaining percent 10 
     queue-buffers ratio 10
    class class-default
     bandwidth remaining percent 25 
     queue-buffers ratio 25
   policy-map system-cpp-policy
   policy-map AutoQos-4.0-Trust-Dscp-Input-Policy
    class class-default
     set dscp dscp table AutoQos-4.0-Trust-Dscp-Table
   policy-map AutoQoS-Police-CiscoPhone
    class AutoQoS-VoIP-RTP-Trust
     set dscp ef
    class AutoQoS-VoIP-Control-Trust
     set dscp cs3
   policy-map AutoQos-4.0-CiscoPhone-Input-Policy
    class AutoQos-4.0-Voip-Data-CiscoPhone-Class
     set dscp ef
     police cir 128000 bc 8000
      conform-action transmit 
      exceed-action set-dscp-transmit dscp table policed-dscp
    class AutoQos-4.0-Voip-Signal-CiscoPhone-Class
     set dscp cs3
     police cir 32000 bc 8000
      conform-action transmit 
      exceed-action set-dscp-transmit dscp table policed-dscp
    class AutoQos-4.0-Default-Class
     set dscp default ");

    let switch_config_line = if device_type == "Switch" {
        format!("switch {} provision {}", switch_number, device_model)
    } else {
        String::new()
    };
    format!("
service timestamps debug datetime msec\n\
            service timestamps log datetime msec\n\
            service timestamps log datetime msec\n\
            hostname {}\n\
            ip domain name {}\n\
            crypto key generate rsa general-keys modulus 2048\n\
            ip ssh version 2\n\
            logging console notifications\n\
            clock timezone {}\n\
            clock summer-time cdt recurring\n\
            ip routing\n\
            login on-success log\n\
            udld enable\n\
            vtp mode transparent\n\
            interface Loopback254
             ip address {}
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
             description Auvik Traffc Analysis
             exporter Export-FNF-Monitor-1
             cache timeout active 60
             record Record-FNF
            authentication mac-move permit\n\
            spanning-tree mode rapid-pvst\n\
            spanning-tree vlan {} priority 4096\n\
            enable secret 9 {}\n\
            username {} privilege 15 secret 9 {}\n\
            aaa new-model\n\
            aaa group server tacacs+ {}\n \
             server name {}\n \
            aaa authentication login default group {} local\n\
            aaa authentication enable default group {} enable\n\
            aaa session-id common\n\
            ntp server pool.ntp.org prefer
            {}\n\
            {}\n\
            interface Vlan1\n\
            no ip address\n\
            shutdown\n\
            !\n\
            ! (Only if ip routing is enabled)\n\
            {}\n\
            !\n\
            no ip http server\n\
            no ip http secure-server\n\
            !\n\
            {}\n\
            ip route (network) (subnet) (next-hop)\n\
            logging trap notifications\n\
            logging source-interface Loopback254\n\
            logging host {}\n\
            {}\n\
            snmp-server community {} RO\n\
            snmp-server trap-source Loopback254\n\
            snmp-server location {}\n\
            snmp-server host {} version 2c {}\n\
            tacacs-server timeout 60\n\
            tacacs server {}\n\
            address ipv4 {}\n\
            key {}\n\
            !\n\
            banner motd ^\n\
            * *************************************************************************\n\
            *                                                                         *\n\
            * WARNING: This system is for the use of authorized {}                    *\n\
            * personnel and consultants only!!                                        *\n\
            *                                                                         *\n\
            * Individuals using the computer network system without authorization,    *\n\
            * or in excess of their authorization, are subject to having all their    *\n\
            * activity on this computer network system monitored and recorded by      *\n\
            * system personnel.  To protect the computer network system from          *\n\
            * unauthorized use and to ensure the computer network systems is          *\n\
            * functioning properly, system administrators monitor this system.        *\n\
            * Anyone using this computer network system expressly consents to such    *\n\
            * monitoring and is advised that if such monitoring reveals possible      *\n\
            * conduct of criminal activity, system personnel may provide the          *\n\
            * evidence of such activity to law enforcement officers.                  *\n\
            *                                                                         *\n\
            * Access is restricted to authorized users only. Unauthorized access is   *\n\
            * a violation of state and federal, civil and criminal laws.              *\n\
            ***************************************************************************\n\
            ^\n\
            !\n\
            ip name-server {}\n\
            ip name-server {}\n\
            line con 0\n\
            exec-timeout 0 0\n\
            login local\n\
            logging synchronous\n\
            stopbits 1\n\
            line vty 0 4\n\
            logging synchronous\n\
            transport input ssh\n\
            line vty 5 15\n\
            logging synchronous\n\
            transport input ssh\n\
            line vty 16 31\n\
            transport input ssh\n\
            {}\n\
            {}\n\
            {}\n\
            end",
            hostname, 
            client_domain,
            timezone,
            device_ip,
            auvik_collector,
            vlan_range,
            encrypted_enable_pass, 
            default_admin,
            encrypted_user_pass,
            tacacs_server, 
            ise_server, 
            tacacs_server, 
            tacacs_server, 
            vlan_definitions,
            interface_definitions,
            router_definitions,
            ip_routes,
            auvik_collector,
            custom_snmp_config,
            random_key,
            location,
            auvik_collector,
            snmp_community_string,
            tacacs_server,
            tacacs_server,
            tacacs_key,
            client_name,
            dns_server1,
            dns_server2,
            class_map_definitions, 
            policy_map_definitions,
            switch_config_line,
        )
    }