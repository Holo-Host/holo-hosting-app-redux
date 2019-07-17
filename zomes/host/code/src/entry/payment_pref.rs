use hdk::{
    self,
    entry_definition::ValidatingEntryType,
    holochain_core_types::{dna::entry_types::Sharing, validation::EntryValidationData},
    holochain_json_api::{error::JsonError, json::JsonString},
    holochain_persistence_api::{cas::content::Address, hash::HashString},
};

#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
pub struct PaymentPref {
    pub provider_address: Address,
    pub dna_bundle_hash: HashString,
    pub max_fuel_per_invoice: f64,
    pub max_unpaid_value: f64,
    pub price_per_unit: f64,
}

// TODO: maybe have an Anchor to point to the latest prefs object?
pub fn definitions() -> ValidatingEntryType {
    entry!(
        name: "payment_pref",
        description: "the payment preferences defintion",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: |validation_data: hdk::EntryValidationData<PaymentPref>| {
            match validation_data
            {
                EntryValidationData::Create{entry:_payment_pref,validation_data:_} =>
                {
                    Ok(())
                },
                EntryValidationData::Modify{new_entry:_new_entry,old_entry:_old_entry,old_entry_header:_,validation_data:_} =>
                {
                   Ok(())
                },
                EntryValidationData::Delete{old_entry:_old_entry,old_entry_header:_,validation_data:_} =>
                {
                  Ok(())
                }

            }

        },
        links: [
            from!(
                "app_config",
                link_type: "payment_pref_tag",

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
