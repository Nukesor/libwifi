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

/// This contains helper functions that are used to interpret and extract information from a byte
/// array. These should only be used internally.
mod extractors;

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
    UnHandled(bool),
    Empty,
}

impl Payload {
    pub fn parse(frame_control: &FrameControl, input: &[u8]) -> Payload {
        // For now, only management Frames are handled
        if !matches!(frame_control.frame_type, FrameType::Management) {
            return Payload::UnHandled(true);
        }

        // Check which kind of frame sub-type we got
        match frame_control.frame_subtype {
            FrameSubType::Beacon => Payload::Beacon(Beacon::parse(input)),
            FrameSubType::ProbeReq => Payload::ProbeRequest(ProbeRequest::parse(input)),
            FrameSubType::ProbeResp => Payload::ProbeResponse(ProbeResponse::parse(input)),
            FrameSubType::AssoReq => Payload::AssociationRequest(AssociationRequest::parse(input)),
            FrameSubType::AssoResp => {
                Payload::AssociationResponse(AssociationResponse::parse(input))
            }
            _ => Payload::UnHandled(true),
        }
    }
}
