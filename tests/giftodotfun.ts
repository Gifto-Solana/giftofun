import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Giftodotfun } from "../target/types/giftodotfun";
import { PublicKey, SystemProgram, LAMPORTS_PER_SOL } from "@solana/web3.js";
import { assert } from "chai";

describe("giftodotfun", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Giftodotfun as Program<Giftodotfun>;
  const signer = provider.wallet;

  it("Creates a gift and deposits funds into escrow", async () => {
    const amount = new anchor.BN(0.1 * LAMPORTS_PER_SOL);

    const [escrowPda, bump] = PublicKey.findProgramAddressSync(
      [Buffer.from("escrow"), signer.publicKey.toBuffer()],
      program.programId
    );

    console.log("Escrow PDA:", escrowPda.toString());

    const balanceBefore = await provider.connection.getBalance(signer.publicKey);
    console.log("Signer balance before:", balanceBefore / LAMPORTS_PER_SOL, "SOL");

    const tx = await program.methods
      .createGiftLink(amount)
      .accounts({
        signer: signer.publicKey,
        escrowpdaaccount: escrowPda,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    console.log("Transaction signature:", tx);

    await provider.connection.confirmTransaction(tx);

    const escrowBalance = await provider.connection.getBalance(escrowPda);
    console.log("Escrow balance:", escrowBalance / LAMPORTS_PER_SOL, "SOL");

    // Check that escrow has AT LEAST the deposited amount
    // (will be more due to rent-exempt minimum)
    assert.isAtLeast(
      escrowBalance,
      amount.toNumber(),
      "Escrow should have at least the deposited amount plus rent"
    );

    const balanceAfter = await provider.connection.getBalance(signer.publicKey);
    console.log("Signer balance after:", balanceAfter / LAMPORTS_PER_SOL, "SOL");

    console.log("✅ Gift created successfully!");
    console.log(`📦 Rent-exempt reserve: ${(escrowBalance - amount.toNumber()) / LAMPORTS_PER_SOL} SOL`);
  });
});