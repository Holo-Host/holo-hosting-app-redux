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
    holochain_core_types::{json::JsonString, error::HolochainError},
    holochain_wasm_utils::api_serialization::get_links::GetLinksResult,
};
use hdk::holochain_core_types::{
    hash::HashString,
    cas::content::Address,
    entry::Entry,
};

pub mod host_fn;
pub mod entry;

define_zome! {
    entries: [
    entry::host_doc::definitions()
    ]

    genesis: || { Ok(()) }

    functions: [
            get_all_apps: {
                inputs: | |,
                outputs: |result: ZomeApiResult<GetLinksResult>|,
                handler: host_fn::handle_get_all_apps
            }
            enable_app: {
                inputs: |app_hash: HashString|,
                outputs: |result: ZomeApiResult<()>|,
                handler: host_fn::handle_enable_app
            }
            disable_app: {
                inputs: |app_hash: HashString|,
                outputs: |result: ZomeApiResult<()>|,
                handler: host_fn::handle_disable_app
            }
            get_enabled_app: {
                inputs: | |,
                outputs: |result: ZomeApiResult<Vec<utils::GetLinksLoadElement<host_fn::AppConfig>>>|,
                handler: host_fn::handle_get_enabled_app
            }
            get_host_for_app: {
                inputs: | app_hash:Address |,
                outputs: |result: ZomeApiResult<Vec<ZomeApiResult<Entry>>>|,
                handler: host_fn::handle_get_host_for_app
            }
            register_as_host: {
                inputs: |host_doc:entry::host_doc::HostDoc |,
                outputs: |result: ZomeApiResult<Address>|,
                handler: host_fn::handle_register_as_host
            }
            is_registered_as_host: {
                inputs: | |,
                outputs: |result:  ZomeApiResult<GetLinksResult>|,
                handler: host_fn::handle_is_registered_as_host
            }
            get_kv_updates_dna_to_host: {
                inputs: | |,
                outputs: |result: ZomeApiResult<host_fn::DnaToHost>|,
                handler: host_fn::handle_get_kv_updates_dna_to_host
            }
            kv_updates_host_completed: {
                inputs: |kv_bundle:Vec<host_fn::App2Host> |,
                outputs: |result: ZomeApiResult<()>|,
                handler: host_fn::handle_kv_updates_host_completed
            }
        ]

        traits: {
               hc_public [
                   get_all_apps,
                   enable_app,
                   disable_app,
                   get_enabled_app,
                   get_host_for_app,
                   register_as_host,
                   is_registered_as_host,
                   get_kv_updates_dna_to_host,
                   kv_updates_host_completed
                  ]
           }
}