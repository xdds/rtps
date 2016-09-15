use super::ReaderInitArgs;

use super::super::traits::{ ReaderTrait, EndpointTrait };
use super::super::super::common_types::*;

pub struct StatelessReader {
    guid: Guid,
    unicast_locator_list: LocatorList,
    multicast_locator_list: LocatorList
}

impl StatelessReader {
    pub fn new(args: ReaderInitArgs) -> Self {
        StatelessReader {
            guid: args.guid,
            unicast_locator_list: args.unicast_locator_list,
            multicast_locator_list: args.multicast_locator_list
        }
    }
}

impl ReaderTrait for StatelessReader {
}

impl EndpointTrait for StatelessReader {
    fn topic_kind(&self) -> TopicKind {
        TopicKind::NO_KEY
    }

    fn unicast_locator_list(&self) -> &LocatorList {
        &self.unicast_locator_list
    }

    fn multicast_locator_list(&self) -> &LocatorList {
        &self.multicast_locator_list
    }
}