use nom::number::complete::{le_u16, le_u64};
use nom::sequence::tuple;
use nom::IResult;

use crate::components::*;
use crate::parsers::{parse_header, parse_ssid, parse_station_info};
use crate::traits::*;

#[derive(Clone, Debug)]
pub struct Beacon {
    pub header: Header,
    pub timestamp: u64,
    pub beacon_interval: u16,
    pub capability_info: u16,
    pub ssid: Ssid,
    pub station_info: StationInfo,
}

impl Beacon {
    pub fn parse(input: &[u8]) -> IResult<&[u8], Beacon> {
        let (input, (header, timestamp, beacon_interval, capability_info, ssid, station_info)) =
            tuple((
                parse_header,
                le_u64,
                le_u16,
                le_u16,
                parse_ssid,
                parse_station_info,
            ))(input)?;

        Ok((
            input,
            Beacon {
                header,
                timestamp,
                beacon_interval,
                capability_info,
                ssid,
                station_info,
            },
        ))
    }
}

impl HasHeader for Beacon {
    fn get_header(&self) -> &Header {
        &self.header
    }
}
