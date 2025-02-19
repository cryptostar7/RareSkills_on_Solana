use anchor_lang::prelude::*;

declare_id!("4jMse1NJXzi3jsH9bmixxppA3SsCFwydanQwZsWEWugA");

#[program]
pub mod day_12 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
