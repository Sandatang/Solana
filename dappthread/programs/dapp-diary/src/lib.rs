use anchor_lang::prelude::*;

declare_id!("DJYHqZPeSeQZgqTAqifgKm1AXSaN3RTCTsWxzsSPdqNR");

#[program]
pub mod dapp_diary {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
