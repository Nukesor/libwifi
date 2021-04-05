#![feature(destructuring_assignment)]
use nom::IResult;

/// Contains structs representing recurring sets of structured data.
/// For instance, MAC-Addresses, default headers, etc.
pub mod components;
pub mod frame_types;
/// Contains [nom] parsers for internal usage
pub mod parsers;
pub mod traits;
pub mod variants;

use crate::components::{FrameControl, MacAddress};
use crate::parsers::parse_frame_control;
use crate::traits::Addresses;
use crate::variants::*;

/// This represents a full IEE 800.11 frame.
/// It's devided into the Header,
pub struct Frame {
    pub control: FrameControl,
    pub payload: Payload,
    //frame_check_sequence: [u8; 4],
}

impl Frame {
    pub fn parse(input: &[u8]) -> IResult<&[u8], Frame> {
        let (input, frame_control) = parse_frame_control(&input)?;
        println!(
            "Type/Subtype: {:?}, {:?}",
            frame_control.frame_type, frame_control.frame_subtype
        );
        println!("Payload bytes: {:?}", &input);

        let (input, payload) = Payload::parse(&frame_control, &input)?;

        Ok((
            input,
            Frame {
                control: frame_control,
                payload,
            },
        ))
    }
}

impl Frame {
    pub fn src(&self) -> Option<&MacAddress> {
        match &self.payload {
            Payload::Beacon(beacon) => beacon.src(&self.control),
            _ => None,
        }
    }
}
