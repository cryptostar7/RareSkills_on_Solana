use anchor_lang::{
    prelude::*,
    solana_program::sysvar::recent_blockhashes::RecentBlockhashes
};

declare_id!("3q9MZDMjGM7UD1fAWFDGkxnuBcBALcshmiFL3AGgZi6V");

#[program]
pub mod day_11 {
    use super::*;
    use chrono::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let clock: Clock = Clock::get()?;
        let time_stamp = clock.unix_timestamp;

        let data_time = chrono::NaiveDateTime::from_timestamp_opt(time_stamp, 0).unwrap();
        let day_of_the_week = data_time.weekday();

        msg!("Week day is: {}", day_of_the_week);

        let arr = [ctx.accounts.recent_blockhashes.clone()];
        let accounts_iter = &mut arr.iter();
        let sh_sysvar_info = next_account_info(accounts_iter)?;
        let recent_blockhashes = RecentBlockhashes::from_account_info(sh_sysvar_info);
        let data = recent_blockhashes.last().unwrap();

        msg!("The last blockhash is: {}", data.blockhash);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    pub recent_blockhashes: AccountInfo<'info>,
}
