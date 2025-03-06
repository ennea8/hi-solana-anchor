use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{mint_to, Mint, MintTo, Token, TokenAccount};

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
    pub fn initialize_token_mint(_ctx: Context<InitializeMint>) -> Result<()> {
        msg!("Token mint initialized");
        Ok(())
    }

    pub fn add_movie_review(
        ctx: Context<AddMovieReview>,
        title: String,
        description: String,
        rating: u8,
    ) -> Result<()> {
        require!(
            rating >= 1 && rating <= 5,
            MovieReviewError::RatingNotInRange
        );
        require!(title.len() <= 50, MovieReviewError::TitleTooLong);
        require!(
            description.len() <= 300,
            MovieReviewError::DescriptionTooLong
        );

        msg!("Movie Review Account Created");
        msg!("Title: {}", title);
        msg!("Description: {}", description);
        msg!("Rating: {}", rating);

        let movie_review = &mut ctx.accounts.movie_review;
        movie_review.reviewer = ctx.accounts.initializer.key();
        movie_review.title = title;
        movie_review.rating = rating;
        movie_review.description = description;

        // 发起 CPI
        mint_to(
            // 构造 CpiContext
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                MintTo {
                    authority: ctx.accounts.initializer.to_account_info(),
                    to: ctx.accounts.token_account.to_account_info(),
                    mint: ctx.accounts.mint.to_account_info(),
                },
                &[&["mint".as_bytes(), &[ctx.bumps.mint]]], // ？？？
            ),
            10 * 10u64.pow(6),
        )?;

        msg!("Minted tokens");

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
    pub token_program: Program<'info, Token>, // 官方提供的token程序，不可变
    #[account(
        seeds = ["mint".as_bytes()],
        bump,
        mut
    )]
    pub mint: Account<'info, Mint>, // 存储supply 和 decimals数据
    #[account(
        init_if_needed,
        payer = initializer,
        associated_token::mint = mint,
        associated_token::authority = initializer
    )]
    pub token_account: Account<'info, TokenAccount>, // 包含owner，amount数据
    pub associated_token_program: Program<'info, AssociatedToken>, //required: 在 associated_token 上使用 token_account 约束。 ？？？
}

#[derive(Accounts)]
pub struct InitializeMint<'info> {
    #[account(
        init,
        seeds = ["mint".as_bytes()],
        bump,
        payer = user,
        mint::decimals = 6,
        mint::authority = user,
    )]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
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
