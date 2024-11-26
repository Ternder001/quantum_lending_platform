use anchor_lang::prelude::*;
use crate::schema::PriceOracle;
use crate::error::ErrorCode;

#[derive(Accounts)]
pub struct GetAssetPrice<'info> {
    pub price_oracle: Account<'info, PriceOracle>,
}

pub fn get_asset_price(price_oracle: &PriceOracle, asset: Pubkey) -> Result<u64> {
    price_oracle
        .get_price(&asset)
        .ok_or(ErrorCode::PriceUnavailable.into())
}
