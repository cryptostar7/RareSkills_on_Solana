use anchor_lang::prelude::*;

declare_id!("9mAj7689DFavC2rnz7U4wCG9oeH59qWrZSEQDPE7TRtU");

#[program]
pub mod day_10 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

fn get_a_num() -> u64 {
    1
}

#[derive(Accounts)]
pub struct Initialize {}
