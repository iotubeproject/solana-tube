import * as fs from 'fs';
import {PublicKey, Keypair, Connection, clusterApiUrl} from '@solana/web3.js';
import {createAccount, createMint, TOKEN_PROGRAM_ID} from '@solana/spl-token';
import {CToken} from '../src';

async function main() {
    const rpc = clusterApiUrl('devnet');
    // const rpc = 'http://localhost:8899';

    const secret = JSON.parse(
        fs.readFileSync(`${process.env.PRIVATE_KEY_PATH}`).toString(),
    ) as number[];
    const secretKey = Uint8Array.from(secret);
    const payer = Keypair.fromSecretKey(secretKey);
    const connection = new Connection(rpc, 'confirmed');

    const cTokenProgramId = new PublicKey(`${process.env.C_TOKEN_PROGRAM_ID}`);
    const cTokenAccount = Keypair.generate();
    const [authority, _bumpSeed] = PublicKey.findProgramAddressSync(
        [cTokenAccount.publicKey.toBuffer()],
        cTokenProgramId,
    );
    const config = new PublicKey(`${process.env.CONFIG}`);

    let destination = 4690;
    let tokenMint;
    let tokenAccount;

    if (destination === 0) {
        // base chain is not solana
        console.log('creating token mint');
        tokenMint = await createMint(
            connection,
            payer,
            authority,
            null,
            9, // decimals
            Keypair.generate(),
            undefined,
            TOKEN_PROGRAM_ID,
        );
        tokenAccount = PublicKey.default;
    } else {
        // base chain is solana
        tokenMint = new PublicKey(`${process.env.TOKEN_MINT}`);

        console.log(`creating token account`);
        tokenAccount = await createAccount(
            connection,
            payer,
            tokenMint,
            authority,
            Keypair.generate(),
        );
    }

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
        1000000000000,
        1000000,
        cTokenProgramId,
        payer,
    );
    console.log(
        `Solana cToken for ${tokenMint} token with ${authority} authority and ${destination} destination is ${cTokenAccount.publicKey}`,
    );
}

main();
