use crate::frame::components::{FrameControl, MacAddress};
use crate::Addresses;

#[derive(Clone, Debug)]
pub struct Ack {
    pub frame_control: FrameControl,
    pub duration: [u8; 2],
    pub destination: MacAddress,
}

impl Addresses for Ack {
    fn src(&self) -> Option<&MacAddress> {
        None
    }

    fn dest(&self) -> &MacAddress {
        &self.destination
    }

    fn bssid(&self) -> Option<&MacAddress> {
        None
    }
}
