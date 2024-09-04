# Getting started

## Install dependencies
```
yarn install
```

## Add New witness 

### Mint token to witness address
It could be done via wallet or solana cli `spl-token mint`

### Deposit spl token to the program

```
export VALIDATOR_PROGRAM_ID=8adwt6BEu7KPhcaZ3vrrk7xMWwJwz3MM6Rve2YH4YrrA
export REALM=4JhYKsVLgUGdXwt8451PRAsZqwBhii4VWdUUHzVCVMhv
export PRIVATE_KEY_HEX=xxxxxxxxx
export DEPOSIT_AMOUNT=xxxxxxxxx
yarn ts-node src/deposit.ts
```