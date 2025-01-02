use cgp::prelude::*;

#[async_trait]
#[cgp_component {
    name: EventProcessorComponent,
    provider: EventProcessor,}]
pub trait CanProcessEvent: Async + HasErrorType {
    async fn process_event(&self, data: &[Vec<u8>]) -> Result<(), Self::Error>;
}
