use nom::bytes::complete::take;
use nom::number::complete::u8 as get_u8;
use nom::sequence::tuple;
use nom::IResult;

use crate::components::SSID;

pub fn parse_ssid(input: &[u8]) -> IResult<&[u8], SSID> {
    let (input, (element_id, ssid_len)) = tuple((get_u8, get_u8))(input)?;
    let (input, ssid) = take(ssid_len)(input)?;

    Ok((
        input,
        SSID {
            element_id,
            ssid_len: ssid_len as usize,
            value: String::from_utf8_lossy(ssid).to_string(),
        },
    ))
}
