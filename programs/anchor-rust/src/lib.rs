use anchor_lang::prelude::*;

declare_id!("6i8YpAr8uQEZax7SGFPc6X7zqqdD63gw9gxn2X6xWHwW");

#[program]
pub mod anchor_rust {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
