use anchor_lang::prelude::*;

declare_id!("A7dYbtHFqYppVGUw7ew6WnrNmA6uezQt9RwCYNKWbWyr");

#[program]
pub mod mint_nft {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
