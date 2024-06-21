import * as fs from 'fs';
import {PublicKey, Keypair, Connection, clusterApiUrl} from '@solana/web3.js';
import {
    approve,
    getAssociatedTokenAddress,
    TOKEN_PROGRAM_ID,
} from '@solana/spl-token';
import * as borsh from 'borsh';
import {CToken, cTokenAccount, cTokenAccountSchema} from '../src';

async function main() {
    // const rpc = clusterApiUrl('devnet');
    const rpc = `${process.env.SOLANA_RPC_URL}`;

    const secret = JSON.parse(
        fs.readFileSync(`${process.env.PRIVATE_KEY_PATH}`).toString(),
    ) as number[];
    const secretKey = Uint8Array.from(secret);
    const payer = Keypair.fromSecretKey(secretKey);
    const connection = new Connection(rpc, 'confirmed');

    const cToken = new PublicKey(`${process.env.C_TOKEN}`);
    const cTokenProgramId = new PublicKey(`${process.env.C_TOKEN_PROGRAM_ID}`);

    const tokenMint = new PublicKey(`${process.env.TOKEN_MINT}`);
    const config = new PublicKey(`${process.env.CONFIG}`);
    const userTransferAuthority = Keypair.generate();
    const userInfo = await getAssociatedTokenAddress(
        tokenMint,
        payer.publicKey,
    );
    // const [authority, _bumpSeed] = PublicKey.findProgramAddressSync(
    //     [cToken.toBuffer()],
    //     cTokenProgramId,
    // );

    const cTokenData = await connection.getAccountInfo(cToken, 'confirmed');
    const cTokenAccountState = borsh.deserialize(
        cTokenAccountSchema,
        cTokenAccount,
        cTokenData!.data,
    );
    // @ts-ignore
    const tokenAccount = new PublicKey(cTokenAccountState.token);

    const amount = 1000000000n;
    // console.log('Approve token');
    // await approve(
    //     connection,
    //     payer,
    //     userInfo,
    //     userTransferAuthority.publicKey,
    //     payer,
    //     amount,
    // );

    const recipient = '0xBE0a404563130Bc490442dbBCB593E67CcE336b1';

    const signature = await CToken.approveBridge(
        connection,
        cToken,
        config,
        tokenAccount,
        userInfo,
        userTransferAuthority,
        tokenMint,
        TOKEN_PROGRAM_ID,
        amount,
        recipient, 
        [], // payload
        payer,
        cTokenProgramId,
    );
    console.log(`Bridge tx ${signature.toString()}`);
}

main();
