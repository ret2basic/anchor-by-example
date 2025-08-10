use anchor_lang::prelude::*;
declare_id!("Fp9AwyS3tNwRQT54nwjNa5WVKrF2Vn3xHAZTPKvtWute");
#[program]
pub mod hello_world {
    use super::*;
    pub fn hello_world(_ctx: Context<Initialize>) -> Result<()> {
        msg!("Hello world, from solana smart contract");
        Ok(())
    }
}
#[derive(Accounts)]
pub struct Initialize {}
