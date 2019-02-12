
use hdk::holochain_core_types::{
    dna::entry_types::Sharing,
    error::HolochainError,
    json::JsonString,
    // hash::HashString,
    cas::content::Address

};
use hdk::{
    self,
    entry_definition::ValidatingEntryType,
};

#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
pub struct AppDetails {
    pub details:String,
    // pub domain_name:String
}

pub fn definitions()-> ValidatingEntryType{
    entry!(
        name: "app_details",
        description: "Details for an app",
        sharing: Sharing::Public,
        native_type: AppDetails,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |_app: AppDetails, _ctx: hdk::ValidationData| {
            Ok(())
        }
        ,
        links: [
            from!(
                "app_config",
                tag: "details_tag",

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
