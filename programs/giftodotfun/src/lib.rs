pub mod instructions;
use instructions::*;
use anchor_lang::prelude::*;
pub use instructions::*;

declare_id!("ALbkwBmpwk57s3t2S3DPC6H8WM1aQiKDUdH1DzVsR66S");

#[program]
pub mod giftodotfun {
    use super::*;

    pub fn CreateGiftLink(ctx: Context<EnterEscrow>, amount: u64) -> Result<()> {
        CreateGift(ctx, amount)
    }
}


// lib.rs