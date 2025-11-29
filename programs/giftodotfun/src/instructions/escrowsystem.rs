use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};

#[derive(Accounts)]
pub struct EnterEscrow<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(init, space = 8, payer = signer, seeds = [b"escrow", signer.key().as_ref()], bump)]
    pub escrowpdaaccount: Account<'info, Empty>,

    pub system_program: Program<'info, System>,


}

#[account]
pub struct Empty {}

pub fn CreateGift(ctx: Context<EnterEscrow>, amount: u64) -> Result<()> {
    let create_account = ctx.accounts.signer.to_account_info();
    let pda_account = ctx.accounts.escrowpdaaccount.to_account_info();
    let system_program = ctx.accounts.system_program.to_account_info();

    let cpi_context = CpiContext::new(
        system_program,
        Transfer {
            from: create_account,
            to: pda_account,
        },
    );
    transfer(cpi_context, amount)?;

    msg!("We have sucessfully deposited funds into {:?}", ctx.accounts.escrowpdaaccount.key().as_ref());

    Ok(())
}


// escrowsystem.rs