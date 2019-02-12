
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
pub struct HoloFuelAc {
    pub account_number:String,
}

pub fn definitions()-> ValidatingEntryType{
    entry!(
        name: "holofuel_account",
        description: "Details for an providers holofuel account",
        sharing: Sharing::Public,
        native_type: HoloFuelAc,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |_app: HoloFuelAc, _ctx: hdk::ValidationData| {
            // TODO : Check if already exists
            Ok(())
        }
        ,
        links: [
            from!(
                "%agent_id",
                tag: "holofuel_account_details_tag",

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
