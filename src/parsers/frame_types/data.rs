use crate::error::Error;
use crate::frame::components::FrameControl;
use crate::frame::*;
use crate::parsers::parse_data_header;

/// Parse a [Frame::Rts] frame.
///
/// The general structure is:
/// - FrameControl
/// - Duration
/// - Source
/// - Destination
pub fn parse_data(frame_control: FrameControl, input: &[u8]) -> Result<Frame, Error> {
    let (_remaining, header) = parse_data_header(frame_control, input)?;

    Ok(Frame::Data(Data { header }))
}
