use hdk::{
    error::ZomeApiResult,
};
use hdk::holochain_core_types::{
    hash::HashString,
};

pub fn handle_enable_app(app_hash: HashString) -> ZomeApiResult<()> {
    utils::link_entries_bidir(&app_hash, &hdk::AGENT_ADDRESS, "host_enabled", "apps_enabled")?;
    Ok(())
}

// pub fn handle_disable_app(app_hash: Address) -> ZomeApiResult<Address> {
//     hdk::get_entry(&address)
// }
