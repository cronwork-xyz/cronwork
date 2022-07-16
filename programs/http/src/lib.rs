pub mod errors;
pub mod id;
pub mod state;

mod instructions;

pub use id::ID;

use anchor_lang::prelude::*;
use instructions::*;
use state::HttpMethod;

#[program]
pub mod cronos_http {
    use super::*;

    pub fn admin_fee_claim<'info>(ctx: Context<AdminFeeClaim>, amount: u64) -> Result<()> {
        admin_fee_claim::handler(ctx, amount)
    }

    pub fn api_new<'info>(ctx: Context<ApiNew>, base_url: String) -> Result<()> {
        api_new::handler(ctx, base_url)
    }

    pub fn fee_claim<'info>(ctx: Context<FeeClaim>, amount: u64) -> Result<()> {
        fee_claim::handler(ctx, amount)
    }

    pub fn initialize<'info>(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }

    pub fn request_ack<'info>(ctx: Context<RequestAck>) -> Result<()> {
        request_ack::handler(ctx)
    }

    pub fn request_new<'info>(
        ctx: Context<RequestNew>,
        id: String,
        method: HttpMethod,
        route: String,
    ) -> Result<()> {
        request_new::handler(ctx, id, method, route)
    }
}
