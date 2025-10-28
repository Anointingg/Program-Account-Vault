use anchor_lang::prelude::*;

declare_id!("2BjJGkn776vqG6U9We4K1A8A9Nx8kqowdPuwcasjM3hH");

#[program]
pub mod program_account_vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
