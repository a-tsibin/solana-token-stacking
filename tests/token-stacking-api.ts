import {BN} from "@project-serum/anchor";
import {
    SystemProgram,
    Keypair,
    SYSVAR_RENT_PUBKEY,
    PublicKey,
} from "@solana/web3.js";
import {TOKEN_PROGRAM_ID} from "@solana/spl-token";
import {Context} from "./ctx";
import {sha256} from "js-sha256";
import bs58 from "bs58";

export async function initialize(
    ctx: Context,
    roundDuration: number | BN,
    registrationPrice: number | BN
): Promise<void> {
    await ctx.program.methods
        .initialize(
            new BN(roundDuration),
            new BN(registrationPrice)
        )
        .accounts({
            platform: ctx.platform,
            platformAuthority: ctx.platformAuthority.publicKey,
            solVault: ctx.solVault,
            fctrTokenVault: await ctx.fctrVault(),
            fctrMint: ctx.fctrMint,
            bcdevMint: ctx.bcdevMint,
            rent: SYSVAR_RENT_PUBKEY,
            tokenProgram: TOKEN_PROGRAM_ID,
            systemProgram: SystemProgram.programId,
        })
        .signers([ctx.platformAuthority])
        .rpc();
}
