import {PublicKey, Connection, clusterApiUrl} from '@solana/web3.js';
import * as borsh from 'borsh';
import {cTokenAccount, cTokenAccountSchema} from '../src';

async function main() {
    // const rpc = clusterApiUrl('devnet');
    const rpc = `${process.env.SOLANA_RPC_URL}`;
    const connection = new Connection(rpc, 'confirmed');

    const cToken = new PublicKey(`${process.env.C_TOKEN}`);

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
    const is_initialized = cTokenAccountState.initialized;
    // @ts-ignore
    const bumpSeed = cTokenAccountState.bump_seed;
    // @ts-ignore
    const tokenProgramId = new PublicKey(cTokenAccountState.token_program_id);
    // @ts-ignore
    const config = new PublicKey(cTokenAccountState.config);
    // @ts-ignore
    const token = new PublicKey(cTokenAccountState.token);
    // @ts-ignore
    const tokenMint = new PublicKey(cTokenAccountState.token_mint);
    // @ts-ignore
    const destination = cTokenAccountState.destination;
    // @ts-ignore
    const index = cTokenAccountState.index;
    // @ts-ignore
    const max = cTokenAccountState.max;
    // @ts-ignore
    const min = cTokenAccountState.min;

    console.log(`cToken ${process.env.C_TOKEN}:
    {
        is_initialized: ${is_initialized},
        bump_seed: ${bumpSeed},
        token_program_id: ${tokenProgramId},
        config: ${config},
        token: ${token},
        token_mint: ${tokenMint},
        destination: ${destination},
        index: ${index},
        max: ${max},
        min: ${min}
    }`);
}

main();
