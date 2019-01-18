#![feature(try_from)]
#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
//#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate holochain_core_types_derive;

use hdk::{
    error::ZomeApiResult,
    holochain_core_types::{
        error::HolochainError,
        json::JsonString,
        hash::HashString,
        cas::content::Address,
    },
};

pub mod app_config;
pub mod provider_fn;
#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
pub struct App {
    pub uuid:String,
    pub title:String,
    pub author:String,
    pub description:String,
    pub thumbnail:String,
}
define_zome! {
    entries: [
        app_config::definitions()
    ]

    genesis: || { Ok(()) }

    functions: {
        main (Public) {
            register_app: {
                inputs: |ui_hash:HashString, dna_list:Vec<HashString> |,
                outputs: |result: ZomeApiResult<Address>|,
                handler: provider_fn::handle_register_app
            }
        }
    }
}
