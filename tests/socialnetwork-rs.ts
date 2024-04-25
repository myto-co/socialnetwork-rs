import fs from 'fs'
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SocialnetworkRs } from "../target/types/socialnetwork_rs";
import { expect } from "chai";

describe("socialnetwork-rs", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  //const keep = anchor.web3.Keypair.fromSecretKey(fs.readFileSync(__dirname + '/TokBYGXh4hxdqy233c4diPEFH9grzH2mKMxVarZNLsu.json'))

  const program = anchor.workspace.SocialnetworkRs as Program<SocialnetworkRs>;
  const connection = anchor.getProvider().connection

  const airdrop = async (pubkey) => {
    const airdrop = await connection.requestAirdrop(pubkey, 2 * anchor.web3.LAMPORTS_PER_SOL)
    const blockhash = await connection.getLatestBlockhash()
    await connection.confirmTransaction({
      blockhash: blockhash.blockhash,
      lastValidBlockHeight: blockhash.lastValidBlockHeight,
      signature: airdrop,
    })
  }

  it("Is initialized!", async () => {
    let [statePda, _] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("tiktaalik-state")], program.programId)
    let [vaultPda, __] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("tiktaalik-vault")], program.programId)
    let signer = anchor.web3.Keypair.generate()
    
    const airdrop = await connection.requestAirdrop(signer.publicKey, 2 * anchor.web3.LAMPORTS_PER_SOL)
    const blockhash = await connection.getLatestBlockhash()
    await connection.confirmTransaction({
      blockhash: blockhash.blockhash,
      lastValidBlockHeight: blockhash.lastValidBlockHeight,
      signature: airdrop,
    })

    await program.methods.initialize().accounts({
      initializer: signer.publicKey,
      state: statePda,
      solVault: vaultPda,
      systemProgram: anchor.web3.SystemProgram.programId
    }).signers([signer]).rpc();
  });

  describe("Creating a new user", async () => {
    it("Fails if a username is taken", async () => {
    })
    it("Allows for changing the profile information", async () => {
    
    })

    it("Fails if a username is taken", async () => {
    
    })

    it("Allows for changing to an empty username", async () => {
    
    })

    it("Fails if changing to a taken username", async () => {
    
    })

    it("Fails if a username is taken", async () => {
    
    })

    it("Doesn't allow other accounts to change the user info", async () => {

    })
  })

  it("Creates post", async () => {
    let creator = anchor.web3.Keypair.generate();

    await program.methods.createPost("Solana will defeat Ethereum").accounts({
      authority: creator.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId
    }).signers([creator]).rpc()
    let accs = await program.account.post.all()
  })

  describe("Seeding", async () => {

  })
});
