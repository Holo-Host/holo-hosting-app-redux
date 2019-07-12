use boolinator::Boolinator;

use hdk::{
    self,
    entry_definition::ValidatingEntryType,
    holochain_json_api::{
        error::JsonError,
        json::JsonString,
    },
    holochain_core_types::{
        dna::entry_types::Sharing,
        validation::{EntryValidationData},
    },
};


#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson )]
pub struct HoloFuelAc {
    pub account_number:String,
}

pub fn definitions()-> ValidatingEntryType{
    entry!(
        name: "holofuel_account",
        description: "Details for an providers holofuel account",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |validation_data: hdk::EntryValidationData<HoloFuelAc>| {
            match validation_data
            {
                EntryValidationData::Create{entry:_holofuel_account,validation_data:_} =>
                {
                    Ok(())
                },
                EntryValidationData::Modify{new_entry,old_entry,old_entry_header:_,validation_data:_} =>
                {
                   (new_entry.account_number != old_entry.account_number)
                   .ok_or_else(|| String::from("Trying to modify with same data"))
                },
                EntryValidationData::Delete{old_entry,old_entry_header:_,validation_data:_} =>
                {
                   (old_entry.account_number!="SYS")
                   .ok_or_else(|| String::from("Trying to delete native type with content SYS"))
                }

            }

        },
        links: [
            from!(
                "%agent_id",
                link_type: "holofuel_account_details_tag",

                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },

                validation: | _validation_data: hdk::LinkValidationData | {
                    Ok(())
                }
            )
        ]
    )
}
