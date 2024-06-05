import * as fs from 'fs';
import {PublicKey, Keypair, Connection, clusterApiUrl} from '@solana/web3.js';
import * as borsh from 'borsh';
import {cTokenConfig, cTokenConfigSchema} from '../../src';

async function main() {
    // const rpc = clusterApiUrl('devnet');
    const rpc = 'http://localhost:8899';
    const connection = new Connection(rpc, 'confirmed');

    const config = new PublicKey(`${process.env.CONFIG}`);

    const configData = await connection.getAccountInfo(config, 'confirmed');
    const configState = borsh.deserialize(
        cTokenConfigSchema,
        cTokenConfig,
        configData!.data,
    );
    // @ts-ignore
    const is_initialized = configState.initialized;
    // @ts-ignore
    const owner = new PublicKey(configState.owner);
    // @ts-ignore
    const authority = new PublicKey(configState.authority);
    // @ts-ignore
    const fee = configState.fee;
    // @ts-ignore
    const fee_collector = new PublicKey(configState.fee_collector);

    console.log(`Config ${process.env.CONFIG}:
    {
        is_initialized: ${is_initialized},
        owner: ${owner},
        authority: ${authority},
        fee: ${fee},
        fee_collector: ${fee_collector}
    }`);
}

main();
