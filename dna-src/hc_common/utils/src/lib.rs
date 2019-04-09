#![feature(try_from)]
#[macro_use]
extern crate serde_derive;
use core::convert::TryFrom;
use hdk::{
    self,
    holochain_core_types::{
    	hash::HashString,
    	entry::{AppEntryValue, Entry},
    	cas::content::{Address, AddressableContent},
    },
    error::{ZomeApiResult, ZomeApiError}
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetLinksLoadElement<T> {
	pub address: HashString,
	pub entry: T
}

pub type GetLinksLoadResult<T> = Vec<GetLinksLoadElement<T>>;


pub fn get_links_and_load_type<
	S: Into<String>,
	R: TryFrom<AppEntryValue>
>(
    base: &HashString,
    tag: S
) -> ZomeApiResult<GetLinksLoadResult<R>> {
	let link_load_results = hdk::get_links_and_load(base, tag)?;

	Ok(link_load_results
	.iter()
	.map(|maybe_entry| {

		match maybe_entry {
			Ok(entry) => {
				match entry {
					Entry::App(_, entry_value) => {
						let typed_entry = R::try_from(entry_value.to_owned())
						.map_err(|_| ZomeApiError::Internal(
							"Could not convert get_links result to requested type".to_string())
						)?;

			            Ok(GetLinksLoadElement::<R>{
			                entry: typed_entry,
			                address: entry.to_owned().address()
			            })
					},
					_ => Err(ZomeApiError::Internal(
						"get_links did not return an app entry".to_string())
					)
				}
			},
			_ => Err(ZomeApiError::Internal(
				"get_links did not return an app entry".to_string())
			)
		}
	})
	.filter_map(Result::ok)
	.collect())
}


pub fn get_as_type<
	R: TryFrom<AppEntryValue>
> (address: &HashString) -> ZomeApiResult<R> {
	let get_result = hdk::get_entry(address)?;
    let entry = get_result.ok_or(ZomeApiError::Internal("No entry at this address".into()))?;
    match entry {
        Entry::App(_, entry_value) => {
            R::try_from(entry_value.to_owned())
                .map_err(|_| ZomeApiError::Internal(
                    "Could not convert get_links result to requested type".to_string())
                )
        },
        _ => Err(ZomeApiError::Internal(
            "get_links did not return an app entry".to_string())
        )
    }
}


pub fn link_entries_bidir<S: Into<String>>(a: &HashString, b: &HashString, tag_a_b: S, tag_b_a: S) -> ZomeApiResult<()> {
    hdk::link_entries(a, b, tag_a_b)?;
    hdk::link_entries(b, a, tag_b_a)?;
    Ok(())
}

pub fn remove_link_entries_bidir<S: Into<String>>(a: &HashString, b: &HashString, tag_a_b: S, tag_b_a: S) -> ZomeApiResult<()> {
    hdk::remove_link(a, b, tag_a_b)?;
    hdk::remove_link(b, a, tag_b_a)?;
    Ok(())
}

pub fn commit_and_link<S: Into<String>>(entry: &Entry, base: &Address, tag: S) -> ZomeApiResult<Address> {
	let entry_addr = hdk::commit_entry(entry)?;
	hdk::link_entries(base,&entry_addr, tag)?;
	Ok(entry_addr)
}
