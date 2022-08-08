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
    lamports: number,
    userAuthority: Keypair,
): Promise<void> {
    await ctx.program.methods
        .buyTokens(new BN(lamports))
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

export async function sellFctrTokens(
    ctx: Context,
    userAuthority: Keypair,
): Promise<void> {
    await ctx.program.methods
        .sellFctrTokens()
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

export async function sellBcdevTokens(
    ctx: Context,
    amount: number,
    userAuthority: Keypair,
): Promise<void> {
    await ctx.program.methods
        .sellBcdevTokens(new BN(amount))
        .accounts({
            user: await ctx.user(userAuthority.publicKey),
            bcdevVault: await ctx.userBcdevVault(userAuthority.publicKey),
            solVault: ctx.solVault,
            platform: ctx.platform,
            bcdevMint: ctx.bcdevMint,
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

export async function unstake(
    ctx: Context,
    userAuthority: Keypair
): Promise<void> {
    const remainingAccounts = [];
    const receipt = await ctx.program.account.receipt.fetch(
        await ctx.receipt(userAuthority.publicKey)
    );
    const grantors = receipt.grantors;
    for (let i = 1; i <= grantors.length; i++) {
        remainingAccounts.push(
            {
                pubkey: ctx.users[i].publicKey,
                isSigner: false,
                isWritable: true,
            },
            {
                pubkey: await ctx.userFctrVault(ctx.users[i].publicKey),//await ctx.fctrATA(ctx.users[i].publicKey),
                isSigner: false,
                isWritable: true,
            },
            {
                pubkey: await ctx.userBcdevVault(ctx.users[i].publicKey),//await ctx.bcdevATA(ctx.users[i].publicKey),
                isSigner: false,
                isWritable: true,
            }
        );
    }

    await ctx.program.methods
        .unstake()
        .accounts({
            receipt: await ctx.receipt(userAuthority.publicKey),
            user: await ctx.user(userAuthority.publicKey),
            authority: userAuthority.publicKey,
            fctrVault: await ctx.userFctrVault(userAuthority.publicKey),
            bcdevVault: await ctx.userBcdevVault(userAuthority.publicKey),
            platform: ctx.platform,
            platformFctrTokenVault: await ctx.fctrVault(),
            bcdevMint: ctx.bcdevMint,
            systemProgram: SystemProgram.programId,
            tokenProgram: TOKEN_PROGRAM_ID
        })
        .remainingAccounts(remainingAccounts)
        .signers([userAuthority])
        .rpc();
}

export async function grantTokens(
    ctx: Context,
    amount: number,
    confidantUser: PublicKey,
    userAuthority: Keypair
): Promise<void> {
    await ctx.program.methods
        .grantTokens(new BN(amount))
        .accounts({
            receipt: await ctx.receipt(userAuthority.publicKey),
            user: await ctx.user(userAuthority.publicKey),
            fctrVault: await ctx.userFctrVault(userAuthority.publicKey),
            authority: userAuthority.publicKey,
            confidantUser: await ctx.user(confidantUser),
            confidantReceipt: await ctx.receipt(confidantUser),
            confidantAuthority: confidantUser,
            platform: ctx.platform,
            platformFctrTokenVault: await ctx.fctrVault(),
            systemProgram: SystemProgram.programId,
            tokenProgram: TOKEN_PROGRAM_ID
        })
        .signers([userAuthority])
        .rpc();
}

export async function withdraw(
    ctx: Context,
    platformAuthority: Keypair
): Promise<void> {
    await ctx.program.methods
        .withdraw()
        .accounts({
            solVault: ctx.solVault,
            authority: platformAuthority.publicKey,
            fctrTokenVault: await ctx.fctrVault(),
            platform: ctx.platform,
            systemProgram: SystemProgram.programId
        })
        .signers([platformAuthority])
        .rpc();
}