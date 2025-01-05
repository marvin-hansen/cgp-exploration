use crate::{HasApiUrl, HasSymbolType};
use cgp::prelude::*;
use std::collections::HashSet;

#[cgp_component {
    name:  SymbolFetchComponent,
    provider: SymbolFetchProvider,}]
#[async_trait]
pub trait CanFetchExchangeSymbols: HasSymbolType + HasErrorType + HasApiUrl + Async {
    async fn fetch_exchange_symbols(&self) -> Result<HashSet<Self::Symbol>, Self::Error>;
}

#[cgp_component {
    name:  SymbolValidatorComponent,
    provider: SymbolValidatorProvider,}]
#[async_trait]
pub trait CanValidateSymbols: HasSymbolType + HasErrorType + Async {
    async fn validate_symbols(&self, symbols: &[Self::Symbol]) -> Result<bool, Self::Error>;
}
