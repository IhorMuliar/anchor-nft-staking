use anchor_lang::prelude::*;
use anchor_spl::{ token::{ Token, Mint } };

use crate::state::StakeConfig;
use crate::error::StakeError;

#[derive(Accounts)]
pub struct InitializeConfig<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init,
        payer = admin,
        mint::authority = config,
        mint::decimals = 6,
        seeds = [b"rewards_mint", config.key().as_ref()],
        bump
    )]
    pub rewards_mint: Account<'info, Mint>,

    #[account(init, payer = admin, space = 8 + StakeConfig::INIT_SPACE, seeds = [b"config"], bump)]
    pub config: Account<'info, StakeConfig>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitializeConfig<'info> {
    pub fn initialize_config(
        &mut self,
        points_per_stake: u8,
        max_stake: u8,
        freeze_period: u32,
        bumps: &InitializeConfigBumps
    ) -> Result<()> {
        // Validate configuration parameters
        require!(
            points_per_stake > 0 && max_stake > 0 && freeze_period > 0,
            StakeError::InvalidConfiguration
        );
        self.config.set_inner(StakeConfig {
            points_per_stake,
            max_stake,
            freeze_period,
            reward_bump: bumps.rewards_mint,
            bump: bumps.config,
        });

        Ok(())
    }
}