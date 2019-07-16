#[macro_use]
extern crate serde_derive;
use core::convert::TryFrom;

use hdk::{
    self,
    error::{ZomeApiError, ZomeApiResult},
    holochain_core_types::{
        entry::{AppEntryValue, Entry},
        link::LinkMatch,
    },
    holochain_persistence_api::{cas::content::AddressableContent, hash::HashString},
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetLinksLoadElement<T> {
    pub address: HashString,
    pub entry: T,
}

pub type GetLinksLoadResult<T> = Vec<GetLinksLoadElement<T>>;

pub fn get_links_and_load_type<
    // S: Into<String>,
    R: TryFrom<AppEntryValue>,
>(
    base: &HashString,
    tag: String,
) -> ZomeApiResult<GetLinksLoadResult<R>> {
    let link_load_results =
        hdk::get_links_and_load(base, LinkMatch::Exactly(&tag), LinkMatch::Any)?;

    Ok(link_load_results
        .iter()
        .map(|maybe_entry| match maybe_entry {
            Ok(entry) => match entry {
                Entry::App(_, entry_value) => {
                    let typed_entry = R::try_from(entry_value.to_owned()).map_err(|_| {
                        ZomeApiError::Internal(
                            "Could not convert get_links result to requested type".to_string(),
                        )
                    })?;

                    Ok(GetLinksLoadElement::<R> {
                        entry: typed_entry,
                        address: entry.to_owned().address(),
                    })
                }
                _ => Err(ZomeApiError::Internal(
                    "get_links did not return an app entry".to_string(),
                )),
            },
            _ => Err(ZomeApiError::Internal(
                "get_links did not return an app entry".to_string(),
            )),
        })
        .filter_map(Result::ok)
        .collect())
}
