
use hdk::holochain_core_types::{
    dna::entry_types::Sharing,
    cas::content::Address,
    json::RawString,
};
use hdk::{
    self,
    entry_definition::ValidatingEntryType,
};


pub fn definitions()-> ValidatingEntryType{
    entry!(
        name: "domain_name",
        description: "domain_name for an app",
        sharing: Sharing::Public,
        native_type: RawString,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |_dn: RawString, _ctx: hdk::ValidationData| {
            Ok(())
        }
        ,
        links: [
            from!(
                "app_config",
                tag: "domain_name_tag",

                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },

                validation: |_base: Address, _target: Address, _ctx: hdk::ValidationData| {
                    Ok(())
                }
            ),
            from!(
                "app_config",
                tag: "new_domain_name_tag",

                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },

                validation: |_base: Address, _target: Address, _ctx: hdk::ValidationData| {
                    Ok(())
                }
            ),
            to!(
                "app_config",
                tag: "app_hash_tag",

                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },

                validation: |_base: Address, _target: Address, _ctx: hdk::ValidationData| {
                    Ok(())
                }
            ),
            from!(
                "anchor",
                tag: "new_domain_name_tag",

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
