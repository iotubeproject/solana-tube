import * as borsh from 'borsh';
import {BridgeLog, BridgeLogSchema} from '../src';
import {PublicKey} from '@solana/web3.js';

async function main() {
    const log =
        'Bridge: 37cb6e050ce3ad2b3939a555aebde7f15a9c9d37d5ef1ce4fce9e21a4b8c44080100000000000000077423955995f4033ad7ebce2ff04a731c98823c220b40429548094df6684e882a000000307862373130343462366364383433646463313165343239386539646136346461303038363465303939a086010000000000000000000000000051120000';

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

    console.log(`Bridge log:
    {
        token: ${new PublicKey(token)},
        index: ${index},
        sender: ${new PublicKey(sender)},
        recipient: ${recipient},
        amount: ${amount},
        fee: ${fee},
        destination: ${destination},
    }`);
}

main();
