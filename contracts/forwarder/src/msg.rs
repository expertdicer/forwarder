use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{StdError, StdResult, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[cw_serde]
#[cfg_attr(test, derive(Default))]
pub struct InstantiateMsg {
}

#[cw_serde]
pub enum ExecuteMsg {
    Forward { recipient: String, amount: Uint128 },
    ForwardToContract { contract: String, recipient: String, amount: Uint128 },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SudoMsg {
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct MigrateMsg {}
