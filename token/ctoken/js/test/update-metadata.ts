import * as fs from 'fs';
import {
    TokenStandard,
    createV1,
    findMetadataPda,
    mplTokenMetadata,
} from '@metaplex-foundation/mpl-token-metadata';
import {mplToolbox} from '@metaplex-foundation/mpl-toolbox';
import {
    keypairIdentity,
    percentAmount,
    publicKey,
    sol,
} from '@metaplex-foundation/umi';
import {createUmi} from '@metaplex-foundation/umi-bundle-defaults';
import {base58} from '@metaplex-foundation/umi/serializers';

async function main() {
    const umi = createUmi(`${process.env.SOLANA_RPC_URL}`)
        .use(mplTokenMetadata())
        .use(mplToolbox());

    const secret = JSON.parse(
        fs.readFileSync(`${process.env.PRIVATE_KEY_PATH}`).toString(),
    ) as number[];
    const secretKey = Uint8Array.from(secret);
    const keypair = umi.eddsa.createKeypairFromSecretKey(secretKey);

    umi.use(keypairIdentity(keypair));
    const tokenMetadata = {
        name: 'Solana Crosschain IOTX',
        symbol: 'CIOTX',
        uri: 'https://nft.iotex.io/tokens/solana/ciotx/metadata.json',
    };

    const mint = publicKey(`xgf3DoXeqCRVJ9hzU2vbTjfbs6j5BpCHWSmVGBV7Ryx`);
    const metadataAccountAddress = findMetadataPda(umi, {
        mint: mint,
    });
    console.log(
        `Metadata account for ${mint} token is: ${metadataAccountAddress}`,
    );

    const tx = await createV1(umi, {
        mint,
        authority: umi.identity,
        payer: umi.identity,
        updateAuthority: umi.identity,
        name: tokenMetadata.name,
        symbol: tokenMetadata.symbol,
        uri: tokenMetadata.uri,
        sellerFeeBasisPoints: percentAmount(0),
        tokenStandard: TokenStandard.Fungible,
    }).sendAndConfirm(umi);

    const txSig = base58.deserialize(tx.signature);
    console.log(`Update metadata for ${mint} token tx: ${txSig}`);
}

main();
