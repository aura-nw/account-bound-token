const chainConfig = require('../config/chain').defaultChain;

const fs = require('fs');

const { SigningCosmWasmClient } = require('@cosmjs/cosmwasm-stargate');
const { DirectSecp256k1HdWallet } = require('@cosmjs/proto-signing');
const { calculateFee, GasPrice } = require('@cosmjs/stargate');

async function deploy() {
  // Deletes ALL existing entries
  if (process.env.DB_RESET || process.env.NODE_ENV === 'test') {
    await knex('standard_contracts').del();
  }
  const wallet = await DirectSecp256k1HdWallet.fromMnemonic(
    chainConfig.mnemonic,
    {
      prefix: chainConfig.prefix
    }
  );
  const client = await SigningCosmWasmClient.connectWithSigner(chainConfig.rpcEndpoint, wallet);
  const gasPrice = GasPrice.fromString(`0.025${chainConfig.denom}`);
  const uploadFee = calculateFee(2000000, gasPrice);
  const account = (await wallet.getAccounts())[0];

  const aura4973 = fs.readFileSync(`${__dirname}/../target/wasm32-unknown-unknown/release/aura_4973.wasm`);
  const aura4973Response = await client.upload(account.address, aura4973, uploadFee, 'Upload aura4973 contract code');
  console.log(aura4973Response);
}

deploy()