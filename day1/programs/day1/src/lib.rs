use anchor_lang::prelude::*;

declare_id!("Af2eZdMRUFKLeG4nuv5uUJUPQFxME5tUEjHm3zrbAKan");

#[program]
pub mod day1 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
