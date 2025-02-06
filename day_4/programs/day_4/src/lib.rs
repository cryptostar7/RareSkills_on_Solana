use anchor_lang::prelude::*;

declare_id!("3u5QzuRq3sNNb34rX1Ki99DUPthmh3WP9PhExqu28nfA");

#[program]
pub mod day_4 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
