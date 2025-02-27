import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day12 } from "../target/types/day_12";

describe("day_12", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Day12 as Program<Day12>;

  // const StakeHistory_PubKey = new anchor.web3.PublicKey(
  //   "SysvarStakeHistory1111111111111111111111111"
  // )

  const StakeHistory_PubKey = anchor.web3.SYSVAR_STAKE_HISTORY_PUBKEY;
  const recentBlockhashes = anchor.web3.SYSVAR_RECENT_BLOCKHASHES_PUBKEY;
  const instructionSysvar = anchor.web3.SYSVAR_INSTRUCTIONS_PUBKEY;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods
      .initialize(3)
      .accounts({
        stakeHistoryAccount: StakeHistory_PubKey,
        recentBlockhashes: recentBlockhashes,
        instructionSysvar: instructionSysvar,
      })
      .rpc();

      console.log("Your transaction signature", tx);
    
  });


});
