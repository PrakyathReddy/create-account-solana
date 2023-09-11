use anchor_lang::prelude::*;

declare_id!("82qMKny4FNb1ZXiwD9xQEXEV7bjEeUCZcrmjMMizho1M");

#[program]
pub mod create_account {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
