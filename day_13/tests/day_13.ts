import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day13 } from "../target/types/day_13";

describe("day_13", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Day13 as Program<Day13>;

  it("Is initialized!", async () => {
    // Add your test here.

    const listenMyEvent = program.addEventListener('myEvent', (event, slot) => {
      console.log(`slot ${slot} event value ${event.value}`);
    });

    const listenerMySecondEvent = program.addEventListener('mySecondEvent', (event, slot) => {
      console.log(`slot ${slot} event value ${event.value}`);
    })

    await program.methods.initialize();

    await new Promise((resolve) => setTimeout(resolve, 5000));

    program.removeEventListener(listenMyEvent);
    program.removeEventListener(listenerMySecondEvent);
  });
});
