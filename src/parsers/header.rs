use nom::bytes::complete::take;
use nom::combinator::opt;
use nom::sequence::tuple;
use nom::IResult;

use super::{clone_slice, parse_mac};
use crate::components::Header;

pub fn parse_header(input: &[u8]) -> IResult<&[u8], Header> {
    let (remaining, (duration, address_1, address_2, address_3, seq_ctl, address_4)) =
        tuple((
            take(2usize),
            parse_mac,
            parse_mac,
            parse_mac,
            opt(take(2usize)),
            opt(parse_mac),
        ))(input)?;

    let duration = clone_slice::<2>(duration);

    let seq_ctl = if let Some(seq_ctl) = seq_ctl {
        Some(clone_slice::<2>(seq_ctl))
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
