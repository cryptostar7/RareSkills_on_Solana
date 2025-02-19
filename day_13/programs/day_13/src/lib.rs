use anchor_lang::prelude::*;

declare_id!("72hfAr9LadTVsxRHHnGGkxDDLFaJCK5iDYzct3pBDiBB");

#[program]
pub mod day_13 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
