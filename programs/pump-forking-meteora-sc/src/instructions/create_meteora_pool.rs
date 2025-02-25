use anchor_lang::prelude::*;
use anchor_spl::token::Token;

#[derive(Accounts)]
pub struct CreateMeteoraPool<'info> {
    #[account(mut)]
    /// CHECK: The pool account
    pub lb_pair: UncheckedAccount<'info>,

    /// CHECK: Bin array extension account of the pool
    pub bin_array_bitmap_extension: Option<UncheckedAccount<'info>>,

    /// CHECK: Mint account of token X
    pub token_mint_x: UncheckedAccount<'info>,

    /// CHECK: Mint account of token Y
    pub token_mint_y: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK: Reserve account of token X
    pub reserve_x: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK: Reserve account of token Y
    pub reserve_y: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK: Oracle account of the pool
    pub oracle: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK: Preset parameter account of the pool
    pub preset_parameter: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK: User who's create the pool
    pub funder: Signer<'info>,

    /// CHECK: DLMM program
    pub dlmm_program: UncheckedAccount<'info>,

    /// CHECK: DLMM program event authority for event CPI
    pub event_authority: UncheckedAccount<'info>,

    /// CHECK: Token program
    pub token_program: Program<'info, Token>,

    /// CHECK: System program
    pub system_program: Program<'info, System>,

    pub rent: Sysvar<'info, Rent>,
}

pub fn create_meteora_pool<'a, 'b, 'c, 'info>(
    ctx: Context<'a, 'b, 'c, 'info, CreateMeteoraPool<'info>>,
    active_id: i32,
    bin_step: u16,
) -> Result<()> {
    msg!("Create meteora pool");

    let accounts = dlmm::cpi::accounts::InitializeLbPair {
        lb_pair: ctx.accounts.lb_pair.to_account_info(),
        bin_array_bitmap_extension: ctx
            .accounts
            .bin_array_bitmap_extension
            .as_ref()
            .map(|account| account.to_account_info()),
        token_mint_x: ctx.accounts.token_mint_x.to_account_info(),
        token_mint_y: ctx.accounts.token_mint_y.to_account_info(),
        reserve_x: ctx.accounts.reserve_x.to_account_info(),
        reserve_y: ctx.accounts.reserve_y.to_account_info(),
        oracle: ctx.accounts.oracle.to_account_info(),
        preset_parameter: ctx.accounts.preset_parameter.to_account_info(),
        funder: ctx.accounts.funder.to_account_info(),
        token_program: ctx.accounts.token_program.to_account_info(),
        system_program: ctx.accounts.system_program.to_account_info(),
        rent: ctx.accounts.rent.to_account_info(),
        event_authority: ctx.accounts.event_authority.to_account_info(),
        program: ctx.accounts.dlmm_program.to_account_info(),
    };
    let cpi_context = CpiContext::new(ctx.accounts.dlmm_program.to_account_info(), accounts)
        .with_remaining_accounts(ctx.remaining_accounts.to_vec());

    dlmm::cpi::initialize_lb_pair(cpi_context, active_id, bin_step);

    Ok(())
}
