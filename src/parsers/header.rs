use nom::bytes::complete::take;
use nom::sequence::tuple;
use nom::IResult;

use super::{clone_slice, parse_mac};
use crate::components::Header;

pub fn parse_header(input: &[u8]) -> IResult<&[u8], Header> {
    let (remaining, (duration, address_1, address_2, address_3, seq_ctl)) =
        tuple((take(2usize), parse_mac, parse_mac, parse_mac, take(2usize)))(input)?;

    let duration = clone_slice::<2>(duration);
    let seq_ctl = clone_slice::<2>(seq_ctl);

    Ok((
        remaining,
        Header {
            duration,
            address_1,
            address_2,
            address_3,
            seq_ctl: Some(seq_ctl),
            address_4: None,
        },
    ))
}
