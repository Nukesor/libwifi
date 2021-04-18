use crate::frame::components::{FrameControl, MacAddress};
use crate::Addresses;

#[derive(Clone, Debug)]
pub struct Rts {
    pub frame_control: FrameControl,
    pub duration: [u8; 2],
    pub source: MacAddress,
    pub destination: MacAddress,
}

impl Addresses for Rts {
    fn src(&self) -> Option<&MacAddress> {
        Some(&self.source)
    }

    fn dest(&self) -> &MacAddress {
        &self.destination
    }

    fn bssid(&self) -> Option<&MacAddress> {
        None
    }
}
