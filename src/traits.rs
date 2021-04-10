use crate::components::{FrameControl, MacAddress, ManagementHeader};

pub trait HasHeader {
    fn get_header(&self) -> &ManagementHeader;
}

pub trait Addresses {
    /// Returns the sender of the Frame.
    /// This isn't always send in every frame (e.g. CTS).
    fn src(&self, frame_control: &FrameControl) -> Option<&MacAddress>;

    fn dest(&self, frame_control: &FrameControl) -> &MacAddress;

    /// This isn't always send in every frame (e.g. RTS).
    fn bssid(&self, frame_control: &FrameControl) -> Option<&MacAddress>;
}

impl<T: HasHeader> Addresses for T {
    /// Return the mac address of the sender
    fn src(&self, frame_control: &FrameControl) -> Option<&MacAddress> {
        let header = self.get_header();
        if frame_control.to_ds() {
            Some(&header.address_3)
        } else if frame_control.from_ds() {
            Some(&header.address_1)
        } else {
            Some(&header.address_2)
        }
    }

    /// Return the mac address of the receiver.
    /// A full `ff:ff:..` usually indicates a undirected broadcast.
    fn dest(&self, frame_control: &FrameControl) -> &MacAddress {
        let header = self.get_header();
        if frame_control.to_ds() && frame_control.from_ds() {
            &header.address_3
        } else if frame_control.to_ds() {
            &header.address_2
        } else if frame_control.from_ds() {
            &header.address_3
        } else {
            &header.address_1
        }
    }

    /// The BSSID for this request.
    /// In most cases, this is expected to be present.
    /// The only time it's not, is in a wireless distributed system (WDS).
    fn bssid(&self, frame_control: &FrameControl) -> Option<&MacAddress> {
        let header = self.get_header();
        if frame_control.to_ds() {
            Some(&header.address_1)
        } else if frame_control.from_ds() {
            Some(&header.address_2)
        } else {
            Some(&header.address_3)
        }
    }
}
