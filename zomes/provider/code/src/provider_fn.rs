use crate::entry::app_config::AppConfig;
use crate::entry::app_details::AppDetails;
use crate::entry::provider_doc::ProviderDoc;

use hdk::{
    self,
    holochain_core_types::{
        entry::Entry,
        hash::HashString,
        json::RawString,
        cas::content::Address,
    },
    holochain_wasm_utils::api_serialization::get_links::GetLinksResult,
    error::ZomeApiResult,
};

pub fn handle_register_app(ui_hash:HashString,dna_list:Vec<HashString>) -> ZomeApiResult<Address> {
    // TODO
    // Validation before commiting to the DHT
    // Check if user is verified
    // Check if all the hashes exist in the HCHC

    let app_entry = Entry::App("app_config".into(), AppConfig{
        ui_hash,
        dna_list
        }.into());

    utils::commit_and_link(&app_entry, &hdk::AGENT_ADDRESS, "registered_tag")
}

pub fn handle_get_my_registered_app() -> ZomeApiResult<GetLinksResult> {
    hdk::get_links(&hdk::AGENT_ADDRESS, "registered_tag")
}

// TODO Decide the actual details that are needed
pub fn handle_add_app_details(app_details:AppDetails,app_hash:Address) -> ZomeApiResult<Address>{

    let app_details_entry = Entry::App("app_details".into(), app_details.into());
    utils::commit_and_link(&app_details_entry, &app_hash, "details_tag")
}

pub fn handle_get_app_details(app_hash:Address) -> ZomeApiResult<Vec<utils::GetLinksLoadElement<AppDetails>>> {
    utils::get_links_and_load_type(&app_hash, "details_tag")
}

pub fn handle_add_app_domain_name(domain_name:RawString,app_hash:Address) -> ZomeApiResult<Address>{
    let app_domain_name_entry = Entry::App("domain_name".into(), domain_name.into());
    utils::commit_and_link(&app_domain_name_entry, &app_hash, "domain_name_tag")
}

pub fn handle_get_app_domain_name(app_hash:Address) -> ZomeApiResult<Vec<utils::GetLinksLoadElement<String>>> {
    utils::get_links_and_load_type(&app_hash, "domain_name_tag")
}

pub fn handle_register_as_provider(provider_doc:ProviderDoc) -> ZomeApiResult<Address> {
    // TODO : Validation
    let verified_entry = Entry::App("provider_doc".into(), provider_doc.into());
    utils::commit_and_link(&verified_entry, &hdk::AGENT_ADDRESS, "verified_provider_tag")
}

pub fn handle_is_registered_as_provider() -> ZomeApiResult<GetLinksResult> {
    hdk::get_links(&hdk::AGENT_ADDRESS, "verified_provider_tag")
}
