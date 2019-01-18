// TODO: Get the categories of the a app hash/address
use hdk::{
    self,
    holochain_core_types::{
        hash::HashString,
        entry::Entry,
        error::HolochainError,
        json::JsonString,
        json::RawString,
        cas::content::AddressableContent,
    },
    error::{ZomeApiResult},
};

#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
pub struct AppConfig {
    pub ui_hash:HashString,
    pub dna_list:Vec<HashString>,
}

// links a tag and app bidirectionally 
// links a catagory and an app bidirectionally
pub fn handle_adding_category(category:RawString, tag:RawString, hash:HashString) -> ZomeApiResult<()> {
    let category_anchor_entry = Entry::App(
        "category_anchor".into(),
        category.into()
    );
    let category_anchor_addr = hdk::commit_entry(&category_anchor_entry)?;

    let tag_anchor_entry = Entry::App(
        "tag_anchor".into(),
        tag.into()
    );
    let tag_anchor_addr = hdk::commit_entry(&tag_anchor_entry)?;
   
    utils::link_entries_bidir(&hash, &category_anchor_addr, "", "")?;
    utils::link_entries_bidir(&hash, &tag_anchor_addr, "", "")?;
    Ok(())
}


pub fn handle_get_apps_by_category(category: RawString) -> ZomeApiResult<Vec<utils::GetLinksLoadElement<AppConfig>>> {
    let category_anchor_entry = Entry::App(
        "category_anchor".into(),
        category.into()
    );

    utils::get_links_and_load_type(&category_anchor_entry.address(), "")
}
