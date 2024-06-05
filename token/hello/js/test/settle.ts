import * as fs from 'fs';
import {PublicKey, Keypair, Connection, clusterApiUrl, TransactionInstruction, sendAndConfirmTransaction, Transaction} from '@solana/web3.js';
import {
    getAssociatedTokenAddress,
    TOKEN_PROGRAM_ID,
} from '@solana/spl-token';
import * as borsh from 'borsh';
import { cTokenAccount, cTokenAccountSchema } from '../../../ctoken/js/src';

async function main() {
    const secret = JSON.parse(
        fs.readFileSync(`${process.env.PRIVATE_KEY_PATH}`).toString(),
    ) as number[];
    const secretKey = Uint8Array.from(secret);
    const payer = Keypair.fromSecretKey(secretKey);
    const connection = new Connection(clusterApiUrl('devnet'), 'confirmed');

    const cToken = new PublicKey(`${process.env.C_TOKEN}`);
    const cTokenProgramId = new PublicKey(`${process.env.C_TOKEN_PROGRAM_ID}`);

    const tokenMint = new PublicKey(`${process.env.TOKEN_MINT}`);
    const userInfo = await getAssociatedTokenAddress(
        tokenMint,
        payer.publicKey,
    );
    const [authority, _bumpSeed] = PublicKey.findProgramAddressSync(
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

    const helloProgramId = new PublicKey(`${process.env.HELLO_PROGRAM_ID}`);
    const [helloPDAPubkey, _bump_seed] = PublicKey.findProgramAddressSync(
        [Buffer.from("ctoken")],
        helloProgramId,
    );

    console.log(helloPDAPubkey.toString())

    const instruction = new TransactionInstruction({
        keys: [
            {pubkey: cTokenProgramId, isSigner: false, isWritable: false},
            {pubkey: cToken, isSigner: false, isWritable: false},
            {pubkey: authority, isSigner: false, isWritable: false},
            {pubkey: tokenAccount, isSigner: false, isWritable: true},
            {pubkey: userInfo, isSigner: false, isWritable: true},
            {pubkey: helloPDAPubkey, isSigner: false, isWritable: false},
            {pubkey: tokenMint, isSigner: false, isWritable: false},
            {pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false},
        ],
        programId: helloProgramId,
        data: Buffer.alloc(0),
    });
    const signature = await sendAndConfirmTransaction(
        connection,
        new Transaction().add(instruction),
        [payer],
    );

    console.log(`Settle tx ${signature.toString()}`);
}

main();
