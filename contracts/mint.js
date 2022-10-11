const chainConfig = require('../config/chain').defaultChain;

const fs = require('fs');

const { SigningCosmWasmClient } = require('@cosmjs/cosmwasm-stargate');
const { DirectSecp256k1HdWallet } = require('@cosmjs/proto-signing');
// const { calculateFee, GasPrice } = require('@cosmjs/stargate');

async function mint(_contract, _nftID, _recipient, _uri) {
    const wallet = await DirectSecp256k1HdWallet.fromMnemonic(
        chainConfig.mnemonic,
        {
            prefix: chainConfig.prefix
        }
    );
    const client = await SigningCosmWasmClient.connectWithSigner(chainConfig.rpcEndpoint, wallet);

    const account = (await wallet.getAccounts())[0];

    const defaultFee = { amount: [{amount: "200000", denom: chainConfig.denom,},], gas: "200000",};
    const memo = "minting nft";
    //Define the instantiate message
    const ExecuteMintMsg = {"mint":{"nft_id":_nftID,
                            "owner":_recipient,
                            "nft_uri":_uri,}};

    //mint a NFT to minter
    const mintResponse = await client.execute(account.address, _contract, ExecuteMintMsg, defaultFee, memo);
    console.log(mintResponse);
}

const myArgs = process.argv.slice(2);
console.log('myArgs[0]: ', myArgs[0]);
console.log('myArgs[1]: ', myArgs[1]);
console.log('myArgs[2]: ', myArgs[2]);
console.log('myArgs[3]: ', myArgs[3]);
mint(myArgs[0], myArgs[1], myArgs[2], myArgs[3]);
