use nom::bytes::complete::take;
use nom::number::complete::u8 as get_u8;
use nom::sequence::tuple;
use nom::IResult;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct SSID {
    pub element_id: u8,
    pub ssid_len: usize,
    pub value: String,
}

impl SSID {
    pub fn parse(input: &[u8]) -> IResult<&[u8], SSID> {
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
}
