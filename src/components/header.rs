use super::mac::*;

/// This struct tries to represent a "normal" frame header to the best of it's abilities.
///
/// The IEE 802.11 frame specification is quite weird.
/// The formats vary wildly, the size can span from 10 bytes to 34 bytes and the way all of this is
/// interpreted depends on various flags, types.
///
/// This struct is designed to represent the full header!
/// The following is the aproximated format of a full header.
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
/// byte 10-15: Address 2. May be missing.
/// byte 16-21: Address 3. May be missing.
/// byte 22-23: Sequence Control. May be missing.
/// byte 24-29: Address 4. May be missing.
///
/// Which address is used in which way, depends on a combination of
/// - two flags in the FrameControl header.
/// - the Type/Subtype constellation.
///
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
///
/// **Address 4:** \
/// This is only set if a Wireless Distribution System (WDS) is being used (with AP to AP communication)
/// Address 1 contains the receiving AP address.
/// Address 2 contains the transmitting AP address.
/// Address 3 contains the destination station address.
/// Address 4 contains the source station address.
#[derive(Clone, Debug)]
pub struct Header {
    pub duration: [u8; 2],
    pub address_1: MacAddress,
    pub address_2: MacAddress,
    pub address_3: MacAddress,
    pub seq_ctl: Option<[u8; 2]>,
    pub address_4: Option<MacAddress>,
}
