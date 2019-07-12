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
pub struct AppDetails {
    pub name:String,
    pub details:String,
    // pub domain_name:String
}

pub fn definitions()-> ValidatingEntryType{
    entry!(
        name: "app_details",
        description: "Details for an app",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: |validation_data: hdk::EntryValidationData<AppDetails>| {
            match validation_data
            {
                EntryValidationData::Create{entry:_app_details,validation_data:_} =>
                {
                    Ok(())
                },
                EntryValidationData::Modify{new_entry,old_entry,old_entry_header:_,validation_data:_} =>
                {
                   (new_entry.name != old_entry.name)
                   .ok_or_else(|| String::from("Trying to modify with same data"))
                },
                EntryValidationData::Delete{old_entry,old_entry_header:_,validation_data:_} =>
                {
                   (old_entry.name!="SYS")
                   .ok_or_else(|| String::from("Trying to delete native type with content SYS"))
                }

            }

        },
        links: [
            from!(
                "app_config",
                link_type: "details_tag",

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
