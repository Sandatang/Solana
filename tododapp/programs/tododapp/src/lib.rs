use anchor_lang::prelude::*;

declare_id!("69vftSwP1vaHhfuv5ssuysmmkwAo1e48Aw3tEmRWkRCq");

#[program]
pub mod tododapp {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateNote {
    #[account(init, payer = user, space = 2000)]
    pub note: Account<'info, Todo>,

    #[account(mut)]
    pub user: Signers<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Todo {
    pub content: String,
    pub user: Pubkey,
}