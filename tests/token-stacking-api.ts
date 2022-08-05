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
            fctrMint: ctx.fctrMint,
            bcdevMint: ctx.bcdevMint,
            fctrTokenVault: await ctx.fctrVault(),
            rent: SYSVAR_RENT_PUBKEY,
            tokenProgram: TOKEN_PROGRAM_ID,
            systemProgram: SystemProgram.programId,
        })
        .signers([ctx.platformAuthority])
        .rpc();
}

export async function registerUser(
    ctx: Context,
    userAuthority: Keypair,
    grantProgram: boolean
): Promise<void> {
    await ctx.program.methods
        .registerUser(
            grantProgram
        )
        .accounts({
            platform: ctx.platform,
            fctrMint: ctx.fctrMint,
            bcdevMint: ctx.bcdevMint,
            fctrVault: await ctx.userFctrVault(userAuthority.publicKey),
            bcdevVault: await ctx.userBcdevVault(userAuthority.publicKey),
            user: await ctx.user(userAuthority.publicKey),
            receipt: await ctx.receipt(userAuthority.publicKey),
            authority: userAuthority.publicKey,
            solVault: ctx.solVault,
            systemProgram: SystemProgram.programId,
        })
        .signers([userAuthority])
        .rpc();
}

export async function addLiquidity(
    ctx: Context,
    amount: number,
): Promise<void> {
    await ctx.program.methods
        .addLiquidity(new BN(amount))
        .accounts({
            solVault: ctx.solVault,
            platform: ctx.platform,
            authority: ctx.platformAuthority.publicKey,
            systemProgram: SystemProgram.programId,
        })
        .signers([ctx.platformAuthority])
        .rpc();
}

export async function buyTokens(
    ctx: Context,
    fctrToBuy: number,
    userAuthority: Keypair,
): Promise<void> {
    await ctx.program.methods
        .buyTokens(new BN(fctrToBuy))
        .accounts({
            user: await ctx.user(userAuthority.publicKey),
            fctrVault: await ctx.userFctrVault(userAuthority.publicKey),
            solVault: ctx.solVault,
            platform: ctx.platform,
            fctrMint: ctx.fctrMint,
            authority: userAuthority.publicKey,
            systemProgram: SystemProgram.programId,
            tokenProgram: TOKEN_PROGRAM_ID
        })
        .signers([userAuthority])
        .rpc();
}

export async function startRound(
    ctx: Context,
    isFinal: boolean
): Promise<void> {
    await ctx.program.methods
        .startRound(isFinal)
        .accounts({
            platform: ctx.platform,
            authority: ctx.platformAuthority.publicKey,
            systemProgram: SystemProgram.programId
        })
        .signers([ctx.platformAuthority])
        .rpc();
}

export async function stake(
    ctx: Context,
    userAuthority: Keypair
): Promise<void> {
    await ctx.program.methods
        .stake()
        .accounts({
            user: await ctx.user(userAuthority.publicKey),
            receipt: await ctx.receipt(userAuthority.publicKey),
            fctrVault: await ctx.userFctrVault(userAuthority.publicKey),
            platform: ctx.platform,
            platformFctrTokenVault: await ctx.fctrVault(),
            authority: userAuthority.publicKey,
            systemProgram: SystemProgram.programId,
            tokenProgram: TOKEN_PROGRAM_ID
        })
        .signers([userAuthority])
        .rpc();
}