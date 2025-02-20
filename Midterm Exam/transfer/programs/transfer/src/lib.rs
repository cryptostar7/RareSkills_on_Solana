use anchor_lang::prelude::*;

declare_id!("7iFDug1T87BetkqUDAEakFktt8UvKDGpSnS3ZNhtLJqF");

#[program]
pub mod transfer {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
