
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
pub struct HostDoc {
    pub kyc_proof:String
}

pub fn definitions()-> ValidatingEntryType{
    entry!(
        name: "host_doc",
        description: "Details for an host that is verified",
        sharing: Sharing::Public,
        native_type: HostDoc,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |_doc: HostDoc, _ctx: hdk::ValidationData| {
            Ok(())
        }
        ,
        links: [
            from!(
                "%agent_id",
                tag: "verified_host_tag",

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
