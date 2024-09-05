import { Keypair } from '@solana/web3.js';
import Web3 from 'web3';

const seedBytes = Web3.utils.hexToBytes(`${process.env.SEED}`);
const keypair = Keypair.fromSeed(seedBytes);
console.log("Keypair in Hex:", Web3.utils.bytesToHex(keypair.secretKey));
console.log("Public key:", keypair.publicKey.toString())
