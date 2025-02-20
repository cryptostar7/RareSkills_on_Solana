use anchor_lang::prelude::*;

declare_id!("7diQito7iXkZe5Y3udC9bT275UhHJfqyc34R7AbqH1zN");
/**
 * Signer Patter
 */

// #[program]
// pub mod day_14 {
//     use super::*;

//     pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
//         let the_signer1: &mut Signer = &mut ctx.accounts.signer1;
//         let the_signer2: &mut Signer = &mut ctx.accounts.signer2;

//         msg!("The signer1: {:?}", *the_signer1.key);
//         msg!("The signer2: {:?}", *the_signer2.key);

//         Ok(())
//     }
// }


// #[derive(Accounts)]
// pub struct Initialize<'info> {
//     pub signer1: Signer<'info>,
//     pub signer2: Signer<'info>,
// }


/**
 * Only Owner Patter
 */

const OWNER: &str = "8os8PKYmeVjU1mmwHZZNTEv5hpBXi5VvEKGzykduZAik";

#[program]
pub mod day_14 {
    use super::*;

    #[access_control(check(&ctx))]
    pub fn initialize(ctx: Context<OnlyOwner>) -> Result<()> {
        msg!("Hello, I am owner");
        Ok(())
    }
}

fn check(ctx: &Context<OnlyOwner>) -> Result<()> {
    require_keys_eq!(
        ctx.accounts.signer.key(),
        OWNER.parse::<Pubkey>().unwrap(),
        OnlyOwnerError::NotOwner
    );

    Ok(())
}

#[derive(Accounts)]
pub struct OnlyOwner<'info> {
    signer_account: Signer<'info>,
}


#[error_code]
pub enum OnlyOwnerError {
    #[msg("Only owner can call this function")]
    NotOwner,
}