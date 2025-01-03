use cgp::prelude::*;

#[cgp_component {
        name: SymbolTypeComponent,
        provider: ProvideSymbolType}]
pub trait HasSymbolType {
    type Symbol;
}

#[cgp_component {
        name: TimeResolutionTypeComponent,
        provider: ProvideTimeResolutionType,}]
pub trait HasTimeResolutionType {
    type TimeResolution;
}
