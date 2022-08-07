import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { TestProject } from "../target/types/test_project";

const timeIncreasePerBetInSeconds = 60;
const minimalTimeIncreasePerBetInSeconds = 24;
const auctionDurationInSeconds = 15 * 60;
const maxParticipationAmount = 100000;
const minPotSize = 100000;

describe("test-project", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.TestProject as Program<TestProject>;
  let pdaVaultPublicKey: anchor.web3.PublicKey, vaultBump: number;
  let pdaStatePublicKey: anchor.web3.PublicKey, stateBump: number;

  beforeEach(async () => {
    [pdaVaultPublicKey, vaultBump] =
      await anchor.web3.PublicKey.findProgramAddress(
        [Buffer.from("vault")],
        program.programId,
      );

    [pdaStatePublicKey, stateBump] =
      await anchor.web3.PublicKey.findProgramAddress(
        [Buffer.from("state")],
        program.programId,
      );
  });

  it("Is initialized!", async () => {
    const tx = await program.methods
      .initialize(
        new anchor.BN(timeIncreasePerBetInSeconds),
        new anchor.BN(minimalTimeIncreasePerBetInSeconds),
        new anchor.BN(auctionDurationInSeconds),
        new anchor.BN(maxParticipationAmount),
        new anchor.BN(minPotSize),
      )
      .accounts({
        vault: pdaVaultPublicKey,
        auctionInstance: pdaStatePublicKey,
        user: provider.publicKey,
      })
      .rpc();

    // const programState = await program.account.state.fetch(
    //   program.provider.publicKey,
    // );

    // const vault = await program.account.vault.fetch(
    //   programState.vaultPublicKey,
    // );

    // const userAccountInfo = await provider.connection.getBalance(
    //   provider.publicKey,
    // );

    // let txFund = new anchor.web3.Transaction();

    // txFund.add(
    //   anchor.web3.SystemProgram.transfer({
    //     fromPubkey: newUser.publicKey,
    //     toPubkey: provider.wallet.publicKey,
    //     lamports: 1 * anchor.web3.LAMPORTS_PER_SOL,
    //   }),
    // );

    // await anchor.web3.sendAndConfirmTransaction(provider.connection, txFund, [
    //   newUser,
    // ]);
  });

  it("it is funded", async () => {
    const newUser = anchor.web3.Keypair.generate();

    let airdropSig = await provider.connection.requestAirdrop(
      newUser.publicKey,
      5 * anchor.web3.LAMPORTS_PER_SOL,
    );

    const latestBlockHash = await provider.connection.getLatestBlockhash();

    await provider.connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: airdropSig,
    });

    const vaultBalanceBefore = await provider.connection.getBalance(
      pdaVaultPublicKey,
    );

    const userBalanceBefore = await provider.connection.getBalance(
      newUser.publicKey,
    );

    console.log({
      vaultBalanceBefore,
      userBalanceBefore,
    });

    const tx = await program.methods
      .fundVault(new anchor.BN(2 * anchor.web3.LAMPORTS_PER_SOL))
      .accounts({
        payer: newUser.publicKey,
        vault: pdaVaultPublicKey,
      })
      .signers([newUser])
      .rpc();

    console.log("Your transaction signature", tx);

    const vaultBalanceAfter = await provider.connection.getBalance(
      pdaVaultPublicKey,
    );
    const userBalanceAfter = await provider.connection.getBalance(
      newUser.publicKey,
    );

    console.log({
      vaultBalanceAfter,
      userBalanceAfter,
    });
  });
});
