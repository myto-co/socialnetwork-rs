// Migrations are an early feature. Currently, they're nothing more than this
// single deploy script that's invoked from the CLI, injecting a provider
// configured from the workspace's Anchor.toml.

const anchor = require("@coral-xyz/anchor");
import { Program } from "@coral-xyz/anchor";
import { SocialnetworkRs } from "../target/types/socialnetwork_rs";

module.exports = async function (provider) {
  // Configure client to use the provider.
  anchor.setProvider(provider);

  // Add your deploy script here.
  const program = anchor.workspace.SocialnetworkRs as Program<SocialnetworkRs>;
  const connection = anchor.getProvider().connection

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
};
