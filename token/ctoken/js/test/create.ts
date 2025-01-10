import * as fs from 'fs';
import type {TransactionInstruction} from '@solana/web3.js';
import {
    PublicKey,
    Keypair,
    Connection,
    clusterApiUrl,
    sendAndConfirmTransaction,
    Transaction,
    SystemProgram,
} from '@solana/web3.js';
import {
    getAssociatedTokenAddress,
    createAccount,
    TOKEN_PROGRAM_ID,
    TOKEN_2022_PROGRAM_ID,
    AuthorityType,
    createAssociatedTokenAccountInstruction,
    createMintToInstruction,
    MINT_SIZE,
    getMinimumBalanceForRentExemptMint,
    createInitializeMint2Instruction,
    createSetAuthorityInstruction,
} from '@solana/spl-token';
import type {DataV2} from '@metaplex-foundation/mpl-token-metadata';
import {
    createMetadataAccountV3,
    mplTokenMetadata,
} from '@metaplex-foundation/mpl-token-metadata';
import {mplToolbox} from '@metaplex-foundation/mpl-toolbox';
import {
    fromWeb3JsKeypair,
    fromWeb3JsPublicKey,
    toWeb3JsInstruction,
} from '@metaplex-foundation/umi-web3js-adapters';
import {
    createSignerFromKeypair,
    none,
    signerIdentity,
} from '@metaplex-foundation/umi';
import {createUmi} from '@metaplex-foundation/umi-bundle-defaults';
import {CToken} from '../src';

function addMetadataInstructions(
    payer: Keypair,
    tokenMint: PublicKey,
    name: string,
    symbol: string,
    uri: string,
): TransactionInstruction[] {
    const umi = createUmi(`${process.env.SOLANA_RPC_URL}`)
        .use(mplTokenMetadata())
        .use(mplToolbox());
    const signer = createSignerFromKeypair(umi, fromWeb3JsKeypair(payer));
    umi.use(signerIdentity(signer, true));

    const mint = fromWeb3JsPublicKey(tokenMint);

    const onChainData = {
        name: name,
        symbol: symbol,
        uri: uri,
        sellerFeeBasisPoints: 0,
        creators: none(),
        collection: none(),
        uses: none(),
    } as DataV2;
    const data = {
        isMutable: false,
        collectionDetails: null,
        data: onChainData,
    };

    const ixs = createMetadataAccountV3(umi, {
        mint,
        mintAuthority: umi.identity,
        payer: umi.identity,
        updateAuthority: umi.identity,
        ...data,
    }).getInstructions();

    return ixs.map(i => toWeb3JsInstruction(i));
}

async function main() {
    // const rpc = clusterApiUrl('devnet');
    const rpc = `${process.env.SOLANA_RPC_URL}`;

    const secret = JSON.parse(
        fs.readFileSync(`${process.env.PRIVATE_KEY_PATH}`).toString(),
    ) as number[];
    const secretKey = Uint8Array.from(secret);
    const payer = Keypair.fromSecretKey(secretKey);
    const connection = new Connection(rpc, 'confirmed');

    const cTokenProgramId = new PublicKey(`${process.env.C_TOKEN_PROGRAM_ID}`);
    // TODO hardcode for now
    const tokenProgramId = TOKEN_PROGRAM_ID;
    const cTokenAccount = Keypair.generate();
    const [authority, _bumpSeed] = PublicKey.findProgramAddressSync(
        [cTokenAccount.publicKey.toBuffer()],
        cTokenProgramId,
    );
    const config = new PublicKey(`${process.env.CONFIG}`);

    const destination = 0;
    let tokenMint;
    let tokenAccount;

    if (destination === 0) {
        // base chain is not solana
        console.log('creating token mint');
        const tokenKeypair = Keypair.generate();
        tokenMint = tokenKeypair.publicKey;
        const payerATA = await getAssociatedTokenAddress(
            tokenMint,
            payer.publicKey,
        );

        const lamports = await getMinimumBalanceForRentExemptMint(connection);
        await sendAndConfirmTransaction(
            connection,
            new Transaction().add(
                // create mint
                SystemProgram.createAccount({
                    fromPubkey: payer.publicKey,
                    newAccountPubkey: tokenMint,
                    space: MINT_SIZE,
                    lamports,
                    programId: tokenProgramId,
                }),
                createInitializeMint2Instruction(
                    tokenMint,
                    9,
                    payer.publicKey,
                    null,
                    tokenProgramId,
                ),

                // mintTo TODO: only fix
                createAssociatedTokenAccountInstruction(
                    payer.publicKey,
                    payerATA,
                    payer.publicKey,
                    tokenMint,
                ),
                createMintToInstruction(
                    tokenMint,
                    payerATA,
                    payer.publicKey,
                    9354071822509n,
                ),

                // add metadata
                ...addMetadataInstructions(
                    payer,
                    tokenMint,
                    'Crosschain IOTX',
                    'CIOTX',
                    'https://nft.iotex.io/tokens/solana/ciotx/metadata.json',
                ),

                // change authority
                createSetAuthorityInstruction(
                    tokenMint,
                    payer.publicKey,
                    AuthorityType.MintTokens,
                    authority,
                ),
            ),
            [payer, tokenKeypair],
        );

        tokenAccount = tokenMint;
    } else {
        // base chain is solana
        tokenMint = new PublicKey(`${process.env.TOKEN_MINT}`);

        console.log(`creating token account`);
        tokenAccount = await createAccount(
            connection,
            payer,
            tokenMint,
            authority,
            Keypair.generate(),
            undefined,
            tokenProgramId,
        );
    }

    console.log(`cTokenAccount: ${JSON.stringify(cTokenAccount.secretKey)}`);
    console.log(`authority: ${authority.toBase58()}`);
    console.log(`tokenMint: ${tokenMint.toBase58()}`);
    console.log(`tokenAccount: ${tokenAccount.toBase58()}`);

    console.log(`creating cToken account`);
    await CToken.createCToken(
        connection,
        cTokenAccount,
        authority,
        tokenMint,
        tokenAccount,
        tokenProgramId,
        payer, // owner
        config,
        destination,
        50000000000000000n,
        20000000000,
        cTokenProgramId,
        payer,
    );
    console.log(
        `Solana cToken for ${tokenMint} token with ${authority} authority and ${destination} destination is ${cTokenAccount.publicKey}`,
    );
}

main();
