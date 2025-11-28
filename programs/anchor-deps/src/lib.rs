use anchor_lang::prelude::*;
use anchor_spl::token::Mint;


declare_id!("74tCK19kyBqwVtiYSmXfZA7mfgbVF8YhNfauvnGNenFG");

#[program]
pub mod anchor_deps {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct MintNFT<'info>{
#[account(mut)]
pub mint : Account<'info,Mint>

#[account(mut)]

}
