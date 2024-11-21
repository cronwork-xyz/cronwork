use {crate::state::*, anchor_lang::prelude::*, sablier_network_program::state::Config};

#[derive(Accounts)]
pub struct ThreadDeleteAdmin<'info> {
    #[account(has_one = admin)]
    pub config: AccountLoader<'info, Config>,
    pub admin: Signer<'info>,
    ///CHECKS
    #[account(mut)]
    pub close_to: UncheckedAccount<'info>,
    /// The thread to be paused.
    #[account(mut, close = close_to)]
    pub thread: Account<'info, Thread>,
}

pub fn handler(_ctx: Context<ThreadDeleteAdmin>) -> Result<()> {
    Ok(())
}
