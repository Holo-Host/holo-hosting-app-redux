use hdk::{
    error::ZomeApiResult,
};
use hdk::holochain_core_types::{
    hash::HashString,
    json::JsonString,
    error::HolochainError,
    cas::content::Address,
    entry::Entry,
};


#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
pub struct AppConfig {
    pub ui_hash:HashString,
    pub dna_list:Vec<HashString>,
}

pub fn handle_enable_app(app_hash: HashString) -> ZomeApiResult<()> {
    utils::link_entries_bidir(&app_hash, &hdk::AGENT_ADDRESS, "host_enabled", "apps_enabled")?;
    Ok(())
}

pub fn handle_get_enabled_app() -> ZomeApiResult<Vec<utils::GetLinksLoadElement<AppConfig>>> {
    utils::get_links_and_load_type(&hdk::AGENT_ADDRESS, "apps_enabled")
}

pub fn handle_get_host_for_app(app_hash:Address)->ZomeApiResult<Vec<ZomeApiResult<Entry>>>{
    hdk::get_links_and_load(&app_hash, "host_enabled")
}

// TODO : Remove the link
// pub fn handle_disable_app(app_hash: HashString) -> ZomeApiResult<()> {
//
// }
