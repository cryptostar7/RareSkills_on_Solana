use anchor_lang::prelude::*;

declare_id!("QiDoXEMhQmY9Q2uJzWRuxFGsoPFiyr4Fa9AMv9pNxzd");

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
