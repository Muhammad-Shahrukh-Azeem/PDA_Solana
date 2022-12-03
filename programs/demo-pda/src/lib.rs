use anchor_lang::prelude::*;
use std::mem::size_of;

declare_id!("9vXBaSaS7cjBGfzNjWGPiyWYjkFjjvoWAe4wWtvaSDew");

#[program]
pub mod demo_pda {
    use super::*;

    pub fn create_escrow(ctx: Context<CreateEscrow>, amount: u64) -> Result<()> {

        let escrow = &mut ctx.accounts.escrow;

        escrow.from = ctx.accounts.from.key();

        escrow.to = ctx.accounts.to.key();

        escrow.amount = amount;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateEscrow<'info> {

    #[account(
        init,

        seeds = [b"escrow".as_ref(), from.key().as_ref(), to.key().as_ref()],
        bump,
        payer = from,
        space = size_of::<EscrowAccount>()
    )]

    pub escrow: Account<'info, EscrowAccount>,

    #[account(mut)]
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub from: Signer<'info>,

    #[account(mut)]
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub to: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct EscrowAccount {
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub from: Pubkey,

    /// CHECK: This is not dangerous because we don't read or write from this account
    pub to: Pubkey,

    pub amount: u64,
}