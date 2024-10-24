use anchor_lang::prelude::*;

declare_id!("2ux5XFiyn3Yscv8tXYxikeY9UxVJncWDwc7cEFhY7PAi");

#[program]
pub mod aydo {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
