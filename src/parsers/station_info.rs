use nom::bytes::complete::take;
use nom::number::complete::u8 as get_u8;
use nom::sequence::tuple;
use nom::IResult;

use crate::components::StationInfo;

pub fn parse_station_info(mut input: &[u8]) -> IResult<&[u8], StationInfo> {
    let mut response_info = StationInfo::default();

    let mut element_id;
    let mut length;
    let mut data;
    loop {
        (input, (element_id, length)) = tuple((get_u8, get_u8))(input)?;
        (input, data) = take(length)(input)?;

        match element_id {
            1 => response_info.supported_rates = parse_supported_rates(data),
            _ => {
                response_info.data.insert(element_id, data.to_vec());
            }
        };
    }
}

/// This is used in the ProbeResponse frame.
/// It indicates which transmission rates (in Mbps) are supported by this AP.
fn parse_supported_rates(input: &[u8]) -> Vec<f32> {
    let mut rates: Vec<f32> = Vec::new();
    for rate in input {
        match rate {
            0x82 => rates.push(1.0),
            0x84 => rates.push(2.0),
            0x8b => rates.push(5.5),
            0x0c => rates.push(6.0),
            0x12 => rates.push(9.0),
            0x96 => rates.push(11.0),
            0x18 => rates.push(12.0),
            0x24 => rates.push(18.0),
            0x2c => rates.push(22.0),
            0x30 => rates.push(24.0),
            0x42 => rates.push(33.0),
            0x48 => rates.push(36.0),
            0x60 => rates.push(48.0),
            0x6c => rates.push(54.0),
            _ => continue,
        }
    }

    rates
}
