use nom::bytes::complete::take;
use nom::combinator::opt;
use nom::sequence::tuple;

use super::{clone_slice, parse_mac};
use crate::error::Error;
use crate::frame::components::{DataHeader, FrameControl, ManagementHeader};

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

/// Parse and return the [DataHeader] from a given payload.
pub fn parse_data_header(
    frame_control: FrameControl,
    input: &[u8],
) -> Result<(&[u8], DataHeader), Error> {
    let (mut remaining, (duration, address_1, address_2, address_3, seq_ctl)) =
        tuple((take(2usize), parse_mac, parse_mac, parse_mac, take(2usize)))(input)?;

    let duration = clone_slice::<2>(duration);
    let seq_ctl = clone_slice::<2>(seq_ctl);

    let mut address_4 = None;
    if frame_control.to_ds() && frame_control.from_ds() {
        (remaining, address_4) = opt(parse_mac)(remaining)?;
    };

    let mut qos = None;
    if frame_control.frame_subtype.is_qos() {
        let (_remaining, qos_bytes) = take(2usize)(remaining)?;
        qos = Some(clone_slice::<2>(qos_bytes));
        remaining = _remaining;
    }

    Ok((
        remaining,
        DataHeader {
            frame_control,
            duration,
            address_1,
            address_2,
            address_3,
            seq_ctl,
            address_4,
            qos,
        },
    ))
}
