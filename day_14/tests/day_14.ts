import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day14 } from "../target/types/day_14";

describe("day_14", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Day14 as Program<Day14>;

  let myKeypair = anchor.web3.Keypair.generate();

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods
    .initialize()
    .accounts({
      signer1: program.provider.publicKey,
      signer2: myKeypair.publicKey,
    })
    .signers([myKeypair])
    .rpc();

  console.log("The signer1: ", program.provider.publicKey.toBase58());
  console.log("The signer2: ", myKeypair.publicKey.toBase58());
  });
});
