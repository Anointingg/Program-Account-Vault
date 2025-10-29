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
        require_gt!(amount, 0u64, VaultError::InvalidAmount);

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

        if vault.owner == Pubkey::default() {
            vault.owner = ctx.accounts.signer.key();
            vault.bump = ctx.bumps.vault;
        } else {
            require_keys_eq!(
                vault.owner,
                ctx.accounts.signer.key(),
                VaultError::InvalidOwner
            );
        }
        vault.balance = vault.balance.saturating_add(amount);

        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        require_gte!(
            ctx.accounts.vault.balance,
            amount,
            VaultError::InsufficientBalance
        );

        let owner = ctx.accounts.owner.key();
        let owner_seed = [b"vault", owner.as_ref(), &[ctx.accounts.vault.bump]];

        transfer(
            CpiContext::new_with_signer(
                ctx.accounts.system_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.vault.to_account_info(),
                    to: ctx.accounts.owner.to_account_info(),
                },
                &[&owner_seed[..]],
            ),
            amount,
        )?;

        let vault = &mut ctx.accounts.vault;
        vault.balance = vault.balance.saturating_sub(amount);

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

pub const VAULT_SPACE: usize = 8 + 32 + 8 + 1 + 8; // discriminator + owner + balance + bump + padding

#[derive(Accounts)]

pub struct Deposit<'info> {
    #[account(mut)]
    signer: Signer<'info>,

    #[account(
        init, // I believe init_if_needed is a more appropriate approach here.
        payer = signer,
        space = VAULT_SPACE,
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
        bump = vault.bump,
        close = owner,
    )]
    vault: Account<'info, Vault>,

    system_program: Program<'info, System>,
}

#[error_code]
pub enum VaultError {
    #[msg("invalid amount")]
    InvalidAmount,

    #[msg("insufficient balance")]
    InsufficientBalance,

    #[msg("invalid owner")]
    InvalidOwner,
}
