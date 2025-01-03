use crate::HasSymbolType;
use cgp::prelude::*;
use std::collections::HashSet;

#[async_trait]
#[cgp_component {
    name: SymbolFetcherComponent,
    provider: SymbolFetcher,
    context: Context,}]
pub trait CanFetchExchangeSymbols: HasSymbolType + HasErrorType {
    async fn fetch_exchange_symbols(&self) -> Result<HashSet<Self::Symbol>, Self::Error>;
}

#[async_trait]
#[cgp_component {
    name: SymbolValidatorComponent,
    provider: SymbolValidator,}]
pub trait CanValidateSymbols: HasSymbolType + HasErrorType {
    async fn validate_symbols(&self, symbols: &[Self::Symbol]) -> Result<bool, Self::Error>;
}
