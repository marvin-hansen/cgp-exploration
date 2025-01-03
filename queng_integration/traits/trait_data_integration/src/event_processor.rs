use cgp::prelude::*;

#[cgp_component {provider: EventProcessorProvider,}]
#[async_trait]
pub trait CanProcessEvent: Async + HasErrorType {
    async fn process_event(&self, data: &[Vec<u8>]) -> Result<(), Self::Error>;
}
