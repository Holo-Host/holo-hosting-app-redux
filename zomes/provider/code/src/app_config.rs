
use hdk::holochain_core_types::{
    dna::entry_types::Sharing,
    error::HolochainError,
    json::JsonString,
    hash::HashString,
};
use hdk::{
    self,
    entry_definition::ValidatingEntryType,
};

#[derive(Serialize, Deserialize, Debug, DefaultJson)]
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
            hdk::ValidationPackageDefinition::ChainFull
        },

        validation: |_app_config: AppConfig, _ctx: hdk::ValidationData| {
            Ok(())
        }
    )
}