#![feature(try_from)]
#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate holochain_core_types_derive;

use hdk::{
    error::ZomeApiResult,
};
use hdk::holochain_core_types::{
    hash::HashString,
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
            // disable_app: {
            //     inputs: |address: Address|,
            //     outputs: |result: ZomeApiResult<Address>|,
            //     handler: host_fn::handle_disable_app
            // }
        }
    }
}
