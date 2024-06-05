The Solana bridge program consists of two parts: the token program and the validator program. The token program manages Solana (native/spl-token) assets on the ioTube bridge. The validator program oversees off-chain witnesses of events on other chains and validates their proof for the actions of the token program (transfer/mint/burn).

## Description

Solana-IoTeX bridge, leveraged on the [ioTube Bridge architecture](https://docs.iotube.org/introduction/overview-and-architecture), operates on the existing witnesses network with Solana client support. Once two-thirds of witnesses sign the event, a consensus is reached, and the relayer submits the data and signatures to the contract. When bridging assets from Solana to IoTeX, witnesses use the `secp256k1` elliptic curve to sign the data, which can be verified in EVM smart contracts; in the opposite direction, the `ed25519` elliptic curve is used.


### Workflow

![whiteboard_exported_image](https://github.com/iotexproject/iips/assets/55118568/c7520d37-8bc6-4618-a9b3-263ac44f84e8)


## Getting Started

### Dependencies

* [Cargo](https://github.com/rust-lang/cargo)
* [Solana CLI](https://docs.solanalabs.com/cli/install)

### Building program

* Token: `cd token/ctoken/program/ && cargo build-bpf`

* Validator: `cd validator/ && make build`

### Deploying program

* Token: `cd token/ctoken/ && solana program deploy ./target/deploy/ctoken.so`

* Validator: `cd validator/ && make deploy`




## License

This project is licensed under the [NAME HERE] License - see the LICENSE.md file for details