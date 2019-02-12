use hdk::{
    AGENT_ADDRESS,
    AGENT_ID_STR,
    error::ZomeApiResult,
    holochain_core_types::{
        json::JsonString,
        error::HolochainError,
    }
};

#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
pub struct UserDetails {
    hash: String,
    name: String,
}

pub fn handle_get_agent() -> ZomeApiResult<UserDetails> {
    Ok(UserDetails {
        hash: AGENT_ADDRESS.to_string(),
        name: AGENT_ID_STR.to_string()
    })
}
