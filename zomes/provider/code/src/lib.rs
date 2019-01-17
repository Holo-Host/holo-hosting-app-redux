#[macro_use]
extern crate hdk;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use hdk::{
    self,
    error::ZomeApiResult,
    holochain_core_types::json::JsonString,
};

pub mod entry::app_config;
pub mod provider_fn;

define_zome! {
    entries: [
        entry::app_config::definitions(),
    ]

    genesis: || { Ok(()) }

    functions: {
        main (Public) {
            register_app: {
                inputs: |config:JsonString|,
                outputs: |sum: ZomeApiResult<JsonString>|,
                handler: provider_fn::handle_register_app
            }
        }
    }
}
