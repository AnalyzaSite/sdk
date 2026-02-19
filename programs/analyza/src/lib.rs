use anchor_lang::prelude::*;

declare_id!("Analyza1111111111111111111111111111111111");

#[program]
pub mod analyza {
    use super::*;

    pub fn initialize_pool(ctx: Context<InitializePool>) -> Result<()> {
        let pool = &mut ctx.accounts.pool;
        pool.authority = *ctx.accounts.authority.key;
        pool.total_minted = 0;
        pool.max_supply = 1000;
        Ok(())
    }

    pub fn mint(ctx: Context<Mint>, ai_score: u8) -> Result<()> {
        let pool = &mut ctx.accounts.pool;

        require!(pool.total_minted < pool.max_supply, ErrorCode::SoldOut);
        require!(ai_score >= 70, ErrorCode::LowFairnessScore);

        pool.total_minted += 1;

        Ok(())
    }
}

#[account]
pub struct MintPool {
    pub authority: Pubkey,
    pub total_minted: u64,
    pub max_supply: u64,
}

#[derive(Accounts)]
pub struct InitializePool<'info> {
    #[account(init, payer = authority, space = 8 + 32 + 8 + 8)]
    pub pool: Account<'info, MintPool>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Mint<'info> {
    #[account(mut)]
    pub pool: Account<'info, MintPool>,
    pub user: Signer<'info>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Mint already sold out")]
    SoldOut,
    #[msg("AI fairness score too low")]
    LowFairnessScore,
}
