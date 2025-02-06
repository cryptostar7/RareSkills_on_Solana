use anchor_lang::prelude::*;

declare_id!("wVZkGRvX27oEyomZNyCEWJYjK4zk1kMqrBQVDgRK2Wh");

#[program]
pub mod day2 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
