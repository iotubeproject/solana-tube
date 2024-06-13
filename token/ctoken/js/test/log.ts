import * as borsh from 'borsh';
import {BridgeLog, BridgeLogSchema} from '../src';
import {PublicKey} from '@solana/web3.js';

async function main() {
    const log =
        'Bridge: bfe83295da43156698c425b75599ff3c6a637ed4d49544659fc92efaa5c47ba202000000000000002aac73de831e428cc021c172a6de30f16a22115f5beeb67c41023a4d968e9a8a2a00000030786237313034346236636438343364646331316534323938653964613634646130303836346530393940420f0000000000000000000000000052120000020000006768';

    const data = Buffer.from(log.substring(8), 'hex');

    const birdgeLog = borsh.deserialize(BridgeLogSchema, BridgeLog, data);
    // @ts-ignore
    const token = birdgeLog.token;
    // @ts-ignore
    const index = birdgeLog.index;
    // @ts-ignore
    const sender = birdgeLog.sender;
    // @ts-ignore
    const amount = birdgeLog.amount;
    // @ts-ignore
    const fee = birdgeLog.fee;
    // @ts-ignore
    const recipient = birdgeLog.recipient;
    // @ts-ignore
    const destination = birdgeLog.destination;
    // @ts-ignore
    const payload = birdgeLog.payload;

    console.log(`Bridge log:
    {
        token: ${new PublicKey(token)},
        index: ${index},
        sender: ${new PublicKey(sender)},
        recipient: ${recipient},
        amount: ${amount},
        fee: ${fee},
        destination: ${destination},
        payload: ${Buffer.from(payload).toString('hex')},
    }`);
}

main();
