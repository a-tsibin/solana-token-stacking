import { Context } from "./ctx";
import {
  PublicKey,
  sendAndConfirmTransaction,
  SystemProgram,
  Transaction,
} from "@solana/web3.js";

export async function airdrop(
  ctx: Context,
  addresses: PublicKey[],
  amount: number
): Promise<void> {
  await ctx.connection.confirmTransaction(
    await ctx.connection.requestAirdrop(
      ctx.payer.publicKey,
      amount * (addresses.length + 1)
    )
  );

  const tx = new Transaction();

  for (let i = 0; i < addresses.length; i++) {
    tx.add(
      SystemProgram.transfer({
        fromPubkey: ctx.payer.publicKey,
        lamports: amount,
        toPubkey: addresses[i],
      })
    );
  }

  await sendAndConfirmTransaction(ctx.connection, tx, [ctx.payer]);
}

export async function findPDA(
  seeds: (Buffer | Uint8Array)[],
  programId: PublicKey
): Promise<PublicKey> {
  return (await PublicKey.findProgramAddress(seeds, programId))[0];
}

export function sleep(ms: number) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}
