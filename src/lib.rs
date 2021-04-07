#![feature(destructuring_assignment)]
/// Contains structs representing recurring sets of structured data.
/// For instance, MAC-Addresses, default headers, etc.
pub mod components;
pub mod error;
pub mod frame_types;
/// Contains [nom] parsers for internal usage
pub mod parsers;
pub mod traits;
pub mod variants;

use crate::components::{FrameControl, MacAddress};
use crate::error::Error;
use crate::parsers::parse_frame_control;
use crate::traits::Addresses;
use crate::variants::*;

/// This represents a full IEE 800.11 frame.
/// It's devided into the Header,
#[derive(Clone, Debug)]
pub struct Frame {
    pub control: FrameControl,
    pub payload: Payload,
    //frame_check_sequence: [u8; 4],
}

impl Frame {
    pub fn parse(input: &[u8]) -> Result<Frame, Error> {
        let (input, frame_control) = parse_frame_control(input)?;
        println!(
            "Type/Subtype: {:?}, {:?}",
            frame_control.frame_type, frame_control.frame_subtype
        );
        println!("Payload bytes: {:?}", &input);

        let (_, payload) = Payload::parse(&frame_control, input)?;

        Ok(Frame {
            control: frame_control,
            payload,
        })
    }
}

impl Frame {
    pub fn src(&self) -> Option<&MacAddress> {
        match &self.payload {
            Payload::Beacon(inner) => inner.src(&self.control),
            Payload::ProbeRequest(inner) => inner.src(&self.control),
            Payload::ProbeResponse(inner) => inner.src(&self.control),
            Payload::AssociationRequest(inner) => inner.src(&self.control),
            Payload::AssociationResponse(inner) => inner.src(&self.control),
        }
    }

    pub fn dest(&self) -> &MacAddress {
        match &self.payload {
            Payload::Beacon(inner) => inner.dest(&self.control),
            Payload::ProbeRequest(inner) => inner.dest(&self.control),
            Payload::ProbeResponse(inner) => inner.dest(&self.control),
            Payload::AssociationRequest(inner) => inner.dest(&self.control),
            Payload::AssociationResponse(inner) => inner.dest(&self.control),
        }
    }

    pub fn bssid(&self) -> Option<&MacAddress> {
        match &self.payload {
            Payload::Beacon(inner) => inner.bssid(&self.control),
            Payload::ProbeRequest(inner) => inner.bssid(&self.control),
            Payload::ProbeResponse(inner) => inner.bssid(&self.control),
            Payload::AssociationRequest(inner) => inner.bssid(&self.control),
            Payload::AssociationResponse(inner) => inner.bssid(&self.control),
        }
    }
}
