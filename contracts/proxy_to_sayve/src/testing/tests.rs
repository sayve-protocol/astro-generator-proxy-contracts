use crate::contract::{execute, instantiate, query};
use crate::error::ContractError;
use crate::state::{Config, CONFIG};
use crate::testing::mock_querier::mock_dependencies;
use astroport::generator_proxy::{CallbackMsg, Cw20HookMsg, ExecuteMsg, InstantiateMsg, QueryMsg};
use cosmwasm_std::testing::{mock_env, mock_info, MOCK_CONTRACT_ADDR};
use cosmwasm_std::{from_binary, to_binary, Addr, CosmosMsg, SubMsg, Uint128, WasmMsg};
use cw20::{Cw20ExecuteMsg, Cw20ReceiveMsg};
use sayve::lp_staking::execute_msgs::{
    Cw20HookMsg as SayveCw20HookMsg, ExecuteMsg as SayveExecuteMsg,
};

#[test]
fn test_proper_initialization() {
    let mut deps = mock_dependencies(&[]);

    let msg = InstantiateMsg {
        generator_contract_addr: "generator0000".to_string(),
        pair_addr: "pair0000".to_string(),
        lp_token_addr: "sayveust0000".to_string(),
        reward_contract_addr: "reward0000".to_string(),
        reward_token_addr: "sayve0000".to_string(),
    };

    let info = mock_info("addr0000", &[]);
    let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    let config: Config = CONFIG.load(deps.as_ref().storage).unwrap();
    assert_eq!("generator0000", config.generator_contract_addr.as_str());
    assert_eq!("pair0000", config.pair_addr.as_str());
    assert_eq!("sayveust0000", config.lp_token_addr.as_str());
    assert_eq!("reward0000", config.reward_contract_addr.as_str());
    assert_eq!("sayve0000", config.reward_token_addr.as_str());
}

#[test]
fn test_deposit() {
    let mut deps = mock_dependencies(&[]);

    let msg = InstantiateMsg {
        generator_contract_addr: "generator0000".to_string(),
        pair_addr: "pair0000".to_string(),
        lp_token_addr: "sayveust0000".to_string(),
        reward_contract_addr: "reward0000".to_string(),
        reward_token_addr: "sayve0000".to_string(),
    };

    let info = mock_info("addr0000", &[]);
    let _res = instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

    // deposit fails when not sent by LP token
    let deposit_msg = ExecuteMsg::Receive(Cw20ReceiveMsg {
        sender: "generator0000".to_string(),
        amount: Uint128::from(100u128),
        msg: to_binary(&Cw20HookMsg::Deposit {}).unwrap(),
    });

    let res = execute(deps.as_mut(), mock_env(), info, deposit_msg).unwrap_err();
    match res {
        ContractError::Unauthorized {} => {}
        _ => panic!("Must return unauthorized error"),
    };

    // deposit fails when cw20 sender is not generator
    let info = mock_info("sayveust0000", &[]);
    let deposit_msg = ExecuteMsg::Receive(Cw20ReceiveMsg {
        sender: "addr0000".to_string(),
        amount: Uint128::from(100u128),
        msg: to_binary(&Cw20HookMsg::Deposit {}).unwrap(),
    });

    let res = execute(deps.as_mut(), mock_env(), info, deposit_msg).unwrap_err();
    match res {
        ContractError::Unauthorized {} => {}
        _ => panic!("Must return unauthorized error"),
    };

    // successfull deposit
    let info = mock_info("sayveust0000", &[]);
    let deposit_msg = ExecuteMsg::Receive(Cw20ReceiveMsg {
        sender: "generator0000".to_string(),
        amount: Uint128::from(100u128),
        msg: to_binary(&Cw20HookMsg::Deposit {}).unwrap(),
    });
    let res = execute(deps.as_mut(), mock_env(), info, deposit_msg).unwrap();

    assert_eq!(
        res.messages,
        vec![SubMsg::new(CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: "sayveust0000".to_string(),
            funds: vec![],
            msg: to_binary(&Cw20ExecuteMsg::Send {
                contract: "reward0000".to_string(),
                amount: Uint128::from(100u128),
                msg: to_binary(&SayveCw20HookMsg::Bond {}).unwrap(),
            })
            .unwrap(),
        }))]
    );

    deps.querier
        .with_reward_info(Uint128::from(5u128), Uint128::from(100u128));
    let res = query(deps.as_ref(), mock_env(), QueryMsg::Deposit {}).unwrap();
    let query_res: Uint128 = from_binary(&res).unwrap();
    assert_eq!(query_res, Uint128::from(100u128));

    let res = query(deps.as_ref(), mock_env(), QueryMsg::PendingToken {}).unwrap();
    let query_res: Uint128 = from_binary(&res).unwrap();
    assert_eq!(query_res, Uint128::from(5u128));
}

#[test]
fn test_update_rewards() {
    let mut deps = mock_dependencies(&[]);

    let msg = InstantiateMsg {
        generator_contract_addr: "generator0000".to_string(),
        pair_addr: "pair0000".to_string(),
        lp_token_addr: "sayveust0000".to_string(),
        reward_contract_addr: "reward0000".to_string(),
        reward_token_addr: "sayve0000".to_string(),
    };

    let info = mock_info("addr0000", &[]);
    let _res = instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

    // claim rewards from ANC staking contract
    let err = execute(
        deps.as_mut(),
        mock_env(),
        info.clone(),
        ExecuteMsg::UpdateRewards {},
    )
    .unwrap_err();
    assert_eq!(ContractError::Unauthorized {}, err);

    let info = mock_info("generator0000", &[]);
    let res = execute(
        deps.as_mut(),
        mock_env(),
        info,
        ExecuteMsg::UpdateRewards {},
    )
    .unwrap();

    assert_eq!(
        res.messages,
        vec![SubMsg::new(CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: "reward0000".to_string(),
            funds: vec![],
            msg: to_binary(&SayveExecuteMsg::Withdraw {}).unwrap(),
        }))]
    );

    deps.querier.with_token_balances(&[(
        &"sayve0000".to_string(),
        &[(&MOCK_CONTRACT_ADDR.to_string(), &Uint128::from(5u128))],
    )]);
    deps.querier
        .with_reward_info(Uint128::from(0u128), Uint128::from(100u128));

    // token balance on contract increases from claim
    let res = query(deps.as_ref(), mock_env(), QueryMsg::Reward {}).unwrap();
    let query_res: Uint128 = from_binary(&res).unwrap();
    assert_eq!(query_res, Uint128::from(5u128));

    // no pending tokens
    let res = query(deps.as_ref(), mock_env(), QueryMsg::PendingToken {}).unwrap();
    let query_res: Uint128 = from_binary(&res).unwrap();
    assert_eq!(query_res, Uint128::from(0u128));
}

#[test]
fn test_send_rewards() {
    let mut deps = mock_dependencies(&[]);

    let msg = InstantiateMsg {
        generator_contract_addr: "generator0000".to_string(),
        pair_addr: "pair0000".to_string(),
        lp_token_addr: "sayveust0000".to_string(),
        reward_contract_addr: "reward0000".to_string(),
        reward_token_addr: "sayve0000".to_string(),
    };

    let info = mock_info("addr0000", &[]);
    let _res = instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

    // transfer reward token to user
    // fails when called from unauthorized
    let send_rewards_msg = ExecuteMsg::SendRewards {
        account: "addr0000".to_string(),
        amount: Uint128::new(100),
    };
    let res = execute(deps.as_mut(), mock_env(), info, send_rewards_msg.clone()).unwrap_err();
    match res {
        ContractError::Unauthorized {} => {}
        _ => panic!("Must return unauthorized error"),
    };

    // succeeds when coming from generator
    let generator_info = mock_info("generator0000", &[]);
    let res = execute(deps.as_mut(), mock_env(), generator_info, send_rewards_msg).unwrap();

    assert_eq!(
        res.messages,
        vec![SubMsg::new(CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: "sayve0000".to_string(),
            msg: to_binary(&Cw20ExecuteMsg::Transfer {
                recipient: "addr0000".to_string(),
                amount: Uint128::new(100),
            })
            .unwrap(),
            funds: vec![],
        }))]
    );
}

#[test]
fn test_withdraw() {
    let mut deps = mock_dependencies(&[]);

    deps.querier.with_token_balances(&[(
        &String::from("sayveust0000"),
        &[(&String::from(MOCK_CONTRACT_ADDR), &Uint128::new(0))],
    )]);

    let msg = InstantiateMsg {
        generator_contract_addr: "generator0000".to_string(),
        pair_addr: "pair0000".to_string(),
        lp_token_addr: "sayveust0000".to_string(),
        reward_contract_addr: "reward0000".to_string(),
        reward_token_addr: "sayve0000".to_string(),
    };

    let info = mock_info("addr0000", &[]);
    let _res = instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

    // unbond and send lp tokens to user
    // fails when called from unauthorized
    let withrdaw_msg = ExecuteMsg::Withdraw {
        account: "addr0000".to_string(),
        amount: Uint128::new(100),
    };
    let res = execute(deps.as_mut(), mock_env(), info, withrdaw_msg.clone()).unwrap_err();
    match res {
        ContractError::Unauthorized {} => {}
        _ => panic!("Must return unauthorized error"),
    };

    // succeeds when coming from generator
    let generator_info = mock_info("generator0000", &[]);
    let res = execute(deps.as_mut(), mock_env(), generator_info, withrdaw_msg).unwrap();

    assert_eq!(
        res.messages,
        vec![
            SubMsg::new(WasmMsg::Execute {
                contract_addr: "reward0000".to_string(),
                funds: vec![],
                msg: to_binary(&SayveExecuteMsg::Unbond {
                    amount: Uint128::new(100),
                })
                .unwrap(),
            }),
            SubMsg::new(WasmMsg::Execute {
                contract_addr: "cosmos2contract".to_string(),
                msg: to_binary(&ExecuteMsg::Callback(
                    CallbackMsg::TransferLpTokensAfterWithdraw {
                        account: Addr::unchecked("addr0000"),
                        prev_lp_balance: Uint128::new(0),
                    },
                ))
                .unwrap(),
                funds: vec![]
            })
        ]
    );
}

#[test]
fn test_emergency_withdraw() {
    let mut deps = mock_dependencies(&[]);

    deps.querier.with_token_balances(&[(
        &String::from("sayveust0000"),
        &[(&String::from(MOCK_CONTRACT_ADDR), &Uint128::new(100u128))],
    )]);

    let msg = InstantiateMsg {
        generator_contract_addr: "generator0000".to_string(),
        pair_addr: "pair0000".to_string(),
        lp_token_addr: "sayveust0000".to_string(),
        reward_contract_addr: "reward0000".to_string(),
        reward_token_addr: "sayve0000".to_string(),
    };

    let info = mock_info("addr0000", &[]);
    let _res = instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

    // unbond and send lp tokens to user
    // fails when called from unauthorized
    let withrdaw_msg = ExecuteMsg::EmergencyWithdraw {
        account: "addr0000".to_string(),
        amount: Uint128::new(100),
    };
    let res = execute(deps.as_mut(), mock_env(), info, withrdaw_msg.clone()).unwrap_err();
    match res {
        ContractError::Unauthorized {} => {}
        _ => panic!("Must return unauthorized error"),
    };

    // succeeds when coming from generator
    let generator_info = mock_info("generator0000", &[]);
    let res = execute(deps.as_mut(), mock_env(), generator_info, withrdaw_msg).unwrap();

    assert_eq!(
        res.messages,
        vec![
            SubMsg::new(WasmMsg::Execute {
                contract_addr: "reward0000".to_string(),
                funds: vec![],
                msg: to_binary(&SayveExecuteMsg::Unbond {
                    amount: Uint128::new(100),
                })
                .unwrap(),
            }),
            SubMsg::new(WasmMsg::Execute {
                contract_addr: "cosmos2contract".to_string(),
                msg: to_binary(&ExecuteMsg::Callback(
                    CallbackMsg::TransferLpTokensAfterWithdraw {
                        account: Addr::unchecked("addr0000"),
                        prev_lp_balance: Uint128::new(100),
                    },
                ))
                .unwrap(),
                funds: vec![]
            })
        ]
    );
}

#[test]
fn test_query_reward_info() {
    let mut deps = mock_dependencies(&[]);

    let msg = InstantiateMsg {
        generator_contract_addr: "generator0000".to_string(),
        pair_addr: "pair0000".to_string(),
        lp_token_addr: "sayveust0000".to_string(),
        reward_contract_addr: "reward0000".to_string(),
        reward_token_addr: "sayve0000".to_string(),
    };

    let info = mock_info("addr0000", &[]);
    let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    let res = query(deps.as_ref(), mock_env(), QueryMsg::RewardInfo {}).unwrap();
    let query_res: Addr = from_binary(&res).unwrap();
    assert_eq!(query_res, Addr::unchecked("sayve0000"));
}
