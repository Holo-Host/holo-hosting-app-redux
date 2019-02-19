#![feature(try_from)]
#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate holochain_core_types_derive;

use hdk::{
    error::ZomeApiResult,
    holochain_core_types::{
        hash::HashString,
        cas::content::Address,
        error::HolochainError,
        json::JsonString,
    },
    holochain_wasm_utils::api_serialization::get_links::GetLinksResult,
};

pub mod entry;
pub mod provider_fn;

define_zome! {
    entries: [
        entry::app_config::definitions(),
        entry::app_details::definitions(),
        entry::provider_doc::definitions(),
        entry::domain_name::definitions(),
        entry::holofuel_account::definitions(),
        entry::anchor::definitions()
    ]

    genesis: || {
        Ok(())
    }

    functions: [
            register_as_provider: {
                inputs: |provider_doc:entry::provider_doc::ProviderDoc |,
                outputs: |result: ZomeApiResult<Address>|,
                handler: provider_fn::handle_register_as_provider
            }
            is_registered_as_provider: {
                inputs: | |,
                outputs: |result:  ZomeApiResult<GetLinksResult>|,
                handler: provider_fn::handle_is_registered_as_provider
            }
            register_app: {
                inputs: |ui_hash:HashString, dna_list:Vec<HashString> |,
                outputs: |result: ZomeApiResult<Address>|,
                handler: provider_fn::handle_register_app
            }
            get_my_registered_app: {
                inputs: | |,
                outputs: |result: ZomeApiResult<GetLinksResult>|,
                handler: provider_fn::handle_get_my_registered_app
            }
            add_app_details: {
                inputs: |app_details: entry::app_details::AppDetails, app_hash:Address |,
                outputs: |result: ZomeApiResult<Address>|,
                handler: provider_fn::handle_add_app_details
            }
            get_app_details: {
                inputs: |app_hash:Address |,
                outputs: |result: ZomeApiResult<Vec<utils::GetLinksLoadElement<entry::app_details::AppDetails>>> |,
                handler: provider_fn::handle_get_app_details
            }
            add_holofuel_account: {
                inputs: |holofuel_account_details: entry::holofuel_account::HoloFuelAc|,
                outputs: |result: ZomeApiResult<Address>|,
                handler: provider_fn::handle_add_holofuel_account
            }
            get_holofuel_account: {
                inputs: | |,
                outputs: |result: ZomeApiResult<GetLinksResult>|,
                handler: provider_fn::handle_get_holofuel_account
            }
            add_app_domain_name: {
                inputs: |domain_name:String, app_hash:Address |,
                outputs: |result: ZomeApiResult<Address>|,
                handler: provider_fn::handle_add_app_domain_name
            }
            get_app_domain_name: {
                inputs: |app_hash:Address |,
                outputs: |result: ZomeApiResult<Vec<utils::GetLinksLoadElement<String>>> |,
                handler: provider_fn::handle_get_app_domain_name
            }
            get_kv_updates_domain_name: {
                inputs: | |,
                outputs: |result: ZomeApiResult<Vec<provider_fn::DnsDnaKV>> |,
                handler: provider_fn::handle_get_kv_updates_domain_name
            }
            kv_updates_domain_name_completed: {
                inputs: |dns_address:Vec<Address> |,
                outputs: |result: ZomeApiResult<()> |,
                handler: provider_fn::handle_kv_updates_domain_name_completed
            }
        ]
        traits: {
               hc_public [
                   register_as_provider,
                   is_registered_as_provider,
                   register_app,
                   get_my_registered_app,
                   add_app_details,
                   get_app_details,
                   add_holofuel_account,
                   get_holofuel_account,
                   add_app_domain_name,
                   get_app_domain_name,
                   get_kv_updates_domain_name,
                   kv_updates_domain_name_completed
                  ]
           }
}
