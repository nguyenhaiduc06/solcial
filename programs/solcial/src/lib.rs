use anchor_lang::prelude::*;

declare_id!("9b78iHesNLGXKEULj9upDG5o6Eiu1f8A8WJ2egGukCaZ");

#[program]
pub mod solcial {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
