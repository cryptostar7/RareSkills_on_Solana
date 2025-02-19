use anchor_lang::prelude::*;

declare_id!("7diQito7iXkZe5Y3udC9bT275UhHJfqyc34R7AbqH1zN");

#[program]
pub mod day_14 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
