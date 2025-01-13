import * as fs from 'fs';
import {PublicKey, Keypair, Connection} from '@solana/web3.js';
import {TOKEN_PROGRAM_ID} from '@solana/spl-token';
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

    const config = new PublicKey(`${process.env.CONFIG}`);

    const secretPre = JSON.parse(`${process.env.ACCOUNT_KEYPAIR}`) as number[];
    const secretKeyPre = Uint8Array.from(secretPre);
    const cTokenAccount = Keypair.fromSecretKey(secretKeyPre);

    const destination = 0;
    const tokenMint = new PublicKey(`${process.env.TOKEN_MINT}`);
    const tokenAccount = new PublicKey(`${process.env.TOKEN_ACCOUNT}`);

    const [authority, _bumpSeed] = PublicKey.findProgramAddressSync(
        [cTokenAccount.publicKey.toBuffer()],
        cTokenProgramId,
    );

    console.log(`cTokenAccount: [${cTokenAccount.secretKey.toString()}]`);
    console.log(`authority: ${authority.toBase58()}`);
    console.log(`tokenAccount: ${tokenAccount.toBase58()}`);

    console.log(`creating cToken account`);
    await CToken.createCToken(
        connection,
        cTokenAccount,
        authority,
        tokenMint,
        tokenAccount,
        TOKEN_PROGRAM_ID,
        payer, // owner
        config,
        destination,
        10000000000000000n,
        1000000000,
        cTokenProgramId,
        payer,
    );
    console.log(
        `Solana cToken for ${tokenMint} token with ${authority} authority and ${destination} destination is ${cTokenAccount.publicKey}`,
    );
}

main();
