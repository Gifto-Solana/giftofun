use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};

#[derive(Accounts)]
pub struct EnterEscrow<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    // this is the wallet that will deposits funds into the escrowpdaaccount.

    
    pub throwaway_signer: Signer<'info>,
    // ok this throwaway_signer will be a signer made by the frontend. so like it will be in the link or something. we'll figure it out on the way.

    #[account(init, space = 8 + Empty::INIT_SPACE, payer = signer, seeds = [b"gift", throwaway_signer.key().as_ref()], bump)]
    pub escrowpdaaccount: Account<'info, Empty>,
    // this thing actually stores the gift funds

    pub system_program: Program<'info, System>,

}

#[derive(InitSpace)]
#[account]
pub struct Empty {
    creator: Pubkey,

}

pub fn create_gift(ctx: Context<EnterEscrow>, amount: u64) -> Result<()> {
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

    ctx.accounts.escrowpdaaccount.creator = ctx.accounts.signer.key();
    // establishing the creator of the gift

    msg!("We have sucessfully deposited funds into {:?}", ctx.accounts.escrowpdaaccount.key().as_ref());

    Ok(())
}


pub fn redeem_gift(ctx: Context<LeaveEscrow>) -> Result<()> {
    // That's it! close = redeemer handles everything automatically
    Ok(())
}


#[derive(Accounts)]
pub struct LeaveEscrow<'info> {
    #[account(mut)]
    pub redeemer: Signer<'info>,
    
    pub throwaway_signer: Signer<'info>,
    
    #[account(
        mut,
        seeds = [b"gift", throwaway_signer.key().as_ref()],
        bump,
        close = redeemer
    )]
    pub escrowpdaaccount: Account<'info, Empty>,
}
