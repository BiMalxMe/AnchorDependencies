use anchor_lang::prelude::*;
use anchor_spl::token::{
    Mint,
    TokenAccount,
    Token
};


declare_id!("74tCK19kyBqwVtiYSmXfZA7mfgbVF8YhNfauvnGNenFG");

#[program]
pub mod anchor_deps {
    use anchor_spl::token::{self, MintTo};

    use super::*;
    pub fn mint_tokens(ctx: Context<MintTokens>, amount: u64) -> Result<()> {
        token::mint_to(
            CpiContext::new(
                // to_accoint_info translatest he progarm in to the raw mode
                ctx.accounts.token_program.to_account_info(),
                MintTo {
                    mint: ctx.accounts.mint.to_account_info(),
                    to: ctx.accounts.recipient_ata.to_account_info(),
                    authority: ctx.accounts.mint_authority.to_account_info(),
                },
            ),
            amount,
        )?;
        Ok(())
    }
}
// this struct checks and lists all accounts needed for the mint_tokens function
#[derive(Accounts)]
pub struct MintTokens<'info> {
    #[account(mut)]
    // the mint account of the token (like the "template" for the token)
    // mut = mutable because minting increases total supply
    pub mint: Account<'info, Mint>,

    #[account(mut)]
    // the user's token account where new tokens will arrive
    // usually an associated token account (ata)
    pub recipient_ata: Account<'info, TokenAccount>,

    // the person or key that has permission to mint this token
    // they must sign the transaction
    pub mint_authority: Signer<'info>,

    // the official spl token program (never changes)
    // we need it to do the actual minting
    pub token_program: Program<'info, Token>,
}
