import * as fs from 'fs';
import {PublicKey, Keypair, Connection, clusterApiUrl} from '@solana/web3.js';
import {Config} from '../../src';

async function main() {
    // const rpc = clusterApiUrl('mainnet-beta');
    const rpc = `${process.env.SOLANA_RPC_URL}`;

    const secret = JSON.parse(
        fs.readFileSync(`${process.env.PRIVATE_KEY_PATH}`).toString(),
    ) as number[];
    const secretKey = Uint8Array.from(secret);
    const payer = Keypair.fromSecretKey(secretKey);
    const connection = new Connection(rpc, 'confirmed');

    const config = new PublicKey(`${process.env.CONFIG}`);
    const authority = new PublicKey(`${process.env.AUTHORITY}`);
    const cTokenProgramId = new PublicKey(`${process.env.C_TOKEN_PROGRAM_ID}`);

    await Config.changeAuthority(
        connection,
        config,
        authority,
        cTokenProgramId,
        payer,
    );
    console.log(`Change authority to ${process.env.AUTHORITY}`);
}

main();
