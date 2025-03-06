import AppSolana from '@ledgerhq/hw-app-solana'
import TransportNodeHid from "@ledgerhq/hw-transport-node-hid";
import {PublicKey} from '@solana/web3.js';

async function main() {
    const transport = await TransportNodeHid.create();
    const solana = new AppSolana(transport);

    const addrBuffer = await solana.getAddress("44'/501'/0'/0'");
    const publicKey = new PublicKey(addrBuffer.address);
    const solanaAddress = publicKey.toBase58();

    console.log('Solana address:', solanaAddress);
}

main();
