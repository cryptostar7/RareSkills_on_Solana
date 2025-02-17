use anchor_lang::prelude::*;

declare_id!("3q9MZDMjGM7UD1fAWFDGkxnuBcBALcshmiFL3AGgZi6V");

#[program]
pub mod day_11 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
