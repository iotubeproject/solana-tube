import { getRealm, getTokenOwnerRecord, withDepositGoverningTokens } from '@solana/spl-governance';
import { getAssociatedTokenAddress } from '@solana/spl-token';
import { Connection, Keypair, PublicKey, TransactionInstruction, clusterApiUrl } from '@solana/web3.js';
import BN from 'bn.js';
import Web3 from 'web3';
import { optimalSendTransaction } from './utils';

const seedBytes = Web3.utils.hexToBytes(`${process.env.SEED}`);
const keypair = Keypair.fromSeed(seedBytes);

const programId = new PublicKey(`${process.env.VALIDATOR_PROGRAM_ID}`);
const realmAddr = new PublicKey(`${process.env.REALM}`);
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
