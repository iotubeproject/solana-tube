import { Connection, PublicKey, Keypair, TransactionInstruction, clusterApiUrl } from '@solana/web3.js';
import { getAssociatedTokenAddress } from '@solana/spl-token';
import BN from 'bn.js';
import * as fs from 'fs';
import Web3 from 'web3';
import { withDepositGoverningTokens, getRealm, getTokenOwnerRecord } from '@solana/spl-governance';
import { optimalSendTransaction } from './utils';


const programId = new PublicKey(`${process.env.VALIDATOR_PROGRAM_ID}`);
const realmAddr = new PublicKey(`${process.env.REALM}`);
const privateKeyBuffer = Web3.utils.hexToBytes(`${process.env.PRIVATE_KEY_HEX}`);
const keypair = Keypair.fromSecretKey(privateKeyBuffer);
const depositAmount = new BN(`${process.env.DEPOSIT_AMOUNT}`);

console.log("PublicKey: ", keypair.publicKey.toString());


async function main() {
    const rpc = clusterApiUrl('mainnet-beta');
    const connection = new Connection(rpc, 'confirmed');

    const realmAccount = await getRealm(connection, realmAddr);
    console.log("realm info: ", realmAccount);

    let ata = await getAssociatedTokenAddress(
        realmAccount.account.communityMint,
        keypair.publicKey,
    );
    console.log("ATA address: ", ata.toString());

    let instructions: TransactionInstruction[] = [];
    let signers: Keypair[] = [];

    const tokenOwnerRecordPk = await withDepositGoverningTokens(
        instructions,
        programId,
        3,
        realmAccount.pubkey,
        ata,
        realmAccount.account.communityMint,
        keypair.publicKey,
        keypair.publicKey,
        keypair.publicKey,
        depositAmount,
    );
    await optimalSendTransaction(connection, instructions, signers, keypair);

    let tokenOwnerRecord = await getTokenOwnerRecord(connection, tokenOwnerRecordPk);
    console.log("tokenOwnerRecord: ", tokenOwnerRecord);
}

main();
