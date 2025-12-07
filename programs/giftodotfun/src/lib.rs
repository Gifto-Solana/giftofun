pub mod instructions;
use instructions::*;
use anchor_lang::prelude::*;
pub use instructions::*;

declare_id!("ALbkwBmpwk57s3t2S3DPC6H8WM1aQiKDUdH1DzVsR66S");

#[program]
pub mod giftodotfun {
    use super::*;

    pub fn create_gift_link(ctx: Context<EnterEscrow>, amount: u64) -> Result<()> {
        create_gift(ctx, amount)
    }

    pub fn redeem_gift_link(ctx: Context<LeaveEscrow>) -> Result<()> {
        redeem_gift(ctx)
    }
}


// lib.rs