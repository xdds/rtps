use super::super::super::common_types::*;
use super::EndpointTrait;

pub trait ReaderTrait : EndpointTrait {
    fn expects_inline_qos(&self) -> bool {
        false
    }

    fn heartbeat_response_delay(&self) -> Duration {
        Duration::new(1,0)
    }

    fn heartbeat_supression_duration(&self) -> Duration {
        Duration::new(1,0)
    }
}