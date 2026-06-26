use anchor_lang::prelude::*;

use crate::{VAULT_SEED, VAULT_STATE_SEED, VaultState};

#[derive(Accounts)]
pub struct Close<'info> {
    #[account(mut)]
    pub user: Signer<'info>, // To pay for instruction call
    #[account(mut, seeds=[VAULT_SEED, user.key().as_ref()], bump = vault_state.vault_bump)]
    pub vault: SystemAccount<'info>, // To pull deposited lamports and rent
    #[account(mut, close = user, seeds = [VAULT_STATE_SEED, user.key().as_ref()], bump = vault_state.bump)]
    pub vault_state: Account<'info, VaultState>, // To close vault_state data (i.e. bumps)
    pub system_account: Program<'info, System> // To transfer vault lamports to user
}

pub fn close_vault(ctx: Context<Close>) -> Result<()> {
    msg!("Closing vault & withdrawing lamports...");
    let lamports = ctx.accounts.vault.lamports();

    let seeds = &[
        VAULT_SEED,
        ctx.accounts.user.key.as_ref(),
        &[ctx.accounts.vault_state.vault_bump],
    ];

    let signer = &[&seeds[..]];


    let cpi_accounts = anchor_lang::system_program::Transfer {
        from: ctx.accounts.vault.to_account_info(),
        to: ctx.accounts.user.to_account_info()
    };

    let cpi_ctx = CpiContext::new_with_signer(ctx.accounts.system_account.key(), cpi_accounts, signer);

    anchor_lang::system_program::transfer(cpi_ctx, lamports)?;
    msg!("Vault closed! Amount withdrawn: {}", lamports);

    Ok(())
}