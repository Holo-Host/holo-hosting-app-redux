use hdk::{
    error::ZomeApiResult,
    holochain_json_api::{error::JsonError, json::JsonString},
    AGENT_ADDRESS, AGENT_ID_STR,
};

#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
pub struct UserDetails {
    hash: String,
    name: String,
}

pub fn handle_get_agent() -> ZomeApiResult<UserDetails> {
    Ok(UserDetails {
        hash: AGENT_ADDRESS.to_string(),
        name: AGENT_ID_STR.to_string(),
    })
}
