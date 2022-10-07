#![cfg(test)]
// import all requirement to prepare the test environment
use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use cosmwasm_std::{from_binary, DepsMut};

use crate::state::{Aura4973, NftInfo};

const MINTER: &str = "minter";

// function to setup contract for testing
fn setup_contract(deps: DepsMut) -> Aura4973 {
    // get default Aura4973 contract
    let contract = Aura4973::default();

    // prepare the contract info
    let contract_info = crate::msg::InstantiateMsg {
        name: "Aura 4973".to_string(),
        symbol: "A4973".to_string(),
        minter: String::from(MINTER),
    };

    // prepare the message info
    let msg_info = mock_info("creator", &[]);

    // prepare the env
    let env = mock_env();

    // call the instantiate function
    let response = contract
        .instantiate(deps, env, msg_info, contract_info)
        .unwrap();

    // check the response
    assert_eq!(0, response.messages.len());

    contract

}

// function for testing mint nft to owner
#[test]
fn test_mint_nft() {
    // prepare the mock dependencies
    let mut deps = mock_dependencies();
    
    // setup the contract
    setup_contract(deps.as_mut());

    // get contract by using default Aura4973 contract
    let contract = Aura4973::default();

    // prepare the env
    let env = mock_env();

    // prepare the minting message
    let mint_msg = crate::msg::ExecuteMsg::Mint {
        nft_id: "nft_id".to_string(),
        owner: "owner".to_string(),
        nft_uri: "nft_uri".to_string(),
    };

    // a random user cannot mint
    let random = mock_info("random", &[]);

    // call the execute function
    let err = contract
        .execute(deps.as_mut(), env.clone(), random, mint_msg.clone())
        .unwrap_err();
    
    // check the error
    assert_eq!(err, crate::error::ContractError::NotMinter {});

    // the minter can mint
    let minter = mock_info(MINTER, &[]);
    let _res = contract
        .execute(deps.as_mut(), env.clone(), minter, mint_msg.clone())
        .unwrap();

    // prepare the query response to get the list of all nft of a owner with the equipment status is true
    let query_msg_get_eqquipped_nfts = crate::msg::QueryMsg::AllEquippedNftOf {
        owner: "owner".to_string(),
    };

    // get the query response
    let query_res = contract.query(deps.as_ref(), env, query_msg_get_eqquipped_nfts).unwrap();

    // convert the query response to the NftInfo type
    let nft_info: Vec<NftInfo> = from_binary(&query_res).unwrap();

    // check the nft info
    assert_eq!(nft_info[0].id, "nft_id");
    assert_eq!(nft_info[0].nft_uri, "nft_uri".to_string());
    assert_eq!(nft_info[0].owner, "owner".to_string());
    assert_eq!(nft_info[0].equiped, true);

}

// function to unequip a nft for testing
#[test]
fn execute_unequip() {
    // prepare the mock dependencies
    let mut deps = mock_dependencies();
    
    // setup the contract
    setup_contract(deps.as_mut());

    // get contract by using default Aura4973 contract
    let contract = Aura4973::default();

    // prepare the env
    let env = mock_env();

    // prepare the minting message
    let mint_msg = crate::msg::ExecuteMsg::Mint {
        nft_id: "nft_id".to_string(),
        owner: "owner".to_string(),
        nft_uri: "nft_uri".to_string(),
    };

    // minter mint a nft to owner
    let minter = mock_info(MINTER, &[]);
    let _res = contract
        .execute(deps.as_mut(), env.clone(), minter, mint_msg.clone())
        .unwrap();

    // prepare the message
    let unequipped_msg = crate::msg::ExecuteMsg::UnEquip {
        nft_id: "nft_id".to_string(),
    };

    // owner unequip the nft
    let owner = mock_info("owner", &[]);
    let _res = contract
        .execute(deps.as_mut(), env.clone(), owner, unequipped_msg.clone())
        .unwrap();
    
    // prepare the query response to get the list of all nft of a owner with the equipment status is true
    let query_msg_get_eqquipped_nfts = crate::msg::QueryMsg::AllEquippedNftOf {
        owner: "owner".to_string(),
    };

    // prepare the env
    let env1 = mock_env();

    // get the query response
    let query_res = contract.query(deps.as_ref(), env1, query_msg_get_eqquipped_nfts).unwrap();

    // convert the query response to the NftInfo type
    let nft_info: Vec<NftInfo> = from_binary(&query_res).unwrap();

    // check the number of nft
    assert_eq!(nft_info.len(), 0);

    // prepare the query response to get the list of all nft of a owner with the equipment status is false
    let query_msg_get_uneqquipped_nfts = crate::msg::QueryMsg::AllUnequippedNftOf {
        owner: "owner".to_string(),
    };

    // prepare the env
    let env2 = mock_env();

    // get the query response
    let query_res_unequipped = contract.query(deps.as_ref(), env2, query_msg_get_uneqquipped_nfts).unwrap();

    // convert the query response to the NftInfo type
    let nft_info: Vec<NftInfo> = from_binary(&query_res_unequipped).unwrap();

    // check the nft info
    assert_eq!(nft_info[0].id, "nft_id");
    assert_eq!(nft_info[0].nft_uri, "nft_uri".to_string());
    assert_eq!(nft_info[0].owner, "owner".to_string());
    assert_eq!(nft_info[0].equiped, false);

}

// function to re-equip a nft for testing
#[test]
fn execute_re_equip() {
    // prepare the mock dependencies
    let mut deps = mock_dependencies();
    
    // setup the contract
    setup_contract(deps.as_mut());

    // get contract by using default Aura4973 contract
    let contract = Aura4973::default();

    // prepare the env
    let env = mock_env();

    // prepare the minting message
    let mint_msg = crate::msg::ExecuteMsg::Mint {
        nft_id: "nft_id_re-equip".to_string(),
        owner: "owner".to_string(),
        nft_uri: "nft_uri".to_string(),
    };

    // minter mint a nft to owner
    let minter = mock_info(MINTER, &[]);
    let _res = contract
        .execute(deps.as_mut(), env.clone(), minter, mint_msg.clone())
        .unwrap();

    // prepare the message
    let unequipped_msg = crate::msg::ExecuteMsg::UnEquip {
        nft_id: "nft_id_re-equip".to_string(),
    };

    // owner unequip the nft
    let owner = mock_info("owner", &[]);
    let _res = contract
        .execute(deps.as_mut(), env.clone(), owner, unequipped_msg.clone())
        .unwrap();
    
    // prepare the query response to get the list of all nft of a owner with the equipment status is true
    let query_msg_get_eqquipped_nfts = crate::msg::QueryMsg::AllEquippedNftOf {
        owner: "owner".to_string(),
    };

    // prepare the env
    let env1 = mock_env();

    // get the query response
    let query_res = contract.query(deps.as_ref(), env1, query_msg_get_eqquipped_nfts).unwrap();

    // convert the query response to the NftInfo type
    let nft_info: Vec<NftInfo> = from_binary(&query_res).unwrap();

    // check the number of nft
    assert_eq!(nft_info.len(), 0);

    // prepare the query response to get the list of all nft of a owner with the equipment status is false
    let query_msg_get_uneqquipped_nfts = crate::msg::QueryMsg::AllUnequippedNftOf {
        owner: "owner".to_string(),
    };

    // prepare the env
    let env2 = mock_env();

    // get the query response
    let query_res_unequipped = contract.query(deps.as_ref(), env2, query_msg_get_uneqquipped_nfts).unwrap();

    // convert the query response to the NftInfo type
    let nft_info: Vec<NftInfo> = from_binary(&query_res_unequipped).unwrap();

    // check the nft info
    assert_eq!(nft_info[0].id, "nft_id_re-equip");
    assert_eq!(nft_info[0].nft_uri, "nft_uri".to_string());
    assert_eq!(nft_info[0].owner, "owner".to_string());
    assert_eq!(nft_info[0].equiped, false);

    // get id of unequipped nft
    let unequipped_nft_id = &nft_info[0].id;

    // prepare the message
    let re_equipped_msg = crate::msg::ExecuteMsg::Equip {
        nft_id: unequipped_nft_id.clone(),
    };

    // owner re-equip the nft
    let owner = mock_info("owner", &[]);
    let _res = contract
        .execute(deps.as_mut(), env.clone(), owner, re_equipped_msg.clone())
        .unwrap();

    // prepare the query response to get the list of all nft of a owner with the equipment status is true
    let query_msg_get_eqquipped_nfts = crate::msg::QueryMsg::AllEquippedNftOf {
        owner: "owner".to_string(),
    };

    // prepare the env
    let env1 = mock_env();

    // get the query response
    let query_res = contract.query(deps.as_ref(), env1, query_msg_get_eqquipped_nfts).unwrap();

    // convert the query response to the NftInfo type
    let nft_info: Vec<NftInfo> = from_binary(&query_res).unwrap();

    // check the number of nft
    assert_eq!(nft_info.len(), 1);

    // check the nft info
    assert_eq!(nft_info[0].id, "nft_id_re-equip");
    assert_eq!(nft_info[0].nft_uri, "nft_uri".to_string());
    assert_eq!(nft_info[0].owner, "owner".to_string());
    assert_eq!(nft_info[0].equiped, true);

}

// function to test minter can unadmit a nft
#[test]
fn execute_unadmit() {
    // prepare the mock dependencies
    let mut deps = mock_dependencies();
    
    // setup the contract
    setup_contract(deps.as_mut());

    // get contract by using default Aura4973 contract
    let contract = Aura4973::default();

    // prepare the env
    let env = mock_env();

    // prepare the minting message
    let mint_msg = crate::msg::ExecuteMsg::Mint {
        nft_id: "nft_id_unadmit".to_string(),
        owner: "owner".to_string(),
        nft_uri: "nft_uri".to_string(),
    };

    // minter mint a nft to owner
    let minter = mock_info(MINTER, &[]);
    let _res = contract
        .execute(deps.as_mut(), env.clone(), minter, mint_msg.clone())
        .unwrap();
    
    // check number of equipped nft of owner
    let query_msg_get_eqquipped_nfts = crate::msg::QueryMsg::AllEquippedNftOf {
        owner: "owner".to_string(),
    };

    // prepare the env
    let env1 = mock_env();

    // get the query response
    let query_res = contract.query(deps.as_ref(), env1, query_msg_get_eqquipped_nfts).unwrap();

    // convert the query response to the NftInfo type
    let nft_info: Vec<NftInfo> = from_binary(&query_res).unwrap();

    // check the number of nft
    assert_eq!(nft_info.len(), 1);

    // prepare the message
    let unadmit_msg = crate::msg::ExecuteMsg::UnAdmit {
        nft_id: "nft_id_unadmit".to_string(),
    };

    // minter unadmit the nft
    let minter = mock_info(MINTER, &[]);
    let _res = contract
        .execute(deps.as_mut(), env.clone(), minter, unadmit_msg.clone())
        .unwrap();

    // check number of equipped nft of owner
    let query_msg_get_eqquipped_nfts = crate::msg::QueryMsg::AllEquippedNftOf {
        owner: "owner".to_string(),
    };

    // prepare the env
    let env1 = mock_env();

    // get the query response
    let query_res = contract.query(deps.as_ref(), env1, query_msg_get_eqquipped_nfts).unwrap();

    // convert the query response to the NftInfo type
    let nft_info: Vec<NftInfo> = from_binary(&query_res).unwrap();

    // check the number of nft
    assert_eq!(nft_info.len(), 0);

}