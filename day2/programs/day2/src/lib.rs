use anchor_lang::prelude::*;

declare_id!("wVZkGRvX27oEyomZNyCEWJYjK4zk1kMqrBQVDgRK2Wh");

#[program]
pub mod day2 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, a: u64, b: u64, message: String) -> Result<()> {
        msg!("You sent {} and {}", a, b);
        msg!("You said: {:?}", message);
        let x: u64 = 2;
        let y = 3;
        let result = x.pow(y);
        msg!("The result is {}", result);
        Ok(())
    }

    pub fn array(ctx: Context<Initialize>, arr: Vec<u64>) -> Result<()> {
        msg!("Your array {:?}", arr);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
