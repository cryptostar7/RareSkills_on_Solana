use anchor_lang::prelude::*;

declare_id!("GCsXFvpmXxuasPzYUahPtKbhjVsS3NW6wL7Xkijb3WdT");

#[program]
pub mod day_15 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
