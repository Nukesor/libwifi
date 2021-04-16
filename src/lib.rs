#![feature(destructuring_assignment)]
/// Contains structs representing recurring sets of structured data.
/// For instance, MAC-Addresses, default headers, etc.
pub mod components;
/// Libwifi's own [Error](error::Error) implementation
pub mod error;
/// Libwifi's own [Error](error::Error) implementation
pub mod frame;
mod frame_types;
/// Contains [nom] parsers for internal usage
mod parsers;
/// Contains all traits provided by this library
mod traits;

use crate::error::Error;
use crate::parsers::*;

// Re-exports for user convenience
pub use crate::frame::Frame;
pub use crate::frame_types::*;
pub use crate::traits::*;

pub fn parse(input: &[u8]) -> Result<Frame, Error> {
    let (input, frame_control) = parse_frame_control(input)?;
    //println!(
    //    "Type/Subtype: {:?}, {:?}",
    //    frame_control.frame_type, frame_control.frame_subtype
    //);
    //println!("Payload bytes: {:?}", &input);

    // Check which kind of frame sub-type we got
    match frame_control.frame_subtype {
        FrameSubType::Beacon => parse_beacon(frame_control, input),
        FrameSubType::ProbeRequest => parse_probe_request(frame_control, input),
        FrameSubType::ProbeResponse => parse_probe_response(frame_control, input),
        FrameSubType::AssociationRequest => parse_association_request(frame_control, input),
        FrameSubType::AssociationResponse => parse_association_response(frame_control, input),
        FrameSubType::Rts => parse_rts(frame_control, input),
        _ => Err(Error::UnhandledFrameSubtype(frame_control, input.to_vec())),
    }
}
