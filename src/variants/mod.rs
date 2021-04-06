use nom::{
    error::{Error, ErrorKind},
    IResult,
};

pub mod association_request;
pub mod association_response;
pub mod beacon;
pub mod probe_request;
pub mod probe_response;

pub use association_request::AssociationRequest;
pub use association_response::AssociationResponse;
pub use beacon::Beacon;
pub use probe_request::ProbeRequest;
pub use probe_response::ProbeResponse;

use crate::components::FrameControl;
use crate::frame_types::*;

#[derive(Clone, Debug)]
/// This represents all currently supported payloads for various frame types/subtypes.
/// Each variant is represented by its own struct, which can be found in the [variants] module.
pub enum Payload {
    Beacon(Beacon),
    ProbeRequest(ProbeRequest),
    ProbeResponse(ProbeResponse),
    AssociationRequest(AssociationRequest),
    AssociationResponse(AssociationResponse),
}

impl Payload {
    pub fn parse<'a>(frame_control: &FrameControl, input: &'a [u8]) -> IResult<&'a [u8], Payload> {
        // For now, only management Frames are handled
        if !matches!(frame_control.frame_type, FrameType::Management) {
            return Err(nom::Err::Failure(Error::new(input, ErrorKind::IsNot)));
        }

        // Check which kind of frame sub-type we got
        let (input, payload) = match frame_control.frame_subtype {
            FrameSubType::Beacon => {
                let (input, beacon) = Beacon::parse(input)?;
                (input, Payload::Beacon(beacon))
            }
            FrameSubType::ProbeRequest => {
                let (input, request) = ProbeRequest::parse(input)?;
                (input, Payload::ProbeRequest(request))
            }
            FrameSubType::ProbeResponse => {
                let (input, response) = ProbeResponse::parse(input)?;
                (input, Payload::ProbeResponse(response))
            }
            FrameSubType::AssociationRequest => {
                let (input, request) = AssociationRequest::parse(input)?;
                (input, Payload::AssociationRequest(request))
            }
            FrameSubType::AssociationResponse => {
                let (input, response) = AssociationResponse::parse(input)?;
                (input, Payload::AssociationResponse(response))
            }
            _ => {
                return Err(nom::Err::Failure(Error::new(input, ErrorKind::IsNot)));
            }
        };

        Ok((input, payload))
    }
}
