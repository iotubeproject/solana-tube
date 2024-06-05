import * as fs from 'fs';
import {PublicKey, Keypair, Connection, clusterApiUrl} from '@solana/web3.js';
import {Config} from '../../src';

async function main() {
    // const rpc = clusterApiUrl('devnet');
    const rpc = 'http://localhost:8899';

    const secret = JSON.parse(
        fs.readFileSync(`${process.env.PRIVATE_KEY_PATH}`).toString(),
    ) as number[];
    const secretKey = Uint8Array.from(secret);
    const payer = Keypair.fromSecretKey(secretKey);
    const connection = new Connection(rpc, 'confirmed');

    const config = new PublicKey(`${process.env.CONFIG}`);
    const newOwner = new PublicKey(`${process.env.NEW_OWNER}`);
    const cTokenProgramId = new PublicKey(`${process.env.C_TOKEN_PROGRAM_ID}`);

    await Config.transferOwner(
        connection,
        config,
        newOwner,
        cTokenProgramId,
        payer,
    );
    console.log(`Transfer onwer to ${process.env.NEW_OWNER}`);
}

main();
