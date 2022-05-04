import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { LilIntrovert } from "../target/types/lil_introvert";
import { Extrovert } from "../target/types/extrovert";

describe("lil-introvert", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const introvertProgram = anchor.workspace.LilIntrovert as Program<LilIntrovert>;
  const extrovertProgram = anchor.workspace.Extrovert as Program<Extrovert>;

  it("You can talk to me", async () => {
    const tx = await introvertProgram.methods.initialize().accounts({
      instructionSysvarAccount: anchor.web3.SYSVAR_INSTRUCTIONS_PUBKEY
    }).rpc();

    console.log("Your transaction signature", tx);
  });

  it("But CPIs Can't", async () => {
    const cpiTx = await extrovertProgram.methods.letDoSomeBully().accounts({
      instructionSysvarAccount: anchor.web3.SYSVAR_INSTRUCTIONS_PUBKEY,
      introvertProgram: introvertProgram.programId
    }).rpc();

    console.log("Your transaction signature", cpiTx);

  })
});
