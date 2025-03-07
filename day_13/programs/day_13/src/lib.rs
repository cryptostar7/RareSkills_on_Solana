use anchor_lang::prelude::*;

declare_id!("72hfAr9LadTVsxRHHnGGkxDDLFaJCK5iDYzct3pBDiBB");

#[program]
pub mod day_13 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        emit!(MyEvent { value: 1 });
        emit!(MySecondEvent { value: 2, message: "Akira is good".to_string() });
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[event]
pub struct MyEvent {
    pub value: u64,
}

#[event]
pub struct MySecondEvent {
    pub value: u64,
    pub message: String,
}
