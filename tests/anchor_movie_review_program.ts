import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorMovieReviewProgram } from "../target/types/anchor_movie_review_program";
import { expect } from "chai";

describe("anchor_movie_review_program", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.AnchorMovieReviewProgram as Program<AnchorMovieReviewProgram>;

  it("Movie review is added!", async () => {
    const movieReview = anchor.web3.Keypair.generate();

    const title = "Inception";
    const description = "A mind-bending masterpiece about dreams within dreams.";
    const rating = 5;

    await program.methods
      .addMovieReview(title, description, rating)
      .accounts({
        movieReview: movieReview.publicKey,
        initializer: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([movieReview])
      .rpc();

    const account = await program.account.movieAccountState.fetch(movieReview.publicKey);
    
    expect(account.title).to.equal(title);
    expect(account.description).to.equal(description);
    expect(account.rating).to.equal(rating);
    expect(account.reviewer.toBase58()).to.equal(provider.wallet.publicKey.toBase58());
  });
});
