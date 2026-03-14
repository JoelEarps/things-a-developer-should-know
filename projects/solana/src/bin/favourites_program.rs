/*

First smart contract will store our favourite things on the blockchain.
We will learn:
1. Save
2. Update
3. Retrieve update
4. How signing can be used to limit access

In solana smart contracts can store additional data in program derived addresses (PDAs).
PDA address are derived from seeds, they are a type of Key value pair

Solana playgorund

Anchor discrimiatior size

Anchor is a framework that uses rust macros to generate code and reduce boiler plate
This though does add overhead.
*/

/// What does this macro do?
/// Similar to the alloy-rs macro for generate a smart contract
/// I
///
use anchor_lang::prelude::*;

declare_id!("FavProgram1111111111111111111111111111111111");

#[program]
pub mod favourites {
    use super::*;

    pub fn set_favourite(ctx: Context<SetFavourites>, favourite: String) -> Result<()> {
        let favourites_account = &mut ctx.accounts.favourites;
        msg!("Setting favourite: {}", favourite);
        Ok(())
    }
}

#[account]
#[derive(InitSpace)]
pub struct Favourites {
    #[max_len(50)]
    pub name: String,
    pub number: u64,
    #[max_len(50)]
    pub color: String,
    #[max_len(5, 50)]
    pub list_of_hobbies: Vec<String>,
}

// It is idiomatic to call the name of the accounts
// When people call set_favourite they will need to call a list of accounts that will be interacted with for the transaction
// So we declare this a struct of accounts
#[derive(Accounts)]
pub struct SetFavourites<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init_if_needed,
        payer = user,
        space = 8 + Favourites::INIT_SPACE,
        seeds = [b"favourites", user.key().as_ref()],
        bump,
    )]
    pub favourites: Account<'info, Favourites>,

    pub system_program: Program<'info, System>,
}

#[tokio::main]
async fn main() {
    unimplemented!()
}
