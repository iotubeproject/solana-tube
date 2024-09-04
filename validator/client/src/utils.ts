
import {
    Connection, PublicKey, Keypair, TransactionInstruction,
    ComputeBudgetProgram, LAMPORTS_PER_SOL, Transaction,
    sendAndConfirmTransaction, SystemProgram
} from '@solana/web3.js';
import { getSimulationComputeUnits } from '@solana-developers/helpers';


export async function optimalSendTransaction(connection: Connection, instructions: TransactionInstruction[], signers: Keypair[], feePayer: Keypair) {
    // Create the priority fee instructions
    const units = await getSimulationComputeUnits(connection, instructions, feePayer.publicKey, [],)

    const computePriceIx = ComputeBudgetProgram.setComputeUnitPrice({
        microLamports: 100000,
    });

    const computeLimitIx = ComputeBudgetProgram.setComputeUnitLimit({
        units: Math.ceil(units * 1.5),
    });


    let transaction = new Transaction().add(
        computePriceIx,
        computeLimitIx,
        ...instructions
    )

    const latestBlockHash = await connection.getLatestBlockhash()
    transaction.recentBlockhash = latestBlockHash.blockhash;
    transaction.feePayer = feePayer.publicKey;
    signers.push(feePayer);
    // Send the transaction
    try {
        const txid = await sendAndConfirmTransaction(connection, transaction, signers);
        console.log("Transaction sent successfully with signature", txid);
    } catch (e) {
        console.error("Failed to send transaction:", e);
    }

    return;
}

