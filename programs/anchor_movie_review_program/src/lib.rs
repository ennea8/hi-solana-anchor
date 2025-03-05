use anchor_lang::prelude::*;

declare_id!("FEWynf4QcTnscJotG6gxUmnodU47bBj7MkEd2tx8rkdY");

#[error_code]
pub enum MovieReviewError {
    #[msg("Rating must be between 1 and 5")]
    RatingNotInRange,
    #[msg("Title length should be less than 50 characters")]
    TitleTooLong,
    #[msg("Description length should be less than 300 characters")]
    DescriptionTooLong,
}

#[program]
pub mod anchor_movie_review_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn add_movie_review(
        ctx: Context<AddMovieReview>,
        title: String,
        description: String,
        rating: u8,
    ) -> Result<()> {
        require!(rating >= 1 && rating <= 5, MovieReviewError::RatingNotInRange);
        require!(title.len() <= 50, MovieReviewError::TitleTooLong);
        require!(description.len() <= 300, MovieReviewError::DescriptionTooLong);

        msg!("Movie Review Account Created");
        msg!("Title: {}", title);
        msg!("Description: {}", description);
        msg!("Rating: {}", rating);

        let movie_review = &mut ctx.accounts.movie_review;
        movie_review.reviewer = ctx.accounts.initializer.key();
        movie_review.title = title;
        movie_review.rating = rating;
        movie_review.description = description;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
#[instruction(title: String, description: String, rating: u8)]
pub struct AddMovieReview<'info> {
    #[account(
        init,
        payer = initializer,
        space = 8 + MovieAccountState::SPACE,  // 8 bytes for account discriminator
        seeds = [
            &anchor_lang::solana_program::hash::hash(title.as_bytes()).to_bytes()[..32],  // 使用 title 的哈希值
            initializer.key().as_ref(),
        ],
        bump
    )]
    pub movie_review: Account<'info, MovieAccountState>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct MovieAccountState {
    pub reviewer: Pubkey,    // 32
    pub rating: u8,          // 1
    pub title: String,       // 4 + 50
    pub description: String, // 4 + 300
}

impl MovieAccountState {
    pub const SPACE: usize = 32 + // reviewer
        1 +  // rating
        4 + 50 + // title (4 bytes for string length + max 50 chars)
        4 + 300; // description (4 bytes for string length + max 300 chars)
}
