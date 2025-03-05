import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorMovieReviewProgram } from "../target/types/anchor_movie_review_program";
import { expect } from "chai";
import * as crypto from 'crypto';

describe("anchor_movie_review_program", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.AnchorMovieReviewProgram as Program<AnchorMovieReviewProgram>;
  
  // 需确保不同test title值不一样，否则地址会有冲突 

  it("Successfully adds a movie review", async () => {
    const title = "Inception 1";
    const description = "A mind-bending masterpiece about dreams within dreams.";
    const rating = 5;

    let hexString = crypto.createHash('sha256').update(title,'utf-8').digest('hex');
    let titleHash = Uint8Array.from(Buffer.from(hexString,'hex'))

    const [movieReviewPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        titleHash, //Buffer.from(title),
        provider.wallet.publicKey.toBuffer(),
      ],
      program.programId
    );

    await program.methods
      .addMovieReview(title, description, rating)
      .accounts({
        movieReview: movieReviewPda,
        initializer: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    const account = await program.account.movieAccountState.fetch(movieReviewPda);
    
    expect(account.title).to.equal(title);
    expect(account.description).to.equal(description);
    expect(account.rating).to.equal(rating);
    expect(account.reviewer.toBase58()).to.equal(provider.wallet.publicKey.toBase58());
  });

  it("Fails with invalid rating", async () => {
    const title = "Inception 2";
    const description = "A mind-bending masterpiece";
    const rating = 6; // Invalid rating

    try {
      let hexString = crypto.createHash('sha256').update(title,'utf-8').digest('hex');
      let titleHash = Uint8Array.from(Buffer.from(hexString,'hex'))

      const [movieReviewPda] = anchor.web3.PublicKey.findProgramAddressSync(
        [
          titleHash,
          provider.wallet.publicKey.toBuffer(),
        ],
        program.programId
      );

    
      await program.methods
        .addMovieReview(title, description, rating)
        .accounts({
          movieReview: movieReviewPda,
          initializer: provider.wallet.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();
      assert.fail("Should have failed with invalid rating");
    } catch (err) {
      const errMsg = err.toString();
      console.log('errMsg',errMsg)
      expect(errMsg).to.include("Rating must be between 1 and 5");
    }
  });

  it("Fails with too long title", async () => {
    const title = "A".repeat(51); // Title longer than 50 characters
    const description = "A mind-bending masterpiece";
    const rating = 5;

    try {
      let hexString = crypto.createHash('sha256').update(title,'utf-8').digest('hex');
      let titleHash = Uint8Array.from(Buffer.from(hexString,'hex'))


      // 使用标题的哈希作为种子，避免超出长度限制
      const [movieReviewPda] = anchor.web3.PublicKey.findProgramAddressSync(
        [
          titleHash,
          provider.wallet.publicKey.toBuffer(),
        ],
        program.programId
      );

    
      await program.methods
        .addMovieReview(title, description, rating)
        .accounts({
          movieReview: movieReviewPda,
          initializer: provider.wallet.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();
      assert.fail("Should have failed with too long title");
    } catch (err) {
      const errMsg = err.toString();
      console.log('errMsg',errMsg)
      expect(errMsg).to.include("Title length should be less than 50 characters");
    }
  });

  it("Fails with too long description", async () => {
    const title = "Inception 3";
    const description = "A".repeat(301); // Description longer than 300 characters
    const rating = 5;

    let hexString = crypto.createHash('sha256').update(title,'utf-8').digest('hex');
    let titleHash = Uint8Array.from(Buffer.from(hexString,'hex'))

    const [movieReviewPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        titleHash,
        provider.wallet.publicKey.toBuffer(),
      ],
      program.programId
    );

    try {
      await program.methods
        .addMovieReview(title, description, rating)
        .accounts({
          movieReview: movieReviewPda,
          initializer: provider.wallet.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();
      assert.fail("Should have failed with too long description");
    } catch (err) {
      const errMsg = err.toString();
      expect(errMsg).to.include("Description length should be less than 300 characters");
    }
  });
});
