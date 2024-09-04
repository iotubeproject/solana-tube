import type {
    ConfirmOptions,
    Connection,
    Keypair,
    TransactionSignature,
} from '@solana/web3.js';
import {
    ComputeBudgetProgram,
    TransactionInstruction,
    sendAndConfirmTransaction,
} from '@solana/web3.js';
import {PublicKey, SystemProgram, Transaction} from '@solana/web3.js';
import * as borsh from 'borsh';
import {createApproveInstruction} from '@solana/spl-token';

class Assignable {
    // @ts-ignore
    constructor(properties) {
        Object.keys(properties).map(key => {
            // @ts-ignore
            return (this[key] = properties[key]);
        });
    }
}
export class cTokenConfig extends Assignable {}
export const cTokenConfigSchema = new Map([
    [
        cTokenConfig,
        {
            kind: 'struct',
            fields: [
                ['initialized', 'u8'],
                ['owner', [32]],
                ['authority', [32]],
                ['fee', 'u64'],
                ['fee_collector', [32]],
            ],
        },
    ],
]);
export class cTokenAccount extends Assignable {}
export const cTokenAccountSchema = new Map([
    [
        cTokenAccount,
        {
            kind: 'struct',
            fields: [
                ['initialized', 'u8'],
                ['bump_seed', 'u8'],
                ['token_program_id', [32]],
                ['config', [32]],
                ['token', [32]],
                ['token_mint', [32]],
                ['destination', 'u32'],
                ['index', 'u64'],
                ['max', 'u64'],
                ['min', 'u64'],
            ],
        },
    ],
]);

export class BridgeLog extends Assignable {}
export const BridgeLogSchema = new Map([
    [
        BridgeLog,
        {
            kind: 'struct',
            fields: [
                ['token', [32]],
                ['index', 'u64'],
                ['sender', [32]],
                ['recipient', 'string'],
                ['amount', 'u64'],
                ['fee', 'u64'],
                ['destination', 'u32'],
                ['payload', ['u8']],
            ],
        },
    ],
]);

export const C_TOKEN_CONFIG_SIZE = borsh.serialize(
    cTokenConfigSchema,
    new cTokenConfig({
        initialized: true,
        owner: new PublicKey(
            '7vLEZP5JHhKVg3HEGSWcFNaxAKg7L633uMT7ePqmn98V',
        ).toBytes(),
        authority: new PublicKey(
            '7vLEZP5JHhKVg3HEGSWcFNaxAKg7L633uMT7ePqmn98V',
        ).toBytes(),
        fee: 0,
        fee_collector: new PublicKey(
            '7vLEZP5JHhKVg3HEGSWcFNaxAKg7L633uMT7ePqmn98V',
        ).toBytes(),
    }),
).length;
export const C_TOKEN_ACCOUNT_SIZE = borsh.serialize(
    cTokenAccountSchema,
    new cTokenAccount({
        initialized: true,
        bump_seed: 100,
        token_program_id: new PublicKey(
            '7vLEZP5JHhKVg3HEGSWcFNaxAKg7L633uMT7ePqmn98V',
        ).toBytes(),
        config: new PublicKey(
            '7vLEZP5JHhKVg3HEGSWcFNaxAKg7L633uMT7ePqmn98V',
        ).toBytes(),
        token: new PublicKey(
            '7vLEZP5JHhKVg3HEGSWcFNaxAKg7L633uMT7ePqmn98V',
        ).toBytes(),
        token_mint: new PublicKey(
            '7vLEZP5JHhKVg3HEGSWcFNaxAKg7L633uMT7ePqmn98V',
        ).toBytes(),
        destination: 4690,
        index: 0,
        max: 1000000000000,
        min: 100000000,
    }),
).length;

enum InstructionVariant {
    Config = 0,
    TransferOwner,
    ChangeAuthority,
    ChangeFee,
    Create,
    ChangeLimit,
    Bridge,
    Settle,
}

class ConfigPayload extends Assignable {}
const ConfigPayloadSchema = new Map([
    [
        ConfigPayload,
        {
            kind: 'struct',
            fields: [
                ['id', 'u8'],
                ['fee', 'u64'],
            ],
        },
    ],
]);

class TransferOwnerPayload extends Assignable {}
const TransferOwnerPayloadSchema = new Map([
    [
        TransferOwnerPayload,
        {
            kind: 'struct',
            fields: [['id', 'u8']],
        },
    ],
]);

class ChangeAuthorityPayload extends Assignable {}
const ChangeAuthorityPayloadSchema = new Map([
    [
        ChangeAuthorityPayload,
        {
            kind: 'struct',
            fields: [['id', 'u8']],
        },
    ],
]);

class ChangeFeePayload extends Assignable {}
const ChangeFeePayloadSchema = new Map([
    [
        ChangeFeePayload,
        {
            kind: 'struct',
            fields: [
                ['id', 'u8'],
                ['fee', 'u64'],
            ],
        },
    ],
]);

class CreatePayload extends Assignable {}
const CreatePayloadSchema = new Map([
    [
        CreatePayload,
        {
            kind: 'struct',
            fields: [
                ['id', 'u8'],
                ['destination', 'u32'],
                ['max', 'u64'],
                ['min', 'u64'],
            ],
        },
    ],
]);

class BridgePayload extends Assignable {}
const BridgePayloadSchema = new Map([
    [
        BridgePayload,
        {
            kind: 'struct',
            fields: [
                ['id', 'u8'],
                ['amount', 'u64'],
                ['recipient', 'string'],
                ['payload', ['u8']],
            ],
        },
    ],
]);

class SettlePayload extends Assignable {}
const SettlePayloadSchema = new Map([
    [
        SettlePayload,
        {
            kind: 'struct',
            fields: [
                ['id', 'u8'],
                ['amount', 'u64'],
            ],
        },
    ],
]);

class ChangeLimitPayload extends Assignable {}
const ChangeLimitPayloadSchema = new Map([
    [
        ChangeLimitPayload,
        {
            kind: 'struct',
            fields: [
                ['id', 'u8'],
                ['max', 'u64'],
                ['min', 'u64'],
            ],
        },
    ],
]);

export class Config {
    static async getMinBalanceRentForExemptCToken(
        connection: Connection,
    ): Promise<number> {
        return await connection.getMinimumBalanceForRentExemption(
            C_TOKEN_CONFIG_SIZE,
        );
    }

    static configInstruction(
        config: PublicKey,
        owner: PublicKey,
        authority: PublicKey,
        fee: number,
        feeCollector: PublicKey,
        cTokenProgramId: PublicKey,
    ): TransactionInstruction {
        const keys = [
            {pubkey: config, isSigner: false, isWritable: true},
            {pubkey: owner, isSigner: false, isWritable: false},
            {pubkey: authority, isSigner: false, isWritable: false},
            {pubkey: feeCollector, isSigner: false, isWritable: false},
        ];

        const data = new ConfigPayload({
            id: InstructionVariant.Config,
            fee: fee,
        });

        return new TransactionInstruction({
            keys,
            programId: cTokenProgramId,
            data: Buffer.from(borsh.serialize(ConfigPayloadSchema, data)),
        });
    }

    static transferOwnerInstruction(
        config: PublicKey,
        owner: PublicKey,
        newOwner: PublicKey,
        cTokenProgramId: PublicKey,
    ): TransactionInstruction {
        const keys = [
            {pubkey: config, isSigner: false, isWritable: true},
            {pubkey: owner, isSigner: true, isWritable: false},
            {pubkey: newOwner, isSigner: false, isWritable: false},
        ];

        const data = new TransferOwnerPayload({
            id: InstructionVariant.TransferOwner,
        });

        return new TransactionInstruction({
            keys,
            programId: cTokenProgramId,
            data: Buffer.from(
                borsh.serialize(TransferOwnerPayloadSchema, data),
            ),
        });
    }

    static changeAuthorityInstruction(
        config: PublicKey,
        owner: PublicKey,
        authority: PublicKey,
        cTokenProgramId: PublicKey,
    ): TransactionInstruction {
        const keys = [
            {pubkey: config, isSigner: false, isWritable: true},
            {pubkey: owner, isSigner: true, isWritable: false},
            {pubkey: authority, isSigner: false, isWritable: false},
        ];

        const data = new ChangeAuthorityPayload({
            id: InstructionVariant.ChangeAuthority,
        });

        return new TransactionInstruction({
            keys,
            programId: cTokenProgramId,
            data: Buffer.from(
                borsh.serialize(ChangeAuthorityPayloadSchema, data),
            ),
        });
    }

    static async initialConfig(
        connection: Connection,
        config: PublicKey,
        seed: string,
        owner: PublicKey,
        authority: PublicKey,
        fee: number,
        feeCollector: PublicKey,
        payer: Keypair,
        cTokenProgramId: PublicKey,
        confirmOptions?: ConfirmOptions,
    ): Promise<TransactionSignature> {
        const balanceNeeded =
            await Config.getMinBalanceRentForExemptCToken(connection);
        const transaction = new Transaction();
        transaction.add(
            SystemProgram.createAccountWithSeed({
                fromPubkey: payer.publicKey,
                basePubkey: payer.publicKey,
                seed: seed,
                newAccountPubkey: config,
                lamports: balanceNeeded,
                space: C_TOKEN_CONFIG_SIZE,
                programId: cTokenProgramId,
            }),
        );

        const instruction = Config.configInstruction(
            config,
            owner,
            authority,
            fee,
            feeCollector,
            cTokenProgramId,
        );

        transaction.add(instruction);
        return await sendAndConfirmTransaction(
            connection,
            transaction,
            [payer],
            confirmOptions,
        );
    }

    static async transferOwner(
        connection: Connection,
        config: PublicKey,
        newOwner: PublicKey,
        cTokenProgramId: PublicKey,
        owner: Keypair,
        confirmOptions?: ConfirmOptions,
    ): Promise<TransactionSignature> {
        return await sendAndConfirmTransaction(
            connection,
            new Transaction().add(
                Config.transferOwnerInstruction(
                    config,
                    owner.publicKey,
                    newOwner,
                    cTokenProgramId,
                ),
            ),
            [owner],
            confirmOptions,
        );
    }

    static async changeAuthority(
        connection: Connection,
        config: PublicKey,
        authority: PublicKey,
        cTokenProgramId: PublicKey,
        owner: Keypair,
        confirmOptions?: ConfirmOptions,
    ): Promise<TransactionSignature> {
        return await sendAndConfirmTransaction(
            connection,
            new Transaction().add(
                Config.changeAuthorityInstruction(
                    config,
                    owner.publicKey,
                    authority,
                    cTokenProgramId,
                ),
            ),
            [owner],
            confirmOptions,
        );
    }
}

export class CToken {
    static async getMinBalanceRentForExemptCToken(
        connection: Connection,
    ): Promise<number> {
        return await connection.getMinimumBalanceForRentExemption(
            C_TOKEN_ACCOUNT_SIZE,
        );
    }

    static createCTokenInstruction(
        cToken: PublicKey,
        authority: PublicKey,
        tokenMint: PublicKey,
        tokenAccount: PublicKey,
        tokenProgramId: PublicKey,
        owner: PublicKey,
        config: PublicKey,
        destination: number,
        max: number,
        min: number,
        cTokenProgramId: PublicKey,
    ): TransactionInstruction {
        const keys = [
            {pubkey: cToken, isSigner: false, isWritable: true},
            {pubkey: authority, isSigner: false, isWritable: false},
            {pubkey: tokenMint, isSigner: false, isWritable: false},
            {pubkey: tokenAccount, isSigner: false, isWritable: false},
            {pubkey: tokenProgramId, isSigner: false, isWritable: false},
            {pubkey: owner, isSigner: true, isWritable: false},
            {pubkey: config, isSigner: false, isWritable: false},
        ];

        const createData = new CreatePayload({
            id: InstructionVariant.Create,
            destination: destination,
            max: max,
            min: min,
        });

        return new TransactionInstruction({
            keys,
            programId: cTokenProgramId,
            data: Buffer.from(borsh.serialize(CreatePayloadSchema, createData)),
        });
    }

    static bridgeInstruction(
        cToken: PublicKey,
        config: PublicKey,
        cTokenTokenAccount: PublicKey,
        userAccount: PublicKey,
        userTransferAuthority: PublicKey,
        tokenMint: PublicKey,
        tokenProgramInfo: PublicKey,
        amount: bigint,
        recipient: string,
        payload: number[],
        cTokenProgramId: PublicKey,
    ): TransactionInstruction {
        const keys = [
            {pubkey: cToken, isSigner: false, isWritable: true},
            {pubkey: cTokenTokenAccount, isSigner: false, isWritable: true},
            {pubkey: userAccount, isSigner: false, isWritable: true},
            {pubkey: userTransferAuthority, isSigner: true, isWritable: false},
            {pubkey: tokenMint, isSigner: false, isWritable: true},
            {pubkey: tokenProgramInfo, isSigner: false, isWritable: false},
            {pubkey: config, isSigner: false, isWritable: false},
        ];

        const bridgeData = new BridgePayload({
            id: InstructionVariant.Bridge,
            amount: amount,
            recipient: recipient,
            payload: payload,
        });

        return new TransactionInstruction({
            keys,
            programId: cTokenProgramId,
            data: Buffer.from(borsh.serialize(BridgePayloadSchema, bridgeData)),
        });
    }

    static settleInstruction(
        cToken: PublicKey,
        config: PublicKey,
        tokenAuthority: PublicKey,
        cTokenTokenAccount: PublicKey,
        userAccount: PublicKey,
        authority: PublicKey,
        tokenMint: PublicKey,
        tokenProgramInfo: PublicKey,
        amount: bigint,
        cTokenProgramId: PublicKey,
    ): TransactionInstruction {
        const keys = [
            {pubkey: cToken, isSigner: false, isWritable: false},
            {pubkey: tokenAuthority, isSigner: false, isWritable: false},
            {pubkey: cTokenTokenAccount, isSigner: false, isWritable: true},
            {pubkey: userAccount, isSigner: false, isWritable: true},
            {pubkey: authority, isSigner: true, isWritable: false},
            {pubkey: tokenMint, isSigner: false, isWritable: true},
            {pubkey: tokenProgramInfo, isSigner: false, isWritable: false},
            {pubkey: config, isSigner: false, isWritable: false},
        ];

        const settleData = new SettlePayload({
            id: InstructionVariant.Settle,
            amount: amount,
        });

        return new TransactionInstruction({
            keys,
            programId: cTokenProgramId,
            data: Buffer.from(borsh.serialize(SettlePayloadSchema, settleData)),
        });
    }

    static changeLimitInstruction(
        cToken: PublicKey,
        config: PublicKey,
        owner: PublicKey,
        max: bigint,
        min: bigint,
        cTokenProgramId: PublicKey,
    ): TransactionInstruction {
        const keys = [
            {pubkey: config, isSigner: false, isWritable: false},
            {pubkey: cToken, isSigner: false, isWritable: true},
            {pubkey: owner, isSigner: true, isWritable: false},
        ];

        const data = new ChangeLimitPayload({
            id: InstructionVariant.ChangeLimit,
            max: max,
            min: min,
        });

        return new TransactionInstruction({
            keys,
            programId: cTokenProgramId,
            data: Buffer.from(borsh.serialize(ChangeLimitPayloadSchema, data)),
        });
    }

    static async createCToken(
        connection: Connection,
        cToken: Keypair,
        authority: PublicKey,
        tokenMint: PublicKey,
        tokenAccount: PublicKey,
        tokenProgramId: PublicKey,
        owner: Keypair,
        config: PublicKey,
        destination: number,
        max: number,
        min: number,
        cTokenProgramId: PublicKey,
        payer: Keypair,
        confirmOptions?: ConfirmOptions,
    ): Promise<TransactionSignature> {
        const balanceNeeded =
            await CToken.getMinBalanceRentForExemptCToken(connection);
        const transaction = new Transaction();
        transaction.add(
            SystemProgram.createAccount({
                fromPubkey: payer.publicKey,
                newAccountPubkey: cToken.publicKey,
                lamports: balanceNeeded,
                space: C_TOKEN_ACCOUNT_SIZE,
                programId: cTokenProgramId,
            }),
        );

        const instruction = CToken.createCTokenInstruction(
            cToken.publicKey,
            authority,
            tokenMint,
            tokenAccount,
            tokenProgramId,
            owner.publicKey,
            config,
            destination,
            max,
            min,
            cTokenProgramId,
        );

        transaction.add(instruction);
        return await sendAndConfirmTransaction(
            connection,
            transaction,
            [payer, cToken, owner],
            confirmOptions,
        );
    }

    static async bridge(
        connection: Connection,
        cToken: PublicKey,
        config: PublicKey,
        tokenAccount: PublicKey,
        userAccount: PublicKey,
        userTransferAuthority: Keypair,
        tokenMint: PublicKey,
        tokenProgramId: PublicKey,
        amount: bigint,
        recipient: string,
        payload: number[],
        payer: Keypair,
        cTokenProgramId: PublicKey,
        confirmOptions?: ConfirmOptions,
    ): Promise<TransactionSignature> {
        return await sendAndConfirmTransaction(
            connection,
            new Transaction().add(
                CToken.bridgeInstruction(
                    cToken,
                    config,
                    tokenAccount,
                    userAccount,
                    userTransferAuthority.publicKey,
                    tokenMint,
                    tokenProgramId,
                    amount,
                    recipient,
                    payload,
                    cTokenProgramId,
                ),
            ),
            [payer, userTransferAuthority],
            confirmOptions,
        );
    }

    static async approveBridge(
        connection: Connection,
        cToken: PublicKey,
        config: PublicKey,
        tokenAccount: PublicKey,
        userAccount: PublicKey,
        userTransferAuthority: Keypair,
        tokenMint: PublicKey,
        tokenProgramId: PublicKey,
        amount: bigint,
        recipient: string,
        payload: number[],
        payer: Keypair,
        cTokenProgramId: PublicKey,
        confirmOptions?: ConfirmOptions,
    ): Promise<TransactionSignature> {
        const addPriorityFee = ComputeBudgetProgram.setComputeUnitPrice({
            microLamports: 100000,
        });
        return await sendAndConfirmTransaction(
            connection,
            new Transaction()
                // .add(addPriorityFee)
                .add(
                    ...[
                        createApproveInstruction(
                            userAccount,
                            userTransferAuthority.publicKey,
                            payer.publicKey,
                            amount,
                            [],
                            tokenProgramId,
                        ),
                        CToken.bridgeInstruction(
                            cToken,
                            config,
                            tokenAccount,
                            userAccount,
                            userTransferAuthority.publicKey,
                            tokenMint,
                            tokenProgramId,
                            amount,
                            recipient,
                            payload,
                            cTokenProgramId,
                        ),
                    ],
                ),
            [payer, userTransferAuthority],
            confirmOptions,
        );
    }

    // payer is authority
    static async settle(
        connection: Connection,
        cToken: PublicKey,
        config: PublicKey,
        tokenAuthority: PublicKey,
        tokenAccount: PublicKey,
        userAccount: PublicKey,
        tokenMint: PublicKey,
        tokenProgramId: PublicKey,
        amount: bigint,
        payer: Keypair,
        cTokenProgramId: PublicKey,
        confirmOptions?: ConfirmOptions,
    ): Promise<TransactionSignature> {
        return await sendAndConfirmTransaction(
            connection,
            new Transaction().add(
                CToken.settleInstruction(
                    cToken,
                    config,
                    tokenAuthority,
                    tokenAccount,
                    userAccount,
                    payer.publicKey,
                    tokenMint,
                    tokenProgramId,
                    amount,
                    cTokenProgramId,
                ),
            ),
            [payer],
            confirmOptions,
        );
    }

    // payer is owner
    static async changeLimit(
        connection: Connection,
        cToken: PublicKey,
        config: PublicKey,
        max: bigint,
        min: bigint,
        payer: Keypair,
        cTokenProgramId: PublicKey,
        confirmOptions?: ConfirmOptions,
    ): Promise<TransactionSignature> {
        return await sendAndConfirmTransaction(
            connection,
            new Transaction().add(
                CToken.changeLimitInstruction(
                    cToken,
                    config,
                    payer.publicKey,
                    max,
                    min,
                    cTokenProgramId,
                ),
            ),
            [payer],
            confirmOptions,
        );
    }
}
