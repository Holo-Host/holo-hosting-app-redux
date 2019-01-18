#![feature(try_from)]
#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate utils;
#[macro_use]
extern crate holochain_core_types_derive;

pub mod categories_fn;

use crate::categories_fn::AppConfig;
use hdk::{
    holochain_core_types::{
        cas::content::Address,
        dna::entry_types::Sharing,
        json::RawString
    },
    error::ZomeApiResult
};

define_zome! {
    entries: [
        entry!(
            name: "category_anchor",
            description: "",
            sharing: Sharing::Public,
            native_type: RawString,

            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },

            validation: |_name: RawString, _ctx: hdk::ValidationData| {
                Ok(())
            },

            links: [
                to!(
                    "app_config",
                    tag: "",

                    validation_package: || {
                        hdk::ValidationPackageDefinition::Entry
                    },

                    validation: |_base: Address, _target: Address, _ctx: hdk::ValidationData| {
                        Ok(())
                    }
                ),
                from!(
                    "app_config",
                    tag: "",

                    validation_package: || {
                        hdk::ValidationPackageDefinition::Entry
                    },

                    validation: |_base: Address, _target: Address, _ctx: hdk::ValidationData| {
                        Ok(())
                    }
                )
            ]
        ),
        entry!(
            name: "tag_anchor",
            description: "",
            sharing: Sharing::Public,
            native_type: RawString,

            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },

            validation: |_name: RawString, _ctx: hdk::ValidationData| {
                Ok(())
            },

            links: [
                to!(
                    "app_config",
                    tag: "",

                    validation_package: || {
                        hdk::ValidationPackageDefinition::Entry
                    },

                    validation: |_base: Address, _target: Address, _ctx: hdk::ValidationData| {
                        Ok(())
                    }
                ),
                from!(
                    "app_config",
                    tag: "",

                    validation_package: || {
                        hdk::ValidationPackageDefinition::Entry
                    },

                    validation: |_base: Address, _target: Address, _ctx: hdk::ValidationData| {
                        Ok(())
                    }
                )
            ]
        )
    ]

    genesis: || { Ok(()) }

    functions: {
        main (Public) {
            add_category: {
                inputs:| category:RawString,tag:RawString, hash:hdk::holochain_core_types::hash::HashString |,
                outputs: | result: ZomeApiResult<()> |,
                handler: categories_fn::handle_adding_category
            }
            get_apps_by_category: {
                inputs:| category:RawString |,
                outputs: | result: ZomeApiResult<Vec<utils::GetLinksLoadElement<AppConfig>>> |,
                handler: categories_fn::handle_get_apps_by_category
            }
        }
    }
}
