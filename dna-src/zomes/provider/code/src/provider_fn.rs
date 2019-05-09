use crate::entry::app_config::AppConfig;
use crate::entry::provider_doc::ProviderDoc;
use crate::entry::holofuel_account::HoloFuelAc;
use crate::entry::domain_name::DNS;

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
    error::{ZomeApiResult,ZomeApiError},
};

#[derive(Serialize, Deserialize, Debug, DefaultJson)]
pub struct DnsDnaKV {
    dna:HashString,
    dns:Vec<DNSEntry>
}

#[derive(Serialize, Deserialize, Debug, DefaultJson)]
pub struct DNSEntry {
    address:HashString,
    name:String
}

pub fn validate_provider() -> ZomeApiResult<bool> {
    let check = handle_is_registered_as_provider()?;
    if check.addresses().len() != 0 {
        Ok(true)
    }
    else {
        Err(ZomeApiError::Internal("Agent Not a Provider".to_string()))
    }
}

pub fn handle_register_app(app_bundle:AppConfig, domain_name:DNS) -> ZomeApiResult<Address> {
    let app_hash = handle_register_app_bundle(app_bundle)?;
    // handle_add_app_details(app_details, &app_hash)?;
    handle_add_app_domain_name(domain_name, &app_hash)?;
    Ok(app_hash)
}

pub fn handle_register_app_bundle(app_bundle:AppConfig) -> ZomeApiResult<Address> {
    // TODO
    // Validation before commiting to the DHT
    // Check if user is verified
    // Check if all the hashes exist in the HCHC
    validate_provider()?;
    let app_entry = Entry::App("app_config".into(), app_bundle.into());

    // CREATING AN ANCHOR
    let all_apps = Entry::App("anchor".into(), RawString::from("ALL_APPS").into());
    let anchor_address = hdk::commit_entry(&all_apps)?;

    utils::commit_and_link(&app_entry, &anchor_address, "all_apps_tag")?;
    utils::commit_and_link(&app_entry, &hdk::AGENT_ADDRESS, "my_registered_apps_tag")
}

pub fn handle_get_my_registered_app_list() -> ZomeApiResult<GetLinksResult> {
    validate_provider()?;
    hdk::get_links(&hdk::AGENT_ADDRESS, "my_registered_apps_tag")
}
//
// // TODO Decide the actual details that are needed
// pub fn handle_add_app_details(app_details:AppDetails, app_hash:&Address) -> ZomeApiResult<Address>{
//     validate_provider()?;
//     let app_details_entry = Entry::App("app_details".into(), app_details.into());
//     utils::commit_and_link(&app_details_entry, app_hash, "details_tag")
// }

// Copy of the PaymentPref in the host zome
#[derive(Serialize, Deserialize, DefaultJson, Debug, Clone)]
pub struct PaymentPref {
    pub provider_address: Address,
    pub dna_bundle_hash: HashString,
    pub max_fuel_per_invoice: f64,
    pub max_unpaid_value: f64,
    pub price_per_unit: f64,
}

#[derive(Serialize, Deserialize, Debug, DefaultJson)]
pub struct AppBundle{
    pub app_bundle:AppConfig,
    // pub app_details:Vec<utils::GetLinksLoadElement<AppDetails>>,
    pub payment_pref:Vec<utils::GetLinksLoadElement<PaymentPref>>
}

pub fn handle_get_app_details(app_hash:Address) -> ZomeApiResult<AppBundle> {
    Ok(AppBundle{
        // get app AppConfig
        app_bundle:utils::get_as_type(&app_hash)?,
        // get app details_tag
        // app_details: utils::get_links_and_load_type(&app_hash, "details_tag")?,
        // get app servicelog details ir
        payment_pref: utils::get_links_and_load_type(&app_hash, "payment_pref_tag")?
    })
}

pub fn handle_add_app_domain_name(domain_name:DNS, app_hash:&Address) -> ZomeApiResult<Address>{
    validate_provider()?;
    let app_domain_name_entry = Entry::App("domain_name".into(), domain_name.into());
    // Add tag to notify new domain name added
    // let new_domain_names_anchor_entry = Entry::App("anchor".into(), RawString::from("New_Domain_Names").into());
    // let anchor_address = hdk::commit_entry(&new_domain_names_anchor_entry)?;

    let domain_address = hdk::commit_entry(&app_domain_name_entry)?;

    utils::link_entries_bidir(&domain_address, app_hash,"app_hash_tag" ,"domain_name_tag")?;
    hdk::link_entries(app_hash,&domain_address,"new_domain_name_tag")?;
    Ok(domain_address)
}

pub fn handle_get_all_apps() -> ZomeApiResult<GetLinksResult> {
    let all_apps = Entry::App("anchor".into(), RawString::from("ALL_APPS").into());
    let anchor_address = hdk::commit_entry(&all_apps)?;
    hdk::get_links(&anchor_address, "all_apps_tag")
}


// pub fn handle_get_kv_updates_dna_to_host()-> ZomeApiResult<DnaToHost> {
pub fn handle_get_kv_updates_domain_name()-> ZomeApiResult<Vec<DnsDnaKV>> {
        // Get all the apps
        let got_apps:GetLinksResult = handle_get_all_apps()?;
        let all_apps = got_apps.addresses().to_vec();
        // Check the enabled tag
        let mut recently_updated_dns:Vec<DnsDnaKV>=Vec::new();
        for app in all_apps.clone(){
            let app_copy = app.clone();
            let updated_dns:Vec<utils::GetLinksLoadElement<DNS>> = utils::get_links_and_load_type(&app_copy, "new_domain_name_tag")?;
            // Data refactor
            let mut dns_list:Vec<DNSEntry>=Vec::new();
            for dns in updated_dns.clone(){
                dns_list.push(DNSEntry{
                    address:dns.address,
                    name:dns.entry.dns_name
                })
            }

            recently_updated_dns.push(DnsDnaKV{
                dna:app,
                dns:dns_list
            });

            // Remove the new_domain_name tag and add intransition apps
            for dns in &updated_dns{
                hdk::remove_link(&app_copy,&dns.address,"new_domain_name_tag")?;
                hdk::link_entries(&app_copy,&dns.address ,"need_update_domain_name_tag")?;
            }
        }
        Ok(recently_updated_dns)
}

pub fn handle_kv_updates_domain_name_completed(kv_bundle:Vec<DnsDnaKV>)-> ZomeApiResult<()>{
    for kv in kv_bundle{
        for dns in kv.dns {
            hdk::remove_link(&kv.dna,&dns.address.clone(),"need_update_domain_name_tag")?;
        }
    }
    Ok(())
}

pub fn handle_get_app_domain_name(app_hash:Address) -> ZomeApiResult<Vec<utils::GetLinksLoadElement<DNS>>> {
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
