use anchor_lang::prelude::*;

declare_id!("Ev434K8JCWjnYeTpjb9UTs12wqvqrZRViCHjD53yRkTT");

#[program]
pub mod day_3 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
