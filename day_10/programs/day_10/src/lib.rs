use anchor_lang::prelude::*;

declare_id!("9mAj7689DFavC2rnz7U4wCG9oeH59qWrZSEQDPE7TRtU");

/**
 * Public Function
 */

// #[program]
// pub mod day_10 {
//     use super::*;

//     pub fn my_public_function(ctx: Context<Initialize>) ->  Resul<()> {
//         Ok(())
//     }
// }

// #[derive(Accounts)]
// pub struct Initialize {}


/**
 * Interan Function
 */
// #[program]
// pub mod day_10 {
//     use super::*;

//     pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
//         msg!("Greetings from: {:?}", ctx.program_id);
//         Ok(())
//     }

//     pub mod some_internal_function {
//         pub fn internal_function() {

//         }
//     }
// }

// mod do_something {
//     use crate::day_10;

//     pub fn some_func_here () {
//         day_10::some_internal_function::internal_function();
//     }
// }

// #[derive(Accounts)]
// pub struct Initialize {}

/**
 * Private Function
 */

// #[program]
// pub mod day_10 {
//     use super::*;

//     pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
//         some_private_function::private_function();

//         Ok(())
//     }

//     pub mod some_private_function {
//         pub(in crate::day_10) fn private_function() {
            
//         }
//     }
// }

// mod do_something {
//     use crate::day_10;

//     pub fn some_func_here() {
//         day_10::some_private_function::private_function();
//     }
// }

// #[derive(Accounts)]
// pub struct Initialize {}

/**
 * Importing the modules from another file
 */

pub mod calculate;

#[program]
pub mod day_10 {
    use super::*;

    pub fn add_two_numbers(_ctx: Context<AddTwoNumber>, x: u64, y: u64) -> Result<()> {
        let result = calculate::add(x, y);

        msg!("{} + {} = {}", x, y, result);
        Ok(())
    }

    fn test() -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct AddTwoNumber {}
