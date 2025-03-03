use anchor_lang::prelude::*;

declare_id!("HUUAj59uBGfY74wXDbEtf3kvp1X4kPhVdVwEyc5tMdMg");

#[program]
pub mod hi_solana_anchor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
