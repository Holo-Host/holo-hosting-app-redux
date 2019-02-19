use crate::entry::app_config::AppConfig;
use crate::entry::app_details::AppDetails;
use crate::entry::provider_doc::ProviderDoc;
use crate::entry::holofuel_account::HoloFuelAc;

use hdk::{
    self,
    holochain_core_types::{
        entry::Entry,
        hash::HashString,
        json::JsonString,
        json::RawString,
        cas::content::Address,
        error::HolochainError,
    },
    holochain_wasm_utils::api_serialization::get_links::GetLinksResult,
    error::ZomeApiResult,
};

#[derive(Serialize, Deserialize, Debug, DefaultJson)]
pub struct DnsDnaKV {
    dna:Vec<HashString>,
    dns:utils::GetLinksLoadElement<String>
}

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

pub fn handle_add_app_domain_name(domain_name:String,app_hash:Address) -> ZomeApiResult<Address>{
    let app_domain_name_entry = Entry::App("domain_name".into(), RawString::from(domain_name).into());
    // Add tag to notify new domain name added
    let new_domain_names_anchor_entry = Entry::App("anchor".into(), RawString::from("New_Domain_Names").into());
    let anchor_address = hdk::commit_entry(&new_domain_names_anchor_entry)?;

    let domain_address = hdk::commit_entry(&app_domain_name_entry)?;

    utils::link_entries_bidir(&domain_address, &app_hash,"app_hash_tag" ,"domain_name_tag")?;
    hdk::link_entries(&anchor_address,&domain_address,"new_domain_name_tag");
    Ok(domain_address)
}

pub fn handle_get_kv_updates_domain_name()-> ZomeApiResult<Vec<DnsDnaKV>> {
    let new_domain_names_anchor_entry = Entry::App("anchor".into(), RawString::from("New_Domain_Names").into());
    let anchor_address = hdk::commit_entry(&new_domain_names_anchor_entry)?;

    let mut dns_dna_kv:Vec<DnsDnaKV>=Vec::new();
    match utils::get_links_and_load_type(&anchor_address, "new_domain_name_tag"){
        Ok(domain_name)=>{
            for dns in domain_name{
                let app_address = hdk::get_links(&dns.address,"app_hash_tag")?;
                dns_dna_kv.push(DnsDnaKV{
                    dna:app_address.addresses().clone(),
                    dns
                });
            }
        }
        Err(_)=>{}
    }
    Ok(dns_dna_kv)
}

pub fn handle_kv_updates_domain_name_completed(dns_address:Address)-> ZomeApiResult<()>{
    let new_domain_names_anchor_entry = Entry::App("anchor".into(), RawString::from("New_Domain_Names").into());
    let anchor_address = hdk::commit_entry(&new_domain_names_anchor_entry)?;

    hdk::remove_link(&anchor_address,&dns_address,"new_domain_name_tag",)?;
    Ok(())
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

pub fn handle_add_holofuel_account(holofuel_account_details:HoloFuelAc) -> ZomeApiResult<Address> {
    let ac_entry = Entry::App("holofuel_account".into(), holofuel_account_details.into());
    utils::commit_and_link(&ac_entry, &hdk::AGENT_ADDRESS, "holofuel_account_details_tag")
}

pub fn handle_get_holofuel_account() -> ZomeApiResult<GetLinksResult> {
    hdk::get_links(&hdk::AGENT_ADDRESS, "holofuel_account_details_tag")
}
