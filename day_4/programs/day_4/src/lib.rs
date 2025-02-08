use anchor_lang::prelude::*;

declare_id!("3u5QzuRq3sNNb34rX1Ki99DUPthmh3WP9PhExqu28nfA");

// // Using the err! statement
// #[program]
// pub mod day_4 {
//     use super::*;

//     pub fn limit_range(ctx: Context<LimitRange>, a: u64) -> Result<()> {
//         if a < 10 {
//             return err!(MyError::AisTooSmall);
//         }
//         if a > 100 {
//             return err!(MyError::AisTooBig);
//         }

//         msg!("Result = {}", a);
//         Ok(())
//     }

//     pub fn func(ctx: Context<LimitRange>) -> Result<()> {
//         msg!("Will this print?");
//         return err!(MyError::AlwaysErrors);
//     }
// }

// #[derive(Accounts)]
// pub struct LimitRange {}

// #[error_code]
// pub enum MyError {
//     #[msg("a is too big")]
//     AisTooBig,
//     #[msg("a is too small")]
//     AisTooSmall,
//     #[msg("AlwaysErrors")]
//     AlwaysErrors,
// }

/**
 * @title : Using the require statement
 */

#[program]
pub mod day_4 {
    use super::*;

    pub fn limit_range(ctx: Context<ReturnError>, a: u64) -> Result<()> {
        require!(a >= 10, Day4Error::AisTooSmall);
        require!(a <= 100, Day4Error::AisTooBig);

        msg!("Result = {}", a);
        Ok(())
    }

    pub fn func(ctx: Context<ReturnError>) -> Result<()> {
        msg!("Will this print?");
        return err!(Day4Error::AlwaysErrors);
    }
}

#[derive(Accounts)]
pub struct ReturnError {}

#[error_code]
pub enum Day4Error {
    #[msg("a is too small")]
    AisTooSmall,
    #[msg("a is too big")]
    AisTooBig,
    #[msg("AlwaysErrors")]
    AlwaysErrors,
}