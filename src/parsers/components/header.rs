use nom::bytes::complete::take;
use nom::sequence::tuple;

use super::{clone_slice, parse_mac};
use crate::frame::components::{FrameControl, ManagementHeader};
use crate::error::Error;

/// Parse and return the [ManagementHeader] from a given payload.
pub fn parse_management_header(
    frame_control: FrameControl,
    input: &[u8],
) -> Result<(&[u8], ManagementHeader), Error> {
    let (remaining, (duration, address_1, address_2, address_3, seq_ctl)) =
        tuple((take(2usize), parse_mac, parse_mac, parse_mac, take(2usize)))(input)?;

    let duration = clone_slice::<2>(duration);
    let seq_ctl = clone_slice::<2>(seq_ctl);

    Ok((
        remaining,
        ManagementHeader {
            frame_control,
            duration,
            address_1,
            address_2,
            address_3,
            seq_ctl,
        },
    ))
}
