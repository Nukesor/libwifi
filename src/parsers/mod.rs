use nom::bytes::complete::take;
use nom::IResult;

use crate::components::MacAddress;

pub mod frame_control;
pub mod header;
pub mod ssid;
pub mod station_info;

pub use frame_control::parse_frame_control;
pub use header::parse_header;
pub use ssid::parse_ssid;
pub use station_info::parse_station_info;

pub fn parse_mac(input: &[u8]) -> IResult<&[u8], MacAddress> {
    let (remaining, bytes) = take(6usize)(input)?;
    Ok((remaining, MacAddress(clone_slice::<6>(bytes))))
}

pub(crate) fn clone_slice<const X: usize>(slice: &[u8]) -> [u8; X] {
    let mut cloned_slice: [u8; X] = [0; X];
    cloned_slice.copy_from_slice(&slice[0..X]);

    cloned_slice
}
