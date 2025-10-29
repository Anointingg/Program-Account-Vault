use anchor_lang::prelude::*;

declare_id!("2BjJGkn776vqG6U9We4K1A8A9Nx8kqowdPuwcasjM3hH");

#[program]
pub mod program_account_vault {
    use anchor_lang::system_program::{transfer, Transfer};

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        // check if deposit has enough for rent too
        require_gt!(
            amount,
            Rent::get()?.minimum_balance(0),
            VaultError::InvalidAmount
        );

        transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.signer.to_account_info(),
                    to: ctx.accounts.vault.to_account_info(),
                },
            ),
            amount,
        )?;

        let vault = &mut ctx.accounts.vault;
        vault.owner = ctx.accounts.signer.key();
        vault.balance = vault.balance.saturating_add(amount);
        vault.bump = ctx.bumps.vault;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[account]
pub struct Vault {
    owner: Pubkey,
    balance: u64,
    bump: u8,
}

#[derive(Accounts)]

pub struct Deposit<'info> {
    #[account(mut)]
    signer: Signer<'info>,

    #[account(
        init, // I believe init_if_needed is a more appropriate approach here.
        payer = signer,
        space = 8 + 6 + 32 + 6,
        seeds = [b"vault", signer.key().as_ref()],
        bump
    )]
    vault: Account<'info, Vault>,

    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    owner: Signer<'info>,

    #[account(
        mut,
        has_one = owner,
        seeds = [b"vault", owner.key().as_ref()],
        bump = vault.bump
    )]
    vault: Account<'info, Vault>,

    system_program: Program<'info, System>,
}

#[error_code]
pub enum VaultError {
    #[msg("invalid amount")]
    InvalidAmount,
}
