use hdk::{
    error::ZomeApiResult,
    holochain_wasm_utils::api_serialization::get_links::GetLinksResult,
};
use hdk::holochain_core_types::{
    hash::HashString,
    json::JsonString,
    json::RawString,
    error::HolochainError,
    cas::content::Address,
    entry::Entry,
};
use crate::entry::host_doc::HostDoc;


#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
pub struct AppConfig {
    pub ui_hash:HashString,
    pub dna_list:Vec<HashString>,
}


pub fn handle_get_all_apps() -> ZomeApiResult<GetLinksResult> {
    let all_apps = Entry::App("anchor".into(), RawString::from("ALL_APPS").into());
    let anchor_address = hdk::commit_entry(&all_apps)?;

    hdk::get_links(&anchor_address, "all_apps_tag")
}

pub fn handle_enable_app(app_hash: HashString) -> ZomeApiResult<()> {
    utils::link_entries_bidir(&app_hash, &hdk::AGENT_ADDRESS, "host_enabled", "apps_enabled")?;

    // check if its a recently_disabled_app_tag
    hdk::remove_link(&app_hash,&hdk::AGENT_ADDRESS,"recently_disabled_app_tag")?;

    hdk::link_entries(&app_hash,&hdk::AGENT_ADDRESS,"recently_enabled_app_tag");

    Ok(())
}

pub fn handle_disable_app(app_hash: HashString) -> ZomeApiResult<()> {
    utils::remove_link_entries_bidir(&app_hash, &hdk::AGENT_ADDRESS, "host_enabled", "apps_enabled")?;

    // check if its a recently_disabled_app_tag
    hdk::remove_link(&app_hash,&hdk::AGENT_ADDRESS,"recently_enabled_app_tag")?;

    hdk::link_entries(&app_hash,&hdk::AGENT_ADDRESS,"recently_disabled_app_tag");

    Ok(())
}

#[derive(Serialize, Deserialize, Debug, DefaultJson)]
pub struct DnaToHost{
    recently_enabled_apps:Vec<app2host>,
    recently_disabled_apps:Vec<app2host>
}
#[derive(Serialize, Deserialize, Debug, DefaultJson)]
struct app2host{
    app:HashString,
    host:Vec<String>
}
pub fn handle_get_kv_updates_dna_to_host()-> ZomeApiResult<DnaToHost> {
    // Get all the apps
    let got_apps:GetLinksResult = handle_get_all_apps()?;
    let all_apps = got_apps.addresses().to_vec();
    // Check the enabled tag
    let mut recently_enabled_apps:Vec<app2host>=Vec::new();
    for app in all_apps.clone(){
        let app_copy = app.clone();
        let enabled_agents:Vec<ZomeApiResult<Entry>> = hdk::get_links_and_load(&app_copy, "recently_enabled_app_tag")?;
        let mut agent_address_list:Vec<String>=Vec::new();
        for a in enabled_agents{
            match a?{
                Entry::AgentId(a) => agent_address_list.push(a.key),
                _ =>{}
            }
        }
        recently_enabled_apps.push(app2host{
            app,
            host:agent_address_list
        })
    }

    // Check the disabled tag
    let mut recently_disabled_apps:Vec<app2host>=Vec::new();
    for app in all_apps.clone(){
        let app_copy = app.clone();
        let disabled_agents:Vec<ZomeApiResult<Entry>> = hdk::get_links_and_load(&app_copy, "recently_disabled_app_tag")?;
        // Data Refactored
        let mut agent_address_list:Vec<String>=Vec::new();
        for a in disabled_agents{
            match a?{
                Entry::AgentId(a) => agent_address_list.push(a.key),
                _ =>{}
            }
        }
        recently_disabled_apps.push(app2host{
            app,
            host:agent_address_list
        })
    }
    Ok(DnaToHost{
        recently_enabled_apps,
        recently_disabled_apps
    })
}

pub fn handle_get_enabled_app() -> ZomeApiResult<Vec<utils::GetLinksLoadElement<AppConfig>>> {
    utils::get_links_and_load_type(&hdk::AGENT_ADDRESS, "apps_enabled")
}

pub fn handle_get_host_for_app(app_hash:Address)->ZomeApiResult<Vec<ZomeApiResult<Entry>>>{
    hdk::get_links_and_load(&app_hash, "host_enabled")
}

pub fn handle_register_as_host(host_doc:HostDoc) -> ZomeApiResult<Address> {
    // TODO : Validation
    let verified_entry = Entry::App("host_doc".into(), host_doc.into());
    utils::commit_and_link( &verified_entry,&hdk::AGENT_ADDRESS, "verified_host_tag")
    // Ok(address)
}

pub fn handle_is_registered_as_host() -> ZomeApiResult<GetLinksResult> {
    hdk::get_links(&hdk::AGENT_ADDRESS, "verified_host_tag")
}

/*************************/
/* Service Log Functions */
/*************************/
// TODO : This functions need to bridge to the service logs

// // Suggested struct for the service log config
// #[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
// pub struct ServiceLogConfig {
//     pub app_hash:String, // Bundle hash in the Holo Hosting for which the Config is going to be set
//     pub payment_per_invoice:String, //
//     pub unpaied_limit:String // This is the limit after which the app would be stoped hosted on the HoloPort
// }
//
// // This need to bridge to the Service log app and the details need to be commited in the Service Log DHT
// pub fn handle_bridging_to_add_app_service_log_config(service_log_config:ServiceLogConfig,app_hash:Address) //-> ZomeApiResult<Address>
// {
//     // TODO
// }
