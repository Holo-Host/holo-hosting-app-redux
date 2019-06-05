use boolinator::Boolinator;

use hdk::holochain_core_types::{
    dna::entry_types::Sharing,
    error::HolochainError,
    json::JsonString,
    validation::{EntryValidationData},

};
use hdk::{
    self,
    entry_definition::ValidatingEntryType,
};


#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
pub struct DNS {
    pub dns_name:String,
}
pub fn definitions()-> ValidatingEntryType{
    entry!(
        name: "domain_name",
        description: "domain_name for an app",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: |validation_data: hdk::EntryValidationData<DNS>| {
            match validation_data
            {
                EntryValidationData::Create{entry:_domain_name,validation_data:_} =>
                {
                    Ok(())
                },
                EntryValidationData::Modify{new_entry,old_entry,old_entry_header:_,validation_data:_} =>
                {
                   (new_entry.dns_name != old_entry.dns_name)
                   .ok_or_else(|| String::from("Trying to modify with same data"))
                },
                EntryValidationData::Delete{old_entry,old_entry_header:_,validation_data:_} =>
                {
                   (old_entry.dns_name!="SYS")
                   .ok_or_else(|| String::from("Trying to delete native type with content SYS"))
                }

            }

        },
        links: [
            from!(
                "app_config",
                link_type: "domain_name_tag",

                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },

                validation: | _validation_data: hdk::LinkValidationData | {
                    Ok(())
                }
            ),
            from!(
                "app_config",
                link_type: "new_domain_name_tag",

                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },

                validation: | _validation_data: hdk::LinkValidationData | {
                    Ok(())
                }
            ),
            from!(
                "app_config",
                link_type: "need_update_domain_name_tag",

                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },

                validation: | _validation_data: hdk::LinkValidationData | {
                    Ok(())
                }
            ),
            to!(
                "app_config",
                link_type: "app_hash_tag",

                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },

                validation: | _validation_data: hdk::LinkValidationData | {
                    Ok(())
                }
            ),
            from!(
                "anchor",
                link_type: "new_domain_name_tag",

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
