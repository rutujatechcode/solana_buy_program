use anchor_lang::prelude::*;

declare_id!("3y7L66k8qA63gxjTdfjZCFpNZBFQWdtskGQhThBspzxy");

#[program]
pub mod solana_buy_program {
    use super::*;

    pub fn buy(ctx: Context<Buy>, id: u64, amount: u64) -> Result<()> {
        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.buyer.key(),
            &ctx.accounts.contract.key(),
            amount,
        );
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.buyer.to_account_info(),
                ctx.accounts.contract.to_account_info(),
            ],
        )?;
        
        emit!(TransactionSuccess { 
            id,
            amount, 
            buyer: *ctx.accounts.buyer.key, 
        });

        // Print the success message with user address, id, and amount
        msg!(
            "User: {}, ID: {}, Amount: {}",
            *ctx.accounts.buyer.key,
            id,
            amount
        );

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Buy<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,
    #[account(mut)]
    pub contract: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}


#[event]
pub struct TransactionSuccess {
    pub id: u64,
    pub amount: u64,
    pub buyer: Pubkey,
}
