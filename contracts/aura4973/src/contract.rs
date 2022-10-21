#[cfg(not(feature = "library"))]
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, StdResult, Binary, to_binary, Deps, Order, entry_point};
use schemars::JsonSchema;
use sha2::{Digest, Sha256};
use cw2::set_contract_version;
use std::str;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, QueryMsg, InstantiateMsg};
use crate::state::{Aura4973, ContractInfoResponse, NameResponse, SymbolResponse, NumNftsResponse, NftInfo, OwnerOfResponse, ADR36SignDoc, Fee, MsgSignData, MsgSignDataValue};

use bech32::{ToBase32, Variant::Bech32};
use ripemd::{Ripemd160};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:aura-4973";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

const AGREEMENT_STRING: &str = "Agreement(address active,address passive,string tokenURI)";

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    // get default Aura4973 contract
    let contract = Aura4973::default();

    // call the instantiate function
    let response = contract
        .instantiate(deps, msg)
        .unwrap();
        
    // return the response
    Ok(response)

}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    // get default Aura4973 contract
    let contract = Aura4973::default();

    // call and return the execute function
    contract.execute(deps, env, info, msg)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(
    deps: Deps,
    _env: Env,
    msg: QueryMsg,
) -> StdResult<Binary> {
    // get default Aura4973 contract
    let contract = Aura4973::default();

    // call the query function
    let response = contract
        .query(deps, _env, msg)
        .unwrap();

    // return the response
    Ok(to_binary(&response)?)
}

impl<'a> Aura4973<'a>{
    pub fn instantiate(
        &self,
        deps: DepsMut,
        msg: InstantiateMsg,
    ) -> StdResult<Response> {
        set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

        let info = ContractInfoResponse {
            name: msg.name,
            symbol: msg.symbol,
        };
        
        let minter = deps.api.addr_validate(&msg.minter)?;

        self.contract_info.save(deps.storage, &info)?;
        self.minter.save(deps.storage, &minter)?;
        self.nft_count.save(deps.storage, &0u64)?;

        // set the contract version
        set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

        Ok(Response::default())
    }

    pub fn execute(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg,
    ) -> Result<Response, ContractError> {
        match msg {
            ExecuteMsg::Give { to, uri, signature } => self.execute_give(deps, info, to, uri, signature),
            ExecuteMsg::Take { from, uri, signature } => self.execute_take(deps, info, from, uri, signature),
            ExecuteMsg::UnEquip { nft_id } => self.execute_unequip(deps, env, info, nft_id),
            ExecuteMsg::Equip { nft_id } => self.execute_equip(deps, env, info, nft_id),
            // ExecuteMsg::Mint { nft_id, owner, nft_uri } => self.execute_mint(deps, env, info, nft_id, owner, nft_uri),
            ExecuteMsg::UnAdmit { nft_id } => self.execute_unadmit(deps, env, info, nft_id),
        }
    }

    pub fn query(
        &self,
        deps: Deps,
        _env: Env,
        msg: QueryMsg,
    ) -> StdResult<Binary> {
        match msg {
            QueryMsg::ContractInfo {} => to_binary(&self.contract_info(deps)?),
            QueryMsg::Name {} => to_binary(&self.name(deps)?),
            QueryMsg::Symbol {} => to_binary(&self.symbol(deps)?),
            QueryMsg::NftInfo { nft_id } => to_binary(&self.nft_info(deps, nft_id)?),
            QueryMsg::OwnerOf { nft_id } => to_binary(&self.owner_of(deps, nft_id)?),
            QueryMsg::NumNfts {} => to_binary(&self.num_nfts(deps)?),
            QueryMsg::AllUnequippedNftOf { owner} => to_binary(&self.all_unequipped_nft_of(deps, owner)?),
            QueryMsg::AllEquippedNftOf { owner } => to_binary(&self.all_equipped_nft_of(deps, owner)?),
        }
    }

    // The execution functions for contract

    // execute_give function is used to give a nft to another address
    // only the minter can call this function
    pub fn execute_give(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        to: String,
        uri: String,
        signature: String,
    ) -> Result<Response, ContractError> {
        // Cannot give NFT from a user who is not the minter
        let minter = self.minter.load(deps.storage)?;
        if info.sender != minter {
            return Err(ContractError::Unauthorized {});
        }

        // get the nft id using _safeCheckAgreement function
        let nft_id = self._safe_check_agreement(&deps, &minter.into_string(), &to, &uri, &signature);
        
        // check if the nft id is empty, then return error
        if nft_id.is_empty() {
            return Err(ContractError::To {});
        }

        // mint the nft to the address 'to' and return the response of mint function
        self._mint(deps, nft_id, &to.to_string(), uri)
    }

    // execute_take function is used to take a nft from another address
    // the user can only take nft from minter
    pub fn execute_take(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        from: String,
        uri: String,
        signature: String,
    ) -> Result<Response, ContractError> {
        // check 'from' is a valid human's address
        let from_addr = deps.api.addr_validate(&from)?;

        // Cannot take NFT from a user who is not the minter
        let minter = self.minter.load(deps.storage)?;
        if from_addr != minter {
            return Err(ContractError::Unauthorized {});
        }

        // get address of the owner of the nft from info.sender
        let owner = info.sender.clone();

        // get the nft id using _safeCheckAgreement function
        let nft_id = self._safe_check_agreement(&deps, &owner.to_string(), &from, &uri, &signature);
        
        // check if the nft id is empty, then return error
        if nft_id.is_empty() {
            return Err(ContractError::From {});
        }

        // mint the nft to the owner and return the response of function _mint
        self._mint(deps, nft_id, &owner.to_string(), uri)

    }

    // get_bech32_address function is used to get the bech32 address from the public key and hrp
    // hrp is the human readable part of the bech32 address
    // hrp in Aura chain is "aura"
    fn _get_bech32_address(&self, hrp: &str, pubkey: Vec<u8>) -> Result<String, ContractError> {
        // get the hash of the pubkey bytes
        let pk_hash = Sha256::digest(&pubkey);

        // Insert the hash result in the ripdemd hash function
        let mut rip_hasher = Ripemd160::default();
        rip_hasher.update(pk_hash);
        let rip_result = rip_hasher.finalize();

        let address_bytes = rip_result.to_vec();

        let bech32_address = bech32::encode(hrp, address_bytes.to_base32(), Bech32)
            .map_err(|err| ContractError::Hrp(err.to_string()))?;

        Ok(bech32_address)
    }

    // _safeCheckAgreement function is used to check if the agreement is valid or not
    // if the agreement is valid then it returns the id of the nft
    fn _safe_check_agreement(
        &self,
        deps: &DepsMut,
        active: &String,
        passive: &String,
        uri: &String,
        signature: &String,
    ) -> String {
        // get hash for the agreement
        let hash = Aura4973::<'a>::_get_hash(&active, &passive, &uri);
        
        // recover the public key from the signature and the hash
        let pubkey = deps.api.secp256k1_recover_pubkey(&hash, signature.as_bytes(), 1).unwrap();
        
        // get the address of signer from the public key using get_bech32_address function
        let signer_address = self._get_bech32_address("aura", pubkey).unwrap();

        // check if the recovered address is same as the 'to' address, then return empty string
        if signer_address != *passive {
            return "".to_string();
        } else {
            // the id of the nft is the hash of hash value using sha256
            let nft_id = Sha256::digest(&hash);

            // return the nft id as string
            return str::from_utf8(nft_id.as_slice()).unwrap().to_string();
        }
    }

    // the get_hash funtion will concat the address of the sender, the address of the 'to', the uri of the nft and the hash of the string
    fn _get_hash(
        active: &String,
        passive: &String,
        uri: &String,
    ) -> Vec<u8> {
        // hash the constant string and data
        let big_string = format!("{}{}{}{}", AGREEMENT_STRING, active, passive, uri);
        let hash = Sha256::digest(big_string.as_bytes());
        
        // return the hash
        return hash.as_slice().to_vec();
    }

    // _mint is a function that allows the minter mints a nft with id and nft_uri to owner
    pub fn _mint(
        &self,
        deps: DepsMut,
        nft_id: String,
        owner: &String,
        nft_uri: String,
    ) -> Result<Response, ContractError> {
        let owner_addr = deps.api.addr_validate(&owner)?;
        let nft_info = NftInfo {
            id: nft_id.clone(),
            owner: owner_addr.to_string(),
            nft_uri,
            equiped: true,
            is_admitted: false,
        };

        // add the newly created token to tokens in storage
        self.nfts.save(deps.storage, &nft_id, &nft_info)?;

        // increase the number of nfts in storage using incresment function
        self.increment_nfts(deps.storage)?;
        
        Ok(Response::new()
            .add_attribute("action", "mint")
            .add_attribute("nft_id", nft_id)
            .add_attribute("owner", owner))
    }

    // execute_unequip is a function that allows the owner of a nft to unequip it by set the equiped field to false
    pub fn execute_unequip(
        &self,
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        nft_id: String,
    ) -> Result<Response, ContractError> {
        // get information of nft by id
        let mut nft_info = self.nfts.load(deps.storage, &nft_id)?;

        // check if the owner of the nft is the sender
        if nft_info.owner != info.sender.to_string() {
            return Err(ContractError::Unauthorized {});
        }

        // check if the nft is already unequipped
        if !nft_info.equiped {
            return Err(ContractError::NftAlreadyUnequipped {});
        }

        // set the equiped field to false
        nft_info.equiped = false;

        // save the nft info
        self.nfts.save(deps.storage, &nft_id, &nft_info)?;

        // return response
        Ok(Response::new()
            .add_attribute("action", "unequip")
            .add_attribute("nft_id", nft_id)
            .add_attribute("owner", info.sender))
    }

    // execute_unequip is a function that allows the owner of a nft to equip it by set the equiped field to true
    pub fn execute_equip(
        &self,
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        nft_id: String,
    ) -> Result<Response, ContractError> {
        // get information of nft by id
        let mut nft_info = self.nfts.load(deps.storage, &nft_id)?;

        // check if the owner of the nft is the sender
        if nft_info.owner != info.sender.to_string() {
            return Err(ContractError::Unauthorized {});
        }

        // check if the nft is already equipped
        if nft_info.equiped {
            return Err(ContractError::NftAlreadyEquipped {});
        }

        // set the equiped field to true
        nft_info.equiped = true;

        // save the nft info
        self.nfts.save(deps.storage, &nft_id, &nft_info)?;

        // return response
        Ok(Response::new()
            .add_attribute("action", "equip")
            .add_attribute("nft_id", nft_id)
            .add_attribute("owner", info.sender))
    }

    // execute_unadmit is a function that allows the minter add a nft id to the unadmitted list by set the admitted field to true
    pub fn execute_unadmit(
        &self,
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        nft_id: String,
    ) -> Result<Response, ContractError> {
        // check if the minter is the sender
        let minter = self.minter.load(deps.storage)?;
        if info.sender != minter {
            return Err(ContractError::NotMinter {});
        }

        // get information of nft by id
        let nft_info = self.nfts.load(deps.storage, &nft_id)?;

        // check if the nft is already unadmitted
        if nft_info.is_admitted {
            return Err(ContractError::NftAlreadyUnadmitted {});
        }
        else {
            // set the admitted field to true
            let mut nft_info = self.nfts.load(deps.storage, &nft_id)?;
            nft_info.is_admitted = true;

            // save the nft info
            self.nfts.save(deps.storage, &nft_id, &nft_info)?
        }

        // return response
        Ok(Response::new()
            .add_attribute("action", "unadmit")
            .add_attribute("nft_id", nft_id)
            .add_attribute("minter", info.sender))
    }

    // contract_info returns the contract info
    fn contract_info(&self, deps: Deps) -> StdResult<ContractInfoResponse> {
        self.contract_info.load(deps.storage)
    }

    // num_nfts returns the number of distributed nfts in the contract
    fn num_nfts(&self, deps: Deps) -> StdResult<NumNftsResponse> {
        let count = self.nft_count.load(deps.storage)?;
        Ok(NumNftsResponse { count })
    }

    // owner_of returns the owner of the token with the given id
    pub fn owner_of(
        &self,
        deps: Deps,
        nft_id: String,
    ) -> StdResult<OwnerOfResponse> {
        let info = self.nfts.load(deps.storage, &nft_id)?;
        Ok(OwnerOfResponse {
            owner: info.owner,
        })
    }

    // nft_info returns the information of the nft with the given id
    pub fn nft_info(
        &self,
        deps: Deps,
        nft_id: String,
    ) -> StdResult<NftInfo> {
        let info = self.nfts.load(deps.storage, &nft_id)?;
        Ok(NftInfo {
            id: info.id,
            owner: info.owner,
            nft_uri: info.nft_uri,
            equiped: info.equiped,
            is_admitted: info.is_admitted,
        })
    }

    // name returns the name of the contract
    fn name(&self, deps: Deps) -> StdResult<NameResponse> {
        let info = self.contract_info(deps)?;
        Ok(NameResponse { name: info.name })
    }

    // symbol returns the symbol of the contract
    fn symbol(&self, deps: Deps) -> StdResult<SymbolResponse> {
        let info = self.contract_info(deps)?;
        Ok(SymbolResponse { symbol: info.symbol })
    }

    // all_unequipped_nft_of is a function that returns all the nfts of a given owner that are unequipped
    pub fn all_unequipped_nft_of(
        &self,
        deps: Deps,
        owner: String,
    ) -> StdResult<Vec<NftInfo>> {
        let mut unequipped_nfts: Vec<NftInfo> = vec![];
        // load all nfts from storage prefixed by owner index 
        self.nfts.idx.owner
            .prefix(owner)
            .range(deps.storage, None, None, Order::Ascending)
            .for_each(|item| {
                let (_, nft_info) = item.unwrap();
                if !nft_info.equiped && !nft_info.is_admitted {
                    unequipped_nfts.push(nft_info);
                }
            });
        Ok(unequipped_nfts)
    }

    // all_equipped_nft_of is a function that returns all the nfts of a given owner that are equipped
    pub fn all_equipped_nft_of(
        &self,
        deps: Deps,
        owner: String,
    ) -> StdResult<Vec<NftInfo>> {
        let mut equipped_nfts: Vec<NftInfo> = vec![];
        // load all nfts from storage prefixed by owner index 
        self.nfts.idx.owner
            .prefix(owner)
            .range(deps.storage, None, None, Order::Ascending)
            .for_each(|item| {
                let (_, nft_info) = item.unwrap();
                if nft_info.equiped && !nft_info.is_admitted {
                    equipped_nfts.push(nft_info);
                }
            });
        Ok(equipped_nfts)
    }

}


// create signable structure from message and chain ID
pub fn get_sign_doc(
    signer: String,
    data: String,
    chain_id: String
) -> ADR36SignDoc {
    // create signable structure
    let doc = ADR36SignDoc {
        chain_id: chain_id,
        account_number: "0".to_string(),
        sequence: "0".to_string(),
        fee: Fee {
            gas: "0".to_string(),
            amount: [].to_vec(),
        },
        msgs: [MsgSignData {
            r#type: "sign/MsgSignData".to_string(),
            value: MsgSignDataValue {
                signer: signer,
                data: data.into_bytes(),
            },
        }].to_vec(),
        memo: "".to_string()
    };

    doc
}
// testing function for get_signDoc
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_sign_doc() {
        // get signDoc
        let signer = "signer".to_string();
        let data = "data".to_string();
        let chain_id = "chainID".to_string();
        let sign_doc = get_sign_doc(signer, data, chain_id);
        println!("{}", serde_json::to_string(&sign_doc).unwrap());
        
        // throw to check manually
        assert_eq!(1, 0);
    }
}

