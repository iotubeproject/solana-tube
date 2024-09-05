# Getting started

## Install dependencies
```
yarn install
npm install -g typescript
npm install -g ts-node
```

## Generate PrivateKey from Seed 

```
export SEED=xxxxxxxxx
ts-node src/keyfromseed.ts
```

## Add New witness 

### Mint token to witness address
It could be done via wallet or solana cli `spl-token mint`

```
solana transfer --allow-unfunded-recipient -u mainnet-beta {xxx} 0.1
spl-token create-account -v -u mainnet-beta --fee-payer {xxx} --owner {xxx} 2Jjhgu65Eedfwy3u2ymQvrcREjenpjqMUXPLkehNL8BS
spl-token mint -v -u mainnet-beta --mint-authority {xxx} 2Jjhgu65Eedfwy3u2ymQvrcREjenpjqMUXPLkehNL8BS 200 {xxx}
```

### Deposit spl token to the program

```
export VALIDATOR_PROGRAM_ID=8adwt6BEu7KPhcaZ3vrrk7xMWwJwz3MM6Rve2YH4YrrA
export REALM=4JhYKsVLgUGdXwt8451PRAsZqwBhii4VWdUUHzVCVMhv
export PRIVATE_KEY_HEX=xxxxxxxxx
export DEPOSIT_AMOUNT=xxxxxxxxx
ts-node src/deposit.ts
```