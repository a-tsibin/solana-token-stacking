import * as anchor from "@project-serum/anchor";
import {BN, Program} from "@project-serum/anchor";
import {Connection, Keypair, PublicKey} from "@solana/web3.js";
import {SolanaTokenStacking} from "../target/types/solana_token_stacking";
import {findATA, TokenAccount} from "./token";
import {airdrop, findPDA} from "./utils";

export class Context {
    connection: Connection;

    program: Program<SolanaTokenStacking>;

    payer: Keypair;

    platform: PublicKey;
    platformAuthority: Keypair;
    solVault: PublicKey;

    fctrMint: PublicKey;
    bcdevMint: PublicKey;

    users: Keypair[];

    constructor() {
        this.connection = new Connection("http://localhost:8899", "recent");
        this.program = anchor.workspace.SolanaTokenStacking;
        this.payer = new Keypair();

        this.platformAuthority = new Keypair();

        this.users = [];
        for (let i = 0; i < 5; i++) {
            this.users.push(new Keypair());
        }
    }

    async setup() {
        await airdrop(
            this,
            [
                this.platformAuthority.publicKey,
            ].concat(this.users.map((s) => s.publicKey)),
            100_000_000
        );

        this.platform = await findPDA(
            [Buffer.from("platform")],
            this.program.programId
        );
        this.solVault = await findPDA(
            [Buffer.from("sol_vault")],
            this.program.programId
        );
        this.fctrMint = await findPDA(
            [Buffer.from("fctr_mint")],
            this.program.programId
        );
        this.bcdevMint = await findPDA(
            [Buffer.from("bcdev_mint")],
            this.program.programId
        );
    }

    async user(userAuthority: PublicKey): Promise<PublicKey> {
        return await findPDA(
            [Buffer.from("user"), userAuthority.toBuffer()],
            this.program.programId
        );
    }

    async fctrVault(): Promise<TokenAccount> {
        const address = await findPDA(
            [Buffer.from("fctr_token_vault")],
            this.program.programId
        );
        return new TokenAccount(address, this.fctrMint);
    }

    async userFctrVault(authority: PublicKey): Promise<TokenAccount> {
        const address = await findPDA(
            [
                Buffer.from("fctr_vault"),
                authority.toBuffer(),
            ],
            this.program.programId
        );
        return new TokenAccount(address, this.fctrMint);
    }

    async userBcdevVault(authority: PublicKey): Promise<TokenAccount> {
        const address = await findPDA(
            [
                Buffer.from("bcdev_vault"),
                authority.toBuffer(),
            ],
            this.program.programId
        );
        return new TokenAccount(address, this.bcdevMint);
    }

    async receipt(authority: PublicKey): Promise<PublicKey> {
        return await findPDA(
            [
                Buffer.from("receipt"),
                authority.toBuffer(),
            ],
            this.program.programId
        );
    }

    async fctrATA(owner: PublicKey): Promise<TokenAccount> {
        return await findATA(this, owner, this.fctrMint);
    }

    async bcdevATA(owner: PublicKey): Promise<TokenAccount> {
        return await findATA(this, owner, this.bcdevMint);
    }

    async solVaultBalance() {
        return (await this.connection.getBalance(this.solVault)) - 890880;
    }
}
