
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
pub struct ProviderDoc {
    pub kyc_proof:String
}

pub fn definitions()-> ValidatingEntryType{
    entry!(
        name: "provider_doc",
        description: "Details for an provider that is verified",
        sharing: Sharing::Public,
        native_type: ProviderDoc,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |_doc: ProviderDoc, _ctx: hdk::ValidationData| {
            Ok(())
        }
        ,
        links: [
            to!(
                "%agent_id",
                tag: "verified_provider_tag",

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
