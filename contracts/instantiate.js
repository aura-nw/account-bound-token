const chainConfig = require('../config/chain').defaultChain;

const fs = require('fs');

const { SigningCosmWasmClient } = require('@cosmjs/cosmwasm-stargate');
const { DirectSecp256k1HdWallet } = require('@cosmjs/proto-signing');
// const { calculateFee, GasPrice } = require('@cosmjs/stargate');

async function instantiate(_codeID) {
    const wallet = await DirectSecp256k1HdWallet.fromMnemonic(
        chainConfig.mnemonic,
        {
            prefix: chainConfig.prefix
        }
    );
    const client = await SigningCosmWasmClient.connectWithSigner(chainConfig.rpcEndpoint, wallet);

    const account = (await wallet.getAccounts())[0];

    const defaultFee = { amount: [{amount: "200000", denom: chainConfig.denom,},], gas: "200000",};

    const codeId = _codeID;
    //Define the instantiate message
    const instantiateMsg = {"name":"AURA ACCOUNT BOUND",
                            "symbol":"AAB",
                            "minter":"aura1uh24g2lc8hvvkaaf7awz25lrh5fptthu2dhq0n",};


    //Instantiate the contract
    const instantiateResponse = await client.instantiate(account.address, Number(_codeID), instantiateMsg, "Instantiate contract", defaultFee);
    console.log(instantiateResponse);

    // print out the address of the newly created contract
    const contracts = await client.getContracts(_codeID);
    console.log(contracts);
}

const myArgs = process.argv.slice(2);
instantiate(myArgs[0]);
