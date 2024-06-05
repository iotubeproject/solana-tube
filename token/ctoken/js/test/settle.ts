import * as fs from 'fs';
import {PublicKey, Keypair, Connection, clusterApiUrl} from '@solana/web3.js';
import {getAssociatedTokenAddress, TOKEN_PROGRAM_ID} from '@solana/spl-token';
import * as borsh from 'borsh';
import {CToken, cTokenAccount, cTokenAccountSchema} from '../src';

async function main() {
    // const rpc = clusterApiUrl('devnet');
    const rpc = 'http://localhost:8899';

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
    const userInfo = await getAssociatedTokenAddress(
        tokenMint,
        payer.publicKey,
    );
    const [tokenAuthority, _bumpSeed] = PublicKey.findProgramAddressSync(
        [cToken.toBuffer()],
        cTokenProgramId,
    );

    const cTokenData = await connection.getAccountInfo(cToken, 'confirmed');
    const cTokenAccountState = borsh.deserialize(
        cTokenAccountSchema,
        cTokenAccount,
        cTokenData!.data,
    );
    // @ts-ignore
    const tokenAccount = new PublicKey(cTokenAccountState.token);

    const amount = 10000n;

    const signature = await CToken.settle(
        connection,
        cToken,
        config,
        tokenAuthority,
        tokenAccount,
        userInfo,
        tokenMint,
        TOKEN_PROGRAM_ID,
        amount,
        payer,
        cTokenProgramId,
    );
    console.log(`Settle tx ${signature.toString()}`);
}

main();
