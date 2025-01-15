import * as fs from 'fs';
import {PublicKey, Keypair, Connection} from '@solana/web3.js';
import {CToken} from '../src';

async function main() {
    // const rpc = clusterApiUrl('devnet');
    const rpc = `${process.env.SOLANA_RPC_URL}`;

    const secret = JSON.parse(
        fs.readFileSync(`${process.env.PRIVATE_KEY_PATH}`).toString(),
    ) as number[];
    const secretKey = Uint8Array.from(secret);
    const payer = Keypair.fromSecretKey(secretKey);
    const connection = new Connection(rpc, 'confirmed');

    const cTokenProgramId = new PublicKey(`${process.env.C_TOKEN_PROGRAM_ID}`);
    const cToken = new PublicKey(`${process.env.C_TOKEN}`);
    const config = new PublicKey(`${process.env.CONFIG}`);

    const signature = await CToken.changeLimit(
        connection,
        cToken,
        config,
        BigInt('1000000000000000000'),
        BigInt('1000000'),
        payer,
        cTokenProgramId,
    );
    console.log(`Change ${cToken.toBase58()} tx ${signature.toString()}`);
}

main();
