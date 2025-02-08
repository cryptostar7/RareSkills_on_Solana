use anchor_lang::prelude::*;

declare_id!("DUDHZKoRReoY1TcYuXuZkwEMXxCXZ3Wj3XwsWikygTiz");

#[program]
pub mod day_5 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
