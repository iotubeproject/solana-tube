import * as fs from 'fs';
import {PublicKey, Keypair, Connection, clusterApiUrl} from '@solana/web3.js';
import {Config} from '../../src';
import AppSolana from '@ledgerhq/hw-app-solana'
import TransportNodeHid from "@ledgerhq/hw-transport-node-hid";

async function main() {
    // const rpc = clusterApiUrl('devnet');
    const rpc = `${process.env.SOLANA_RPC_URL}`;

    // const secret = JSON.parse(
    //     fs.readFileSync(`${process.env.PRIVATE_KEY_PATH}`).toString(),
    // ) as number[];
    // const secretKey = Uint8Array.from(secret);
    // const payer = Keypair.fromSecretKey(secretKey);
    const connection = new Connection(rpc, 'confirmed');

    const config = new PublicKey(`${process.env.CONFIG}`);
    const newOwner = new PublicKey(`${process.env.NEW_OWNER}`);
    const cTokenProgramId = new PublicKey(`${process.env.C_TOKEN_PROGRAM_ID}`);

    const transport = await TransportNodeHid.create();
    const solana = new AppSolana(transport);
    const addrBuffer = await solana.getAddress("44'/501'/0'/0'");
    const ownerAddr = new PublicKey(addrBuffer.address);

    await Config.transferOwnerWithLedger(
        connection,
        config,
        newOwner,
        cTokenProgramId,
        solana,
        ownerAddr,
    );
    console.log(`Transfer onwer to ${process.env.NEW_OWNER}`);
}

main();
