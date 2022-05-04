use anchor_lang::{
    prelude::*,
    solana_program::{
        sysvar,
        sysvar::{instructions::get_instruction_relative}
    },
};

declare_id!("7HHAYoTw39hNTeJut54zKTm6gizw2fByz5gbNUcBt78c");

#[program]
pub mod lil_introvert {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let instruction_sysvar_account = &ctx.accounts.instruction_sysvar_account.to_account_info();
        let current_ix_progrm_id = get_instruction_relative(0, &instruction_sysvar_account)?.program_id;

        // No one can talk me via CPI
        require_keys_eq!(
            current_ix_progrm_id,
            crate::ID,
            RestrictError::OoopsIMIntrovert
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    /// CHECK: account constraints checked in account trait
    #[account(address = sysvar::instructions::ID)]
    instruction_sysvar_account: AccountInfo<'info>,
}

#[error_code]
pub enum RestrictError {
    #[msg("OOOPS!, I'm introver")]
    OoopsIMIntrovert,
}