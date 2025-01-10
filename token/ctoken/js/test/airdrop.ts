import * as fs from 'fs';
import type {TransactionInstruction} from '@solana/web3.js';
import {
    PublicKey,
    Keypair,
    Connection,
    clusterApiUrl,
    sendAndConfirmTransaction,
    Transaction,
    ComputeBudgetProgram,
} from '@solana/web3.js';
import {
    createAssociatedTokenAccountInstruction,
    createTransferInstruction,
    getAssociatedTokenAddress,
} from '@solana/spl-token';

async function main() {
    // const rpc = clusterApiUrl('mainnet-beta');
    const rpc = `${process.env.SOLANA_RPC_URL}`;

    const secret = JSON.parse(
        fs.readFileSync(`${process.env.PRIVATE_KEY_PATH}`).toString(),
    ) as number[];
    const secretKey = Uint8Array.from(secret);
    const payer = Keypair.fromSecretKey(secretKey);
    const connection = new Connection(rpc, 'confirmed');

    const tokenMint = new PublicKey(`${process.env.TOKEN_MINT}`);

    const users = [
        {
            account: 'ALp8YDxSWtSKgseMA5yKvAckCtGCp2z7ezkAFVWsST7T',
            amount: 7616418287370,
            offCurve: true,
        },
        {
            account: 'E7DFEyxUjzD6aSvt4U5CNeYFJK8jGhYJ6xrvkQ1dt7RK',
            amount: 999202973032,
            offCurve: false,
        },
        {
            account: 'cFeGY3Qpi9tCCC6Lu5NqpPwUe45bFkJYEQ5RLpzkm3e',
            amount: 678450562107,
            offCurve: false,
        },
        {
            account: 'Eg6vAgtRjS7ojwAHh3SXhRKj1ybm2q8zinZ57fWzBRvC',
            amount: 20000000000,
            offCurve: false,
        },
        {
            account: '4HsXosuruDaMYfXQ9as6XYpweUhjACXPFrV9W9JqiYWV',
            amount: 20000000000,
        },
        {
            account: 'JDJBgDwKBATpti2nYYYfrmt2uwFhZM87vD4d3zCfukC9',
            amount: 20000000000,
            offCurve: false,
        },
    ];

    const ixs: TransactionInstruction[] = [];
    const payerATA = await getAssociatedTokenAddress(
        tokenMint,
        payer.publicKey,
    );
    for (let i = 0; i < users.length; i++) {
        const user = users[i];
        const userAccount = new PublicKey(user.account);
        const ata = await getAssociatedTokenAddress(
            tokenMint,
            userAccount,
            user.offCurve,
        );

        ixs.push(
            createAssociatedTokenAccountInstruction(
                payer.publicKey,
                ata,
                userAccount,
                tokenMint,
            ),
        );
        ixs.push(
            createTransferInstruction(
                payerATA,
                ata,
                payer.publicKey,
                user.amount,
            ),
        );
    }

    const addPriorityFee = ComputeBudgetProgram.setComputeUnitPrice({
        microLamports: 1000,
    });
    const signature = await sendAndConfirmTransaction(
        connection,
        new Transaction().add(addPriorityFee, ...ixs),
        [payer],
    );

    console.log(`Airdrop tx ${signature.toString()}`);
}

main();
