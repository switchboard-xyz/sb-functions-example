pub mod sdk;
pub use sdk::*;
use anchor_lang::prelude::*;
use solana_program::pubkey;
use bytemuck;

declare_id!("8kjszBCEgkzAsU6QySHSZvr9yFaboau2RnarCQFFvasS");

#[derive(Default, Eq, PartialEq, Copy, Clone, AnchorSerialize, AnchorDeserialize, Debug)]
pub struct BorshDecimal {
    pub mantissa: i128,
    pub scale: u32,
}

#[program]
pub mod demo {
    use super::*;

    pub fn ping(ctx: Context<Ping>, params: PingParams) -> Result<()> {
        // let func = ctx.accounts.function.load()?;
        msg!("{:#?}", params);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ping<'info> {
    /// CHECK: todo
    #[account(constraint = validate_fn_quote(&function, &quote, &signer))]
    pub function: AccountInfo<'info>,
    /// CHECK: todo
    pub quote: AccountInfo<'info>,
    pub signer: Signer<'info>,
}
#[derive(Clone, AnchorSerialize, AnchorDeserialize, Debug, Default)]
pub struct PingParams {
    pub prices: Vec<BorshDecimal>,
    pub volumes: Vec<BorshDecimal>,
    pub twaps: Vec<BorshDecimal>,
}
