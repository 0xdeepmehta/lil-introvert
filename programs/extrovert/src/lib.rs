use anchor_lang::prelude::*;
use lil_introvert::cpi::accounts::Initialize;
use lil_introvert::program::LilIntrovert;

declare_id!("47ai2PNBTrRW9R7cqCHkk9keEovYvDBsNV7Kyrf8vtpU");

#[program]
pub mod extrovert {
    use super::*;

    pub fn let_do_some_bully(ctx: Context<LetsBully>) -> Result<()> {
        let cpi_program = ctx.accounts.introvert_program.to_account_info();
        let cpi_account = Initialize {
            instruction_sysvar_account: ctx.accounts.instruction_sysvar_account.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_account);
        lil_introvert::cpi::initialize(cpi_ctx)

    }
}

#[derive(Accounts)]
pub struct LetsBully<'info> {
    /// CHECK: Checked by CPI
    pub instruction_sysvar_account: AccountInfo<'info>,
    pub introvert_program: Program<'info, LilIntrovert>,
}
