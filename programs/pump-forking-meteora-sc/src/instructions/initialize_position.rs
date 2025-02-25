use anchor_lang::prelude::*;
use anchor_spl::token::Token;

#[derive(Accounts)]
pub struct InitializePosition<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    /// CHECK: initialize positioin
    pub position: UncheckedAccount<'info>,

    ///CHECK: The pool address
    pub lb_pair: UncheckedAccount<'info>,

    pub base: Signer<'info>,

    pub owner: Signer<'info>,

    /// CHECK: system program
    pub sys_program: Program<'info, System>,

    /// CHECK: DLMM program
    pub dlmm_program: UncheckedAccount<'info>,

    /// CHECK: event_authority
    pub event_authority: UncheckedAccount<'info>,

    pub rent: Sysvar<'info, Rent>,
}

pub fn initialize_position<'a, 'b, 'c, 'info>(
    ctx: Context<'a, 'b, 'c, 'info, InitializePosition<'info>>,
    lower_bin_id: i32,
    width: i32,
) -> Result<()> {
    msg!("Initialize Positioin");

    let accounts = dlmm::cpi::accounts::InitializePositionPda {
        position: ctx.accounts.position.to_account_info(),
        lb_pair: ctx.accounts.lb_pair.to_account_info(),
        payer: ctx.accounts.payer.to_account_info(),
        base: ctx.accounts.base.to_account_info(),
        owner: ctx.accounts.owner.to_account_info(),
        system_program: ctx.accounts.sys_program.to_account_info(),
        rent: ctx.accounts.rent.to_account_info(),
        program: ctx.accounts.dlmm_program.to_account_info(),
        event_authority: ctx.accounts.event_authority.to_account_info(),
    };
    let cpi_context = CpiContext::new(ctx.accounts.dlmm_program.to_account_info(), accounts)
        .with_remaining_accounts(ctx.remaining_accounts.to_vec());

    dlmm::cpi::initialize_position_pda(cpi_context, lower_bin_id, width);

    Ok(())
}
