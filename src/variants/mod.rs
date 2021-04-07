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
use crate::error::Error;
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
    pub fn parse(frame_control: &FrameControl, input: &[u8]) -> Result<Payload, Error> {
        // For now, only management Frames are handled
        if !matches!(frame_control.frame_type, FrameType::Management) {
            return Err(Error::UnhandledFrameSubtype(
                frame_control.clone(),
                input.to_vec(),
            ));
        }

        // Check which kind of frame sub-type we got
        let (_, payload) = match frame_control.frame_subtype {
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
                return Err(Error::UnhandledFrameSubtype(
                    frame_control.clone(),
                    input.to_vec(),
                ));
            }
        };

        Ok(payload)
    }
}
