use cgp::prelude::*;

#[cgp_component {
        name: SymbolTypeComponent,
        provider: ProvideSymbolType,
    }]
pub trait HasSymbolType: Async {
    type Symbol: Async;
}

#[cgp_component {
        name: TimeResolutionTypeComponent,
        provider: ProvideTimeResolutionType,
    }]
pub trait HasTimeResolutionType: Async {
    type TimeResolution: Async;
}
