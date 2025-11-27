use anchor_lang::prelude::*;

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
pub struct Initialize {}
