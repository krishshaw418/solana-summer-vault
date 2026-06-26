use anchor_lang::prelude::*;

use crate::{VAULT_SEED, VAULT_STATE_SEED, VaultState};

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub user: Signer<'info>, // The user signing/paying for instruction call
    #[account(mut, seeds=[VAULT_SEED, user.key().as_ref()], bump = vault_state.vault_bump)]
    pub vault: SystemAccount<'info>, // The vault account that holds the SOL of the user
    #[account(seeds=[VAULT_STATE_SEED, user.key().as_ref()], bump = vault_state.bump)]
    pub vault_state: Account<'info, VaultState>, // To get the bump for vault PDA derivation
    pub system_program: Program<'info, System> // To transfer sol to owner of vault
}

pub fn withdraw_lamports(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    msg!("Withdrawing lamports to vault");

    let seeds = &[
        VAULT_SEED,
        ctx.accounts.user.key.as_ref(),
        &[ctx.accounts.vault_state.vault_bump]
    ];

    let signer = &[&seeds[..]];

    let cpi_accounts = anchor_lang::system_program::Transfer {
        from: ctx.accounts.vault.to_account_info(),
        to: ctx.accounts.user.to_account_info()
    };

    let cpi_ctx = CpiContext::new_with_signer(ctx.accounts.system_program.key(), cpi_accounts, signer);

    anchor_lang::system_program::transfer(cpi_ctx, amount)?;
    msg!("Withdrawn {} lamports from vault!", amount);

    Ok(())
}