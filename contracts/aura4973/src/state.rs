use cosmwasm_schema::cw_serde;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, StdResult, Storage};

use cw_storage_plus::{Index, IndexList, IndexedMap, Item, MultiIndex};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct NftInfo {
    pub id: String,
    pub owner: String,
    pub nft_uri: String,
    pub equiped: bool,
    pub is_admitted: bool,
}

pub struct NftIndexes<'a>{
    // String (owner's address) -> String (token_id)
    // String (token_id) -> TokenInfo (token_info)
    pub owner: MultiIndex<'a, String, NftInfo, String>,
}

pub struct Aura4973 <'a>{
    pub contract_info: Item<'a, ContractInfoResponse>,
    pub minter: Item<'a, Addr>,
    pub nft_count: Item<'a, u64>,
    pub nfts: IndexedMap<'a, &'a str, NftInfo, NftIndexes<'a>>,
}

impl Default for Aura4973<'static>{
    fn default() -> Self {
        Self::new(
            "nft_info",
            "minter",
            "num_tokens",
            "tokens",
            "tokens__owner",
        )
    }
}

impl<'a> Aura4973<'a>{
    fn new(
        contract_key: &'a str,
        minter_key: &'a str,
        token_count_key: &'a str,
        tokens_key: &'a str,
        tokens_owner_key: &'a str,
    ) -> Self {
        let indexes = NftIndexes {
            owner: MultiIndex::new(token_owner_idx, tokens_key, tokens_owner_key),
        };
        Self {
            contract_info: Item::new(contract_key),
            minter: Item::new(minter_key),
            nft_count: Item::new(token_count_key),
            nfts: IndexedMap::new(tokens_key, indexes),
        }
    }

    pub fn nft_count(&self, storage: &dyn Storage) -> StdResult<u64> {
        Ok(self.nft_count.may_load(storage)?.unwrap_or_default())
    }

    pub fn increment_nfts(&self, storage: &mut dyn Storage) -> StdResult<u64> {
        let val = self.nft_count(storage)? + 1;
        self.nft_count.save(storage, &val)?;
        Ok(val)
    }

}
    

impl<'a> IndexList<NftInfo> for NftIndexes<'a>{
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<NftInfo>> + '_> {
        let v: Vec<&dyn Index<NftInfo>> = vec![&self.owner];
        Box::new(v.into_iter())
    }
}

#[cw_serde]
pub struct NumNftsResponse {
    pub count: u64,
}

#[cw_serde]
pub struct ContractInfoResponse {
    pub name: String,
    pub symbol: String,
}

#[cw_serde]
pub struct NameResponse {
    pub name: String,
}

#[cw_serde]
pub struct SymbolResponse {
    pub symbol: String,
}

#[cw_serde]
pub struct OwnerOfResponse {
    /// Owner of the nft
    pub owner: String,
}

pub fn token_owner_idx(_pk: &[u8], d: &NftInfo) -> String {
    d.owner.clone()
}
