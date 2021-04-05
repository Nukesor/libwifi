use nom::bytes::complete::take;
use nom::combinator::opt;
use nom::sequence::tuple;
use nom::IResult;

use super::parse_mac;
use crate::components::Header;

pub fn parse_header(input: &[u8]) -> IResult<&[u8], Header> {
    let (remaining, (duration_slice, address_1, address_2, address_3, seq_ctl_slice, address_4)) =
        tuple((
            take(2usize),
            parse_mac,
            parse_mac,
            parse_mac,
            opt(take(2usize)),
            opt(parse_mac),
        ))(input)?;

    let mut duration: [u8; 2] = Default::default();
    duration.copy_from_slice(&duration_slice[0..2]);

    let seq_ctl = if let Some(slice) = seq_ctl_slice {
        let mut seq_ctl: [u8; 2] = Default::default();
        seq_ctl.copy_from_slice(&slice[0..2]);
        Some(seq_ctl)
    } else {
        None
    };

    Ok((
        remaining,
        Header {
            duration,
            address_1,
            address_2,
            address_3,
            seq_ctl,
            address_4,
        },
    ))
}
