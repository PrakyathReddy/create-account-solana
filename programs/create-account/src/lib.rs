use anchor_lang::prelude::*;
use anchor_lang::system_program;

declare_id!("82qMKny4FNb1ZXiwD9xQEXEV7bjEeUCZcrmjMMizho1M");

const LAMPORTS_PER_SOL: u64 = 1_000_000_000;

#[program]
pub mod create_system_account {
    use super::*;

    pub fn create_system_account(ctx: Context<CreateSystemAccount>) -> Result<()> {
        msg!("Program invoked, creating a system account....");
        let new_pubkey = ctx.accounts.new_account.key().to_string();
        msg!("New public key will be: {}", new_pubkey);

        system_program::create_account(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                system_program::CreateAccount {
                    from: ctx.accounts.payer.to_account_info(),
                    to: ctx.accounts.new_account.to_account_info(),
                },
            ),
            LAMPORTS_PER_SOL,                   // 1 SOL
            0,                                  // space = 0 bytes
            &ctx.accounts.system_program.key(), // program id
        )?;

        msg!("Account created successfully!");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateSystemAccount<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut)]
    pub new_account: Signer<'info>,
    pub system_program: Program<'info, System>,
}
