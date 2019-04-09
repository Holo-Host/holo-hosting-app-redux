
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
pub struct HostDoc {
    pub kyc_proof:String
}

pub fn definitions()-> ValidatingEntryType{
    entry!(
        name: "host_doc",
        description: "Details for an host that is verified",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |validation_data: hdk::EntryValidationData<HostDoc>| {
            match validation_data
            {
                EntryValidationData::Create{entry:_domain_name,validation_data:_} =>
                {
                    Ok(())
                },
                EntryValidationData::Modify{new_entry:_,old_entry:_,old_entry_header:_,validation_data:_} =>
                {
                   Ok(())
                },
                EntryValidationData::Delete{old_entry:_,old_entry_header:_,validation_data:_} =>
                {
                   Ok(())
                }

            }

        },
        links: [
            from!(
                "%agent_id",
                tag: "verified_host_tag",

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
