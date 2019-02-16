use hdk::{
    error::ZomeApiResult,
    holochain_wasm_utils::api_serialization::get_links::GetLinksResult,
};
use hdk::holochain_core_types::{
    hash::HashString,
    json::JsonString,
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

pub fn handle_enable_app(app_hash: HashString) -> ZomeApiResult<()> {
    utils::link_entries_bidir(&app_hash, &hdk::AGENT_ADDRESS, "host_enabled", "apps_enabled")?;
    Ok(())
}

pub fn handle_disable_app(app_hash: HashString) -> ZomeApiResult<()> {
    utils::remove_link_entries_bidir(&app_hash, &hdk::AGENT_ADDRESS, "host_enabled", "apps_enabled")?;
    Ok(())
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
