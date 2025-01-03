use cgp::prelude::*;

#[cgp_component {
        name: SymbolTypeComponent,
        provider: SymbolTypeProvider,
    }]
pub trait HasSymbolType: Async {
    type Symbol: Async;
}

#[cgp_component {
        name: TimeResolutionTypeComponent,
        provider: TimeResolutionTypeProvider,
    }]
pub trait HasTimeResolutionType: Async {
    type TimeResolution: Async;
}
