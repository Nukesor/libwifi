use nom::sequence::tuple;
use nom::IResult;

use crate::components::*;
use crate::parsers::{parse_header, parse_ssid, parse_station_info};
use crate::traits::*;

#[derive(Clone, Debug)]
pub struct ProbeRequest {
    pub header: Header,
    pub ssid: Ssid,
    pub station_info: StationInfo,
}

impl ProbeRequest {
    pub fn parse(input: &[u8]) -> IResult<&[u8], ProbeRequest> {
        let (input, (header, ssid, station_info)) =
            tuple((parse_header, parse_ssid, parse_station_info))(input)?;

        Ok((
            input,
            ProbeRequest {
                header,
                ssid,
                station_info,
            },
        ))
    }
}

impl HasHeader for ProbeRequest {
    fn get_header(&self) -> &Header {
        &self.header
    }
}
