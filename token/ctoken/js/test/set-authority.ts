import * as fs from 'fs';
import {PublicKey, Keypair, Connection} from '@solana/web3.js';
import {TOKEN_PROGRAM_ID} from '@solana/spl-token';
// import * as borsh from 'borsh';
import {CToken, cTokenAccount, cTokenAccountSchema} from '../src';

async function main() {
    const rpc = `${process.env.SOLANA_RPC_URL}`;

    const secret = JSON.parse(
        fs.readFileSync(`${process.env.PRIVATE_KEY_PATH}`).toString(),
    ) as number[];
    const secretKey = Uint8Array.from(secret);
    const payer = Keypair.fromSecretKey(secretKey);
    const connection = new Connection(rpc, 'confirmed');

    const cToken = new PublicKey(`${process.env.C_TOKEN}`);
    const config = new PublicKey(`${process.env.CONFIG}`);
    const cTokenProgramId = new PublicKey(`${process.env.C_TOKEN_PROGRAM_ID}`);
    const tokenMint = new PublicKey(`${process.env.TOKEN_MINT}`);

    // Find the current PDA authority
    const [tokenAuthority, _bumpSeed] = PublicKey.findProgramAddressSync(
        [cToken.toBuffer()],
        cTokenProgramId,
    );

    // Create a new authority - reading from environment variable
    const newTokenAuthority = new PublicKey(
        `${process.env.NEW_TOKEN_AUTHORITY}`,
    );

    console.log(`Current token authority: ${tokenAuthority.toBase58()}`);
    console.log(`New token authority: ${newTokenAuthority.toBase58()}`);

    // Transfer token authority
    const signature = await CToken.transferTokenAuthority(
        connection,
        cToken,
        tokenAuthority,
        newTokenAuthority,
        tokenMint,
        TOKEN_PROGRAM_ID,
        config,
        payer, // owner
        cTokenProgramId,
    );

    console.log(`Authority transfer transaction: ${signature}`);
    console.log(
        `Successfully transferred token authority from ${tokenAuthority.toBase58()} to ${newTokenAuthority.toBase58()}`,
    );
}

main().catch(err => {
    console.error(err);
    process.exit(1);
});
