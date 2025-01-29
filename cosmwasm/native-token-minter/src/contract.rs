use cosmwasm_schema::cw_serde;
use cosmwasm_std::{
    entry_point, to_json_binary, BankMsg, Binary, Coin, CosmosMsg, Deps, DepsMut, Env, MessageInfo,
    QueryRequest, Response, StdResult,
};
use token_factory_api::{TokenFactoryMsg, TokenFactoryQuery};
use ucs03_zkgm_token_minter_api::{
    BaseTokenResponse, ExecuteMsg, LocalTokenMsg, MetadataResponse, QueryMsg,
};

use crate::{error::Error, state::ADMIN};

#[cw_serde]
pub enum TokenMinterInitMsg {
    Native,
}

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _: Env,
    info: MessageInfo,
    _: TokenMinterInitMsg,
) -> StdResult<Response> {
    ADMIN.save(deps.storage, &info.sender)?;
    Ok(Response::default())
}

#[cw_serde]
pub struct MigrateMsg {}

#[entry_point]
pub fn migrate(_: DepsMut, _: Env, _: MigrateMsg) -> StdResult<Response> {
    Ok(Response::new())
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<TokenFactoryMsg>, Error> {
    if info.sender != ADMIN.load(deps.storage)? {
        return Err(Error::OnlyAdmin);
    }

    let resp = match msg {
        ExecuteMsg::Wrapped(msg) => {
            let msg: TokenFactoryMsg = msg.into();
            if let TokenFactoryMsg::BurnTokens { denom, amount, .. } = &msg {
                let contains_base_token = info
                    .funds
                    .iter()
                    .any(|coin| &coin.denom == denom && coin.amount == amount);
                if !contains_base_token {
                    return Err(Error::MissingFunds {
                        denom: denom.clone(),
                        amount: *amount,
                    });
                }
            }
            Response::new().add_message(CosmosMsg::Custom(msg))
        }
        ExecuteMsg::Local(msg) => match msg {
            LocalTokenMsg::Escrow { denom, amount, .. } => {
                let contains_base_token = info
                    .funds
                    .iter()
                    .any(|coin| coin.denom == denom && coin.amount == amount);
                if !contains_base_token {
                    return Err(Error::MissingFunds { denom, amount });
                }
                Response::new()
            }
            LocalTokenMsg::Unescrow {
                denom,
                recipient,
                amount,
            } => Response::new().add_message(BankMsg::Send {
                to_address: recipient,
                amount: vec![Coin { denom, amount }],
            }),
        },
    };
    Ok(resp)
}

#[entry_point]
pub fn query(deps: Deps<TokenFactoryQuery>, _: Env, msg: QueryMsg) -> Result<Binary, Error> {
    match msg {
        QueryMsg::BaseToken { base_token } => {
            Ok(to_json_binary(&BaseTokenResponse { base_token })?)
        }
        QueryMsg::Metadata { denom } => {
            let denom_metadata =
                deps.querier
                    .query::<token_factory_api::MetadataResponse>(
                        &QueryRequest::<TokenFactoryQuery>::Custom(TokenFactoryQuery::Metadata {
                            denom: denom.clone(),
                        }),
                    );
            let (name, symbol) = match denom_metadata {
                Ok(token_factory_api::MetadataResponse {
                    metadata: Some(metadata),
                }) => (
                    metadata.name.unwrap_or(denom.clone()),
                    metadata.symbol.unwrap_or(denom),
                ),
                _ => (denom.clone(), denom),
            };

            Ok(to_json_binary(&MetadataResponse { name, symbol })?)
        }
    }
}
