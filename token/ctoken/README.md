cToken Program
==============

## Issue a SPL token

```
// Address: 5XRVN8CPXWiuii9ufuEE5LmZSdtg9qvKFHpvJCL5UTsc
// Decimals: 9
spl-token create-token

// ATA: G4mHebL2E6vRespbLtSZC4BAofQhLpyocB5kthadNJb2
spl-token create-account 5XRVN8CPXWiuii9ufuEE5LmZSdtg9qvKFHpvJCL5UTsc

// Mint
spl-token mint 5XRVN8CPXWiuii9ufuEE5LmZSdtg9qvKFHpvJCL5UTsc 100 G4mHebL2E6vRespbLtSZC4BAofQhLpyocB5kthadNJb2

// Balance
spl-token balance 5XRVN8CPXWiuii9ufuEE5LmZSdtg9qvKFHpvJCL5UTsc

// Transfer
spl-token transfer --fund-recipient --allow-unfunded-recipient 5XRVN8CPXWiuii9ufuEE5LmZSdtg9qvKFHpvJCL5UTsc 1 BBy1K96Y3bohNeiZTHuQyB53LcfZv6NWCSWqQp89TiVu
```

## Deployment

### devnet

```
token(token mint): 5XRVN8CPXWiuii9ufuEE5LmZSdtg9qvKFHpvJCL5UTsc

cToken program: 2Ranq8nkdU7kqgqjzkpaXVQ9SSdvYu5zoteMVVoBBiBZ
config: EvdaveTPSkJ3UReURhijs74MmpzmFWNnbEdtPvsddoqx

// Solana cToken for 5XRVN8CPXWiuii9ufuEE5LmZSdtg9qvKFHpvJCL5UTsc token with GgVc7NPPeJoas5k1Y9V4uiBc26RVs7N2XEqMbYECp7Ms authority and 4690 destination is 85cAA5dLWmD4NXwXKXKdj7cWXZHw7kNtJLJ6XHxgW5hH

// Solana cToken for 6s5ndQVqWQ5Ast4qZA9rF1BCeMUUB7XsKiNj3tkynXJQ token with 2aguEA4gUmtUdAhcj3UJLwgpAjHRufuMKmN9p6voJv18 authority and 0 destination is CZG1WVr9kEXZMEVoUPWaReHQPJMbNMvCA5o8ZxoBowYJ

// Nosana
Solana cToken for devr1BGQndEW5k5zfvG5FsLyZv1Ap73vNgAHcQ9sUVP token with 6yagrxW7rgWt6t27SHDZJmKj7SgmvkSxSQTKGDMtqhpB authority and 4690 destination is EYrfvoHuhTdMMfiqdMJByLtP4xp1LxeWPRnqykQED1bW
```

### Devnet

```
cToken program: 8xzoC5yD95X4e3WfKCeN68JCScJTutTsjtK8H28cxAW6
config: FqipCjdqPxhGrdGrkeFiaAx5dMR7mUE1mfwMZ8GfReoB

Solana cToken for 5XRVN8CPXWiuii9ufuEE5LmZSdtg9qvKFHpvJCL5UTsc token with 5nkPN2npcj5YbsPEr8rK6p6ZRtS7NPpDzmH1YtuJkRZe authority and 4690 destination is AuymeYEJoMum8F9jxGM9HYhdw7qg7H63mvE26u87U8DF
```


### localhost

```
token(token mint): Dv8EAecmZcAGBpunFjoYhmuvek8wrKN4g7rYuKh3Ha4q
ata: 3saeCEsSCQ9LPk9r5p7hPgJ1QRwSGvnjiNH2eWx28fXX (id.json)

cToken program: 2Ranq8nkdU7kqgqjzkpaXVQ9SSdvYu5zoteMVVoBBiBZ
config: EvdaveTPSkJ3UReURhijs74MmpzmFWNnbEdtPvsddoqx
// Solana cToken for Dv8EAecmZcAGBpunFjoYhmuvek8wrKN4g7rYuKh3Ha4q token with Gf57Mjpds1RVedbUGasVWLtVFpEUSbHEUvxsytxuF7EP authority and 4690 destination is 2dRv2PGiR3KRYWeuc75FJ9NWgCd2c7wduXMpEsBXMCAb

// spl-token create-account --owner 54xTERMyzyYFtWYihN5N94mXfVzhJoMUtUcAenTTzAuP --fee-payer ~/.config/solana/id.json GdxiFDj2FwnvMNwes315TG8pJ6mKdbDLrMpBxQ1rTu2y

// Solana cToken for GdxiFDj2FwnvMNwes315TG8pJ6mKdbDLrMpBxQ1rTu2y token with rwBi3DxPgT9Er1rxRxa4fDAPAFxz5tQqyyUAiV6wvBe authority and 0 destination is 2zRdJ8psKh863z4vkSxET3VQW9gvicJHW3aBPH3BFmRp
```

### mainnet

```
token(token mint): EQ5u8epXaVYLWhM3UXckqUMm2LaVky778pdPFwDbndWo
ata: 7qLSWC3xSx4AWtREnXNFg4Z3skuXW1vDufwj66LG69CR

cToken program: A9SGRcytnfx6U1QrnMUwK5sxYyCYY3MpyrPcyeafhSMF
config: E83sFGG5R3psWhotZSihbPb6DLmLnGK122ZF9q7VmYSM

Solana cToken for EQ5u8epXaVYLWhM3UXckqUMm2LaVky778pdPFwDbndWo token with FYtZbkA2xcC1DY4r6V3wVd3EBZKSaVmPBBfaPDLFznQd authority and 4690 destination is 5bqcayC1seq19cL9kDEuigfRoK27SMGQdTKcJABSeXMy


### CIOTX
Solana cToken for xgf3DoXeqCRVJ9hzU2vbTjfbs6j5BpCHWSmVGBV7Ryx token with ALp8YDxSWtSKgseMA5yKvAckCtGCp2z7ezkAFVWsST7T authority and 0 destination is 8DqC4xipdSztGiTmVRPoAYbfa4hKmTDvPMzmfCUxmU8S

### wSOL
Solana cToken for So11111111111111111111111111111111111111112 token with 7oL9f6deE2oVDzwsXmSmaYHhtWkajRNE2yi5ioCUmvLr authority and 4689 destination is 3qdPWqm2iEYkTXRk3XGkuabEeXfentCUwJP1cCWrkre5
```
