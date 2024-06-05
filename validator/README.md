Validator for ioTube Solana bridge
=====================

## Key Code Description

* `process_submit_votes.rs`

Verifying ed25519 signatures from offchain witnesses and converting signed payload into instructions for token program, which are stored in record_transactionaccounts

* `ed25519.rs`

Handling native ed25519 instruction in the program

* `iotube.rs`

Implementation of iotube offchain protocol, parsing verified payload into votes for the governance, and translating them into the instruction for the token program

* `process_execute_transaction.rs`

Consume the record_transaction, invoking CPI for the token program