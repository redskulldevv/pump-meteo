use anchor_lang::prelude::*;
use anchor_spl::token::Token;

#[derive(Accounts)]
pub struct InitBinArrayBitmapExtension<'info> {
    #[account(mut)]
    pub funder: Signer<'info>,

    #[account(mut)]
    /// CHECK: initialize positioin
    pub bin_array_bitmap_extension: UncheckedAccount<'info>,

    ///CHECK: The pool address
    pub lb_pair: UncheckedAccount<'info>,

    /// CHECK: system program
    pub system_program: Program<'info, System>,

    /// CHECK: DLMM program
    pub dlmm_program: UncheckedAccount<'info>,

    /// CHECK: event_authority
    pub event_authority: UncheckedAccount<'info>,

    pub rent: Sysvar<'info, Rent>,
}

pub fn init_bin_array_bitmap_extension<'a, 'b, 'c, 'info>(
    ctx: Context<'a, 'b, 'c, 'info, InitBinArrayBitmapExtension<'info>>,
) -> Result<()> {
    msg!("Initialize binArrayBitmapExtension");

    let accounts = dlmm::cpi::accounts::InitializeBinArrayBitmapExtension {
        lb_pair: ctx.accounts.lb_pair.to_account_info(),
        funder: ctx.accounts.funder.to_account_info(),
        bin_array_bitmap_extension: ctx.accounts.bin_array_bitmap_extension.to_account_info(),
        system_program: ctx.accounts.system_program.to_account_info(),
        rent: ctx.accounts.rent.to_account_info(),
    };
    let cpi_context = CpiContext::new(ctx.accounts.dlmm_program.to_account_info(), accounts)
        .with_remaining_accounts(ctx.remaining_accounts.to_vec());

    dlmm::cpi::initialize_bin_array_bitmap_extension(cpi_context);

    Ok(())
}
