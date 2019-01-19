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
};
use hdk::holochain_core_types::{
    hash::HashString,
    cas::content::Address,
    entry::Entry,
};

pub mod host_fn;

define_zome! {
    entries: []

    genesis: || { Ok(()) }

    functions: {
        main (Public) {
            enable_app: {
                inputs: |app_hash: HashString|,
                outputs: |result: ZomeApiResult<()>|,
                handler: host_fn::handle_enable_app
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
        }
    }
}
