use nom::bytes::complete::take;
use nom::sequence::tuple;

use crate::components::FrameControl;
use crate::error::Error;
use crate::frame::*;
use crate::parsers::{clone_slice, parse_mac};

/// Parse an [AssociationRequest] frame.
///
/// The general structure is:
/// - FrameControl
/// - Duration
/// - Source
/// - Destination
pub fn parse_rts(frame_control: FrameControl, input: &[u8]) -> Result<Frame, Error> {
    let (_, (duration, address_1, address_2)) = tuple((take(2usize), parse_mac, parse_mac))(input)?;

    Ok(Frame::Rts(Rts {
        frame_control,
        duration: clone_slice::<2>(duration),
        source: address_2,
        destination: address_1,
    }))
}
