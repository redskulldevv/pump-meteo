use anchor_lang::prelude::*;

use anchor_spl::token::Token;
// #[event_cpi]
// #[instruction(active_id: i32, bin_step: u16)]
#[derive(Accounts)]
pub struct InitBinArray<'info> {
    #[account(mut)]
    pub funder: Signer<'info>,

    #[account(mut)]
    /// CHECK: initialize positioin
    pub bin_array: UncheckedAccount<'info>,

    ///CHECK: The pool address
    pub lb_pair: UncheckedAccount<'info>,

    /// CHECK: system program
    pub system_program: Program<'info, System>,

    /// CHECK: DLMM program
    pub dlmm_program: UncheckedAccount<'info>,

    /// CHECK: event_authority
    pub event_authority: UncheckedAccount<'info>,
}

pub fn init_bin_array<'a, 'b, 'c, 'info>(
    ctx: Context<'a, 'b, 'c, 'info, InitBinArray<'info>>,
    index: i64,
) -> Result<()> {
    msg!("Initialize binArray");

    let accounts = dlmm::cpi::accounts::InitializeBinArray {
        lb_pair: ctx.accounts.lb_pair.to_account_info(),
        funder: ctx.accounts.funder.to_account_info(),
        bin_array: ctx.accounts.bin_array.to_account_info(),
        system_program: ctx.accounts.system_program.to_account_info(),
    };
    let cpi_context = CpiContext::new(ctx.accounts.dlmm_program.to_account_info(), accounts)
        .with_remaining_accounts(ctx.remaining_accounts.to_vec());

    dlmm::cpi::initialize_bin_array(cpi_context, index);

    Ok(())
}
