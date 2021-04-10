use super::{FrameControl, MacAddress};
use crate::traits::{Addresses, HasHeader};

/// This struct represent a management frame header.
/// It's a pre-defined header format that is valid for all management frames!
///
/// **Bytes 0-1** \
/// These contain protocol meta information and flags. These have already been parsed!
/// Take a look at the [FrameControl] struct for more information.
///
/// **Bytes 2-3** \
/// Those are the duration bytes. These are always present!
/// They are quite specific and not explained here.
///
/// **Bytes 4-29** \
/// These contain all important address information.
///
/// byte 4-9: Address 1. Always present!
/// byte 10-15: Address 2.
/// byte 16-21: Address 3.
/// byte 22-23: Sequence Control.
///
/// Which address is used in which way, depends on a combination of
/// - two flags in the FrameControl header.
/// - the Type/Subtype constellation.
///
/// A rule of thumb is this:
///
/// **Address 1:** \
/// The recipient station address.
/// If `to_ds` is set, this is the AP address.
/// If `from_ds` is set then this is the station address
///
/// **Address 2:** \
/// The transmitter station address.
/// If `from_ds` is set, this is the AP address.
/// If `to_ds` is set then this is the station address.
///
/// **Address 3:** \
/// If Address 1 contains the destination address then Address 3 will contain the source address.
/// Similarly, if Address 2 contains the source address then Address 3 will contain the destination address.
///
/// **Sequence Control:** \
/// Contains the FragmentNumber and SequenceNumber that define the main frame and the number of fragments in the frame.
#[derive(Clone, Debug)]
pub struct ManagementHeader {
    pub frame_control: FrameControl,
    pub duration: [u8; 2],
    pub address_1: MacAddress,
    pub address_2: MacAddress,
    pub address_3: MacAddress,
    pub seq_ctl: [u8; 2],
}

impl<T: HasHeader> Addresses for T {
    /// Return the mac address of the sender
    fn src(&self) -> Option<&MacAddress> {
        let header = self.get_header();
        let frame_control = &header.frame_control;
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
    fn dest(&self) -> &MacAddress {
        let header = self.get_header();
        let frame_control = &header.frame_control;
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
    fn bssid(&self) -> Option<&MacAddress> {
        let header = self.get_header();
        let frame_control = &header.frame_control;
        if frame_control.to_ds() {
            Some(&header.address_1)
        } else if frame_control.from_ds() {
            Some(&header.address_2)
        } else {
            Some(&header.address_3)
        }
    }
}
