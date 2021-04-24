#![feature(destructuring_assignment)]
/// Libwifi's own [Error](error::Error) implementation
pub mod error;
/// The [Frame](frame::Frame) enum and all structs that represent each type of possible frame.
pub mod frame;
/// Enums that represent all frame types and frame sub-types.
mod frame_types;
/// Contains [nom] parsers for internal usage.
mod parsers;
/// Contains all traits provided by this library.
mod traits;

use crate::error::Error;
use crate::parsers::*;

// Re-exports for user convenience
pub use crate::frame::Frame;
pub use crate::frame_types::*;
pub use crate::traits::*;

/// This is the main function for IEE 802.11 parsing.
pub fn parse_frame(input: &[u8]) -> Result<Frame, Error> {
    let (input, frame_control) = parse_frame_control(input)?;
    //println!(
    //    "Type/Subtype: {:?}, {:?}",
    //    frame_control.frame_type, frame_control.frame_subtype
    //);
    //println!("Payload bytes: {:?}", &input);

    // Check which kind of frame sub-type we got
    match frame_control.frame_subtype {
        // Management
        FrameSubType::Beacon => parse_beacon(frame_control, input),
        FrameSubType::ProbeRequest => parse_probe_request(frame_control, input),
        FrameSubType::ProbeResponse => parse_probe_response(frame_control, input),
        FrameSubType::AssociationRequest => parse_association_request(frame_control, input),
        FrameSubType::AssociationResponse => parse_association_response(frame_control, input),

        // Control
        FrameSubType::Rts => parse_rts(frame_control, input),
        FrameSubType::Cts => parse_cts(frame_control, input),

        // Data
        FrameSubType::Data => parse_data(frame_control, input),
        FrameSubType::QosData => parse_qos_data(frame_control, input),
        _ => Err(Error::UnhandledFrameSubtype(frame_control, input.to_vec())),
    }
}
