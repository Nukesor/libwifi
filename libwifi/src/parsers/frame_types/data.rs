use crate::error::Error;
use crate::frame::components::FrameControl;
use crate::frame::*;
use crate::parsers::parse_data_header;

/// Parse a [Data] frame.
pub fn parse_data(frame_control: FrameControl, input: &[u8]) -> Result<Frame, Error> {
    let (remaining, header) = parse_data_header(frame_control, input)?;

    Ok(Frame::Data(Data {
        header,
        data: remaining.into(),
    }))
}

/// Parse a [NullData] frame.
pub fn parse_null_data(frame_control: FrameControl, input: &[u8]) -> Result<Frame, Error> {
    let (_, header) = parse_data_header(frame_control, input)?;

    Ok(Frame::NullData(NullData { header }))
}

/// Parse a [QosData] frame.
pub fn parse_qos_data(frame_control: FrameControl, input: &[u8]) -> Result<Frame, Error> {
    let (remaining, header) = parse_data_header(frame_control, input)?;

    Ok(Frame::QosData(QosData {
        header,
        data: remaining.into(),
    }))
}

/// Parse a [QosNull] frame.
pub fn parse_qos_null(frame_control: FrameControl, input: &[u8]) -> Result<Frame, Error> {
    let (_, header) = parse_data_header(frame_control, input)?;

    Ok(Frame::QosNull(QosNull { header }))
}
