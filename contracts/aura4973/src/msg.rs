use cosmwasm_schema::{cw_serde};

/// Message type for `instantiate` entry_point
#[cw_serde]
pub struct InstantiateMsg {
    /// Name of the NFT contract
    pub name: String,
    /// Symbol of the NFT contract
    pub symbol: String,

    /// The minter is the only one who can create new NFTs.
    /// This is designed for a base NFT that is controlled by an external program
    /// or contract. You will likely replace this with custom logic in custom NFTs
    pub minter: String,
}

/// Message type for `execute` entry_point
#[cw_serde]
pub enum ExecuteMsg {
    /// @notice Mint a token to user
    Mint{
        /// Unique ID of the NFT
        nft_id: String,
        /// The owner of the newly minter NFT
        owner: String,
        /// Universal resource identifier for this NFT
        /// Should point to a JSON file that conforms to the ERC721
        /// Metadata JSON Schema
        nft_uri: String,
    },

    /// @notice Removes the `String token_id` from an account.
    /// @dev Must emit a `event Transfer` with the `address to` field pointing to
    ///  the zero address.
    /// @param token_id The identifier for an ABT.
    UnEquip{ nft_id: String },

    /// @notice add the `String token_id` to account.
    /// @dev Must emit a `event Transfer` with the `address from` field pointing from
    ///  the zero address.
    /// @param token_id The identifier for an ABT.
    Equip{ nft_id: String },

    /// @notice un admit the `String token_id` from account.
    UnAdmit{ nft_id: String },

}

/// Message type for `migrate` entry_point
#[cw_serde]
pub enum MigrateMsg {}

/// Message type for `query` entry_point
#[cw_serde]
pub enum QueryMsg {
    /// With MetaData Extension.
    /// Returns top-level metadata about the contract: `ContractInfoResponse`
    ContractInfo {},

    /// Total number of nfts issued
    NumNfts {},

    /// Return the owner of the given nft, error if nft does not exist
    /// Return type: OwnerOfResponse
    OwnerOf {
        nft_id: String,
    },

    /// With MetaData Extension.
    /// Returns metadata about one particular nft, based on *ERC721 Metadata JSON Schema*
    /// but directly from the contract: `NftInfoResponse`
    NftInfo {
        nft_id: String,
    },

    /// With MetaData Extension.
    /// Returns the list of all nft of a owner with the equipment status is true.
    /// Return type: `AllNftOfResponse`
    AllEquippedNftOf {
        owner: String,
    },

    /// With MetaData Extension.
    /// Returns the list of all nft of a owner with the equipment status is false.
    /// Return type: `AllNftOfResponse`
    AllUnequippedNftOf {
        owner: String,
    },
}
