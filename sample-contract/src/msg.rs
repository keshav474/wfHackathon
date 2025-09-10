use cosmwasm_std::Binary;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub oracle_pubkey: Binary,
    pub oracle_key_type: String, // "secp256k1" or "ed25519"
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Send { recipient: String },
    OracleDataUpdate { data: String, signature: Binary },
    UpdateOracle { new_pubkey: Binary, new_key_type: Option<String> },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetOracleData {},
    GetOraclePubkey {},
    GetAdmin {},
}
