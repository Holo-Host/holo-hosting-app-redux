use hdk::{
    error::{ZomeApiResult,ZomeApiError},
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
// use std::convert::TryFrom;
// use serde_json::{Result, Value};

#[derive(Serialize, Deserialize, Debug, DefaultJson)]
pub struct AppConfig {
    pub ui_hash:HashString,
    pub dna_list:Vec<HashString>,
}

#[derive(Serialize, Deserialize, Debug, DefaultJson)]
pub struct DnaToHost{
    recently_enabled_apps:Vec<App2Host>,
    recently_disabled_apps:Vec<App2Host>
}

#[derive(Serialize, Deserialize, Debug, DefaultJson)]
pub struct App2Host{
    app:HashString,
    host:Vec<String>
}

#[derive(Serialize, Deserialize, Debug, DefaultJson)]
pub struct AllApps{
    hash:HashString,
    details:String
}

pub fn validate_host() -> ZomeApiResult<bool> {
    let check = handle_is_registered_as_host()?;
    if check.addresses().len() != 0 {
        hdk::debug("PASSING TRUE")?;
        Ok(true)
    }
    else {
        hdk::debug("PASSING FALSE")?;
        Err(ZomeApiError::Internal(
            "Agent Not a Host".to_string()))
    }
}

pub fn handle_get_all_apps() -> ZomeApiResult<Vec<AllApps>> {
    validate_host()?;
    let all_apps = Entry::App("anchor".into(), RawString::from("ALL_APPS").into());
    let anchor_address = hdk::commit_entry(&all_apps)?;
    let all_apps_commit = hdk::get_links(&anchor_address, "all_apps_tag")?;
    let app_address = all_apps_commit.addresses();

    let mut app_details_list: Vec<AllApps> =Vec::new();
    for x in app_address{
        let details = hdk::call(hdk::THIS_INSTANCE,"provider",Address::from(hdk::PUBLIC_TOKEN.to_string()),"get_app_details",json!({"app_hash":x}).into())?;
        app_details_list.push(AllApps{
            hash:x.to_owned(),
            details:String::from(details.to_owned())
        });
    }
    Ok(app_details_list)
}

pub fn handle_enable_app(app_hash: HashString) -> ZomeApiResult<()> {
    validate_host()?;
    utils::link_entries_bidir(&app_hash, &hdk::AGENT_ADDRESS, "host_enabled", "apps_enabled")?;

    // check if its a recently_disabled_app_tag
    hdk::remove_link(&app_hash,&hdk::AGENT_ADDRESS,"recently_disabled_app_tag")?;

    hdk::link_entries(&app_hash,&hdk::AGENT_ADDRESS,"recently_enabled_app_tag")?;

    Ok(())
}

pub fn handle_disable_app(app_hash: HashString) -> ZomeApiResult<()> {
    validate_host()?;
    utils::remove_link_entries_bidir(&app_hash, &hdk::AGENT_ADDRESS, "host_enabled", "apps_enabled")?;

    // check if its a recently_disabled_app_tag
    hdk::remove_link(&app_hash,&hdk::AGENT_ADDRESS,"recently_enabled_app_tag")?;

    hdk::link_entries(&app_hash,&hdk::AGENT_ADDRESS,"recently_disabled_app_tag")?;

    Ok(())
}

fn handle_get_all_apps_addresses() -> ZomeApiResult<GetLinksResult> {
    validate_host()?;
    let all_apps = Entry::App("anchor".into(), RawString::from("ALL_APPS").into());
    let anchor_address = hdk::commit_entry(&all_apps)?;

    hdk::get_links(&anchor_address, "all_apps_tag")
}

pub fn handle_get_kv_updates_dna_to_host()-> ZomeApiResult<DnaToHost> {
    // Get all the apps
    let got_apps:GetLinksResult = handle_get_all_apps_addresses()?;
    let all_apps = got_apps.addresses().to_vec();
    // Check the enabled tag
    let mut recently_enabled_apps:Vec<App2Host>=Vec::new();
    for app in all_apps.clone(){
        let app_copy = app.clone();
        let mut enabled_agents:Vec<ZomeApiResult<Entry>> = hdk::get_links_and_load(&app_copy, "recently_enabled_app_tag")?;
        let mut enabled_agents_old:Vec<ZomeApiResult<Entry>> = hdk::get_links_and_load(&app_copy, "need_updates_enabled_from_kv_store")?;

        enabled_agents.append(&mut enabled_agents_old);

        let mut agent_address_list:Vec<String>=Vec::new();
        for a in enabled_agents{
            match a?{
                Entry::AgentId(a) => agent_address_list.push(a.pub_sign_key),
                _ =>{}
            }
        }
        recently_enabled_apps.push(App2Host{
            app,
            host:agent_address_list.clone()
        });

        // Remove the enable tag and add intransition apps
        for agent in agent_address_list{
            hdk::remove_link(&app_copy,&HashString::from(agent.clone()),"recently_enabled_app_tag")?;
            hdk::link_entries(&app_copy,&HashString::from(agent.clone()),"need_updates_enabled_from_kv_store")?;
        }

    }

    // Check the disabled tag
    let mut recently_disabled_apps:Vec<App2Host>=Vec::new();
    for app in all_apps.clone(){
        let app_copy = app.clone();
        let mut disabled_agents:Vec<ZomeApiResult<Entry>> = hdk::get_links_and_load(&app_copy, "recently_disabled_app_tag")?;
        let mut disabled_agents_old:Vec<ZomeApiResult<Entry>> = hdk::get_links_and_load(&app_copy, "need_updates_disabled_from_kv_store")?;

        disabled_agents.append(&mut disabled_agents_old);

        // Data Refactored
        let mut agent_address_list:Vec<String>=Vec::new();
        for a in disabled_agents{
            match a?{
                Entry::AgentId(a) => agent_address_list.push(a.pub_sign_key),
                _ =>{}
            }
        }
        recently_disabled_apps.push(App2Host{
            app,
            host:agent_address_list.clone()
        });
        // Remove the disabled tag and add intransition apps
        for agent in agent_address_list{
            hdk::remove_link(&app_copy,&HashString::from(agent.clone()),"recently_disabled_app_tag")?;
            hdk::link_entries(&app_copy,&HashString::from(agent.clone()),"need_updates_disabled_from_kv_store")?;
        }
    }
    Ok(DnaToHost{
        recently_enabled_apps,
        recently_disabled_apps
    })
}

pub fn handle_kv_updates_host_completed(kv_bundle:Vec<App2Host>)-> ZomeApiResult<()>{
    for kv in kv_bundle{
        for host_address in kv.host {
            hdk::remove_link(&kv.app,&HashString::from(host_address.clone()),"need_updates_enabled_from_kv_store",)?;
            hdk::remove_link(&kv.app,&HashString::from(host_address.clone()),"need_updates_disabled_from_kv_store",)?;
        }
    }
    Ok(())
}
pub fn handle_get_enabled_app_list() -> ZomeApiResult<Vec<utils::GetLinksLoadElement<AppConfig>>> {
    validate_host()?;
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
