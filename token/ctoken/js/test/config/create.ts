import * as fs from 'fs';
import {PublicKey, Keypair, Connection, clusterApiUrl} from '@solana/web3.js';
import {Config} from '../../src';

async function main() {
    // const rpc = clusterApiUrl('devnet');
    const rpc = `${process.env.SOLANA_RPC_URL}`;

    const secret = JSON.parse(
        fs.readFileSync(`${process.env.PRIVATE_KEY_PATH}`).toString(),
    ) as number[];
    const secretKey = Uint8Array.from(secret);
    const payer = Keypair.fromSecretKey(secretKey);
    const connection = new Connection(rpc, 'confirmed');
    const authority = new PublicKey(`${process.env.AUTHORITY}`);

    const seed = process.env.SEED!;
    const cTokenProgramId = new PublicKey(`${process.env.C_TOKEN_PROGRAM_ID}`);
    const configPubkey = await PublicKey.createWithSeed(
        payer.publicKey,
        seed,
        cTokenProgramId,
    );

    console.log(`creating cToken config`);
    await Config.initialConfig(
        connection,
        configPubkey,
        seed,
        payer.publicKey, // owner
        authority, // authority
        0, // fee
        payer.publicKey, // fee collector,
        payer,
        cTokenProgramId,
    );
    console.log(`Created cToken config is ${configPubkey}`);
}

main();
