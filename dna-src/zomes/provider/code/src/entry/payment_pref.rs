
use hdk::holochain_core_types::{
    dna::entry_types::Sharing,
    json::JsonString,
    cas::content::Address,
    error::HolochainError,
    hash::HashString,
};
use hdk::{
    self,
    entry_definition::ValidatingEntryType,
};

#[derive(Serialize, Deserialize, DefaultJson, Debug)]
pub struct PaymentPref {
    pub provider_address: Address,
    pub dna_bundle_hash: HashString,
    pub max_fuel_per_invoice: f64,
    pub max_unpaid_value: f64,
}

// TODO: maybe have an Anchor to point to the latest prefs object?
pub fn definitions() -> ValidatingEntryType {
    entry!(
        name: "payment_pref",
        description: "the payment preferences defintion",
        sharing: Sharing::Public,
        native_type: PaymentPref,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |_my_entry: PaymentPref, _validation_data: hdk::ValidationData| {
            Ok(())
        },
        links: [
            from!(
                "app_config",
                tag: "payment_pref_tag",

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
