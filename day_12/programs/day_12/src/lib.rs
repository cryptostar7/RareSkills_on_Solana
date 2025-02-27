use anchor_lang::prelude::*;

declare_id!("4jMse1NJXzi3jsH9bmixxppA3SsCFwydanQwZsWEWugA");

/**
 * Accessing Solana Sysvars in Anchor, using the get method.
 */
// #[program]
// pub mod day_12 {
//     use super::*;

//     pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
//         // Slot is time which new block can be proposed.
//         // One slot is almost 400ms
//         let clock: Clock = Clock::get()?; 
        
//          // Epoch is the duration of slot set which validators can get reward after one epoch. 
//          // One epoch is consist of almost 432000 slots.
//         let epoch_schedule = EpochSchedule::get()?;

//         let rent_var = Rent::get()?;

//         msg!("clock: {:?}", clock);
//         msg!("Epoch Schedule: {:?}", epoch_schedule);
//         msg!("Rent {:?}", rent_var);
//         Ok(())
//     }
// }

// #[derive(Accounts)]
// pub struct Initialize {}

/**
 * Accessing Sysvars in Anchor using Sysvar Public Address
 */

#[program]
pub mod day_12 {
    use super::*;
    use anchor_lang::solana_program::sysvar::{
        instructions, 
        fees::Fees, 
        recent_blockhashes::RecentBlockhashes
    };

    pub fn initialize(ctx: Context<Initialize>, number: u32) -> Result<()> {
        // // Accessing the StakeHistory sysvar
        // // Create an array to store the StakeHistory account
        // let arr = [ctx.accounts.stake_history_account.clone()];

        // // Create an iterator for the array
        // let accounts_iter = &mut arr.iter();

        // // Get the next account info from the iterator (still StakeHistory)
        // let sh_sysvar_info = next_account_info(accounts_iter)?;

        // // Create a StakeHistory instance from the account info
        // let stake_history = StakeHistory::from_account_info(sh_sysvar_info)?;

        // msg!("stake_history: {:?}", stake_history);

        // Ok(())

        // Get Instruction Sysvar
        let arr = [ctx.accounts.instruction_sysvar.clone()];

        let account_info_iter = &mut arr.iter();

        let instruction_sysvar_account = next_account_info(account_info_iter)?;

        // Load the instruction details from the instruction sysvar account
        let instruction_details = 
            instructions::load_instruction_at_checked(0, instruction_sysvar_account)?;
        
        msg!("Instruction details of this transaction: {:?}",
            instruction_details
        );

        msg!("Number is: {}", number);

        Ok(())

    }


}

#[derive(Accounts)]
pub struct Initialize<'info> {

    /// CHECK:
    pub stake_history_account: AccountInfo<'info>, // We create an account for the StakeHistory sysvar

    /// CHECK:
    pub recent_blockhashes: AccountInfo<'info>,

    /// CHECK:
    pub instruction_sysvar: AccountInfo<'info>,
}
