use anchor_lang::prelude::*;

declare_id!("9uyaujBebdYM9gNdnreC4s8ivjT68Hy7HhDdAyWidVUt");

#[program]
pub mod anchor_counter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        
        // Account 实现了DerefMut，可直接访问内部的account
        counter.count = 0;
        msg!("Counter Account Created");
        msg!("Current Count: { }", counter.count);
        Ok(())
    }

    pub fn increment(ctx: Context<Update>, increment_by: u64) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        // counter.count += increment_by;
        msg!("Previous counter: {}", counter.count);
        counter.count = counter.count.checked_add(increment_by).unwrap();
        msg!("Current Count: { }", counter.count);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = user,
        space = DISCRIMINATOR + Counter::INIT_SPACE
    )]
    pub counter: Account<'info, Counter>, 
    #[account(mut)]
    pub user: Signer<'info>, // payer for the transaction fee
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub counter: Account<'info, Counter>,
    pub user: Signer<'info>,
}

#[account]
#[derive(InitSpace)]
pub struct Counter {
    pub count: u64,
}

const DISCRIMINATOR: usize = 8;
