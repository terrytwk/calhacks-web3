import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Bounty } from "../target/types/bounty";

const OWNER = "9Noqgyni2vUrjFuKaEbd74vMouyeyrm5u1khiB9LabfV";
const RESEARCHER = "J8diPEqmv3VDQ4MBg8HCR4xJmBzs4uk57i1uWwnwdQtd"
const TRAINER = "29zmHPXBmgHEXwtGxHsijwgYkEiC5Pysg9gAVKa1QwDX"

describe("bounty", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Bounty as Program<Bounty>;

  it("Is initialized!", async () => {
    // Add your test here.
    //const tx = await program.methods.initBounty(new anchor.web3.PublicKey(""),new anchor.web3.PublicKey(""),"",new anchor.BN(0),new anchor.BN(0)).rpc();
    anchor.Wallet
    const tx = await program.methods.initEscrowAccount(
      new anchor.web3.PublicKey("9dxhHdZsHLnJVzqXE3hMfAEUw91YvmM8VzHtf36nWosq"),
      new anchor.BN(100),
      "word",
      new anchor.web3.PublicKey(OWNER),
    );
    program.methods.verifyModel
    console.log("Your transaction signature", tx);
  });
});
