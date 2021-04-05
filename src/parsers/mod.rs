use nom::bytes::complete::take;
use nom::IResult;

use crate::components::MacAddress;

pub mod frame_control;
pub mod header;

pub use frame_control::parse_frame_control;
pub use header::parse_header;

pub fn parse_mac(input: &[u8]) -> IResult<&[u8], MacAddress> {
    let (remaining, bytes) = take(6usize)(input)?;
    let mut address: [u8; 6] = [0; 6];
    address.clone_from_slice(&bytes[0..6]);

    Ok((remaining, MacAddress(address)))
}
