
use hdk::holochain_core_types::{
    dna::entry_types::Sharing,
    error::HolochainError,
    json::JsonString,
    hash::HashString,
    cas::content::Address
    
};
use hdk::{
    self,
    entry_definition::ValidatingEntryType,
};

#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
pub struct AppConfig {
    pub ui_hash:HashString,
    pub dna_list:Vec<HashString>,
}

pub fn definitions()-> ValidatingEntryType{
    entry!(
        name: "app_config",
        description: "config for an app",
        sharing: Sharing::Public,
        native_type: AppConfig,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |_app: AppConfig, _ctx: hdk::ValidationData| {
            Ok(())
        }
        ,
        links: [
            to!(
                "%agent_id",
                tag: "registered_tag",

                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },

                validation: |_base: Address, _target: Address, _ctx: hdk::ValidationData| {
                    Ok(())
                }
            )
        ]
    )
}

