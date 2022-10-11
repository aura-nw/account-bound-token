# account-bound-token
soul-bound nft contract

## Store contract
Run following command to store contract
```bash
node ./contracts/store.js
```

The result of command should be liked this
```bash
{
  originalSize: 216510,
  originalChecksum: 'b97b8705c2b659a64dc824d8d957cf3aad1e5b802c7af159232ff64ccba6a9f0',
  compressedSize: 68746,
  compressedChecksum: '137f40e89d692de145cbc2998ed5f5913badd749071e62239989642294b1a0c9',
  codeId: 504,
  logs: [ { msg_index: 0, log: '', events: [Array] } ],
  height: 5570268,
  transactionHash: 'DAC0D5800560CCA038659B545B2E836929DD3D867E1D07DE915AAB37DEC01E01',
  gasWanted: 2000000,
  gasUsed: 1404308
}
```

You must store the codeID of contract
```bash
    codeId: 504,
```

## Instantiate contract
Run following command to instantiate contract
```bash
node ./contracts/instantiate.js <codeId>
```

The result of command should be
```bash
{
  contractAddress: 'aura1au6pmwct2vwk55uz88clkkeckqr3xpcw5w4kc54lt0x9mx900khqjhyhzf',
  logs: [ { msg_index: 0, log: '', events: [Array] } ],
  height: 5570284,
  transactionHash: '79822277F8B70D0ECDEF6976C0084B816692765CFCFF6C7C54125C8857DE8A5F',
  gasWanted: 200000,
  gasUsed: 172784
}
[ 'aura1au6pmwct2vwk55uz88clkkeckqr3xpcw5w4kc54lt0x9mx900khqjhyhzf' ]
```

You must store the address of contract
```bash
    contractAddress: 'aura1au6pmwct2vwk55uz88clkkeckqr3xpcw5w4kc54lt0x9mx900khqjhyhzf',
```

## Mint a NFT
If you want to mint a NFT with `_nftID` and `_uri` to `_recipient`, run the following command
```bash
node ./contracts/mint.js <contractAddress> <_nftID> <_recipient> <_uri>
```

The result of command should be
```bash
{
  logs: [ { msg_index: 0, log: '', events: [Array] } ],
  height: 5570460,
  transactionHash: '697DFC3388446BBE623BEDF5C925296998C2A921725B0D251CCE8580AE6E0572',
  gasWanted: 200000,
  gasUsed: 147431
}
```

You can check the txHash of minting transaction on
```
https://serenity.aurascan.io/transaction/<transactionHash>
```