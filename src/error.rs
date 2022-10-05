use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Caller is not minter")]
    NotMinter {},

    #[error("NFT is already unequipped")]
    NftAlreadyUnequipped {},

    #[error("NFT is already equipped")]
    NftAlreadyEquipped {},

    #[error("NFT is already unadmitted")]
    NftAlreadyUnadmitted {},
}
