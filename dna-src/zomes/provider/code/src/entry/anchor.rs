use boolinator::Boolinator;

use hdk::{
    self,
    entry_definition::ValidatingEntryType,
    holochain_core_types::{dna::entry_types::Sharing, validation::EntryValidationData},
    holochain_json_api::json::{JsonString, RawString},
};

pub fn definitions() -> ValidatingEntryType {
    entry!(
        name: "anchor",
        description: "anchor for an app",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: |validation_data: hdk::EntryValidationData<RawString>| {
            match validation_data
            {
                EntryValidationData::Create{entry:_anchor,validation_data:_} =>
                {
                    Ok(())
                },
                EntryValidationData::Modify{new_entry,old_entry,old_entry_header:_,validation_data:_} =>
                {
                   (new_entry != old_entry)
                   .ok_or_else(|| String::from("Trying to modify with same data"))
                },
                EntryValidationData::Delete{old_entry:_old_entry,old_entry_header:_,validation_data:_} =>
                {
                   Ok(())
                }

            }

        },
        links: []
    )
}
