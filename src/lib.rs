#![feature(destructuring_assignment)]
/// Contains structs representing recurring sets of structured data.
/// For instance, MAC-Addresses, default headers, etc.
pub mod components;
pub mod error;
pub mod frame;
pub mod frame_types;
/// Contains [nom] parsers for internal usage
pub mod parsers;
pub mod traits;

use crate::error::Error;
use crate::frame::*;
use crate::frame_types::*;
use crate::parsers::*;

pub fn parse(input: &[u8]) -> Result<Frame, Error> {
    let (input, frame_control) = parse_frame_control(input)?;
    //println!(
    //    "Type/Subtype: {:?}, {:?}",
    //    frame_control.frame_type, frame_control.frame_subtype
    //);
    //println!("Payload bytes: {:?}", &input);

    // For now, only management Frames are handled
    if !matches!(frame_control.frame_type, FrameType::Management) {
        return Err(Error::UnhandledFrameSubtype(frame_control, input.to_vec()));
    }

    // Check which kind of frame sub-type we got
    match frame_control.frame_subtype {
        FrameSubType::Beacon => parse_beacon(frame_control, input),
        FrameSubType::ProbeRequest => parse_probe_request(frame_control, input),
        FrameSubType::ProbeResponse => parse_probe_response(frame_control, input),
        FrameSubType::AssociationRequest => parse_association_request(frame_control, input),
        FrameSubType::AssociationResponse => parse_association_response(frame_control, input),
        _ => Err(Error::UnhandledFrameSubtype(frame_control, input.to_vec())),
    }
}
