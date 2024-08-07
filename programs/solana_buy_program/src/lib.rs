use anchor_lang::prelude::*;

declare_id!("Hktf94z7aAKJkTaBmaRLe5WZyLsWXQws3TvCkwCEsEbp");

#[program]
pub mod solana_buy_program {
    use super::*;

    pub fn buy(ctx: Context<Buy>, id: String, amount: u64) -> Result<()> {
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
            tx_hash: ctx.accounts.buyer.key().to_string() 
        });

        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.contract.key(),
            &ctx.accounts.admin.key(),
            amount,
        );
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.contract.to_account_info(),
                ctx.accounts.admin.to_account_info(),
            ],
        )?;

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

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(mut)]
    pub contract: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[event]
pub struct TransactionSuccess {
    pub id: String,
    pub amount: u64,
    pub buyer: Pubkey,
    pub tx_hash: String,
}
