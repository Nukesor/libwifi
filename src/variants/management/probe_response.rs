use nom::number::complete::{le_u16, le_u64};
use nom::sequence::tuple;
use nom::IResult;

use crate::components::*;
use crate::parsers::{parse_management_header, parse_station_info};
use crate::traits::*;

#[derive(Clone, Debug)]
pub struct ProbeResponse {
    pub header: ManagementHeader,
    pub timestamp: u64,
    pub beacon_interval: u16,
    pub capability_info: u16,
    pub station_info: StationInfo,
}

impl ProbeResponse {
    pub fn parse(input: &[u8]) -> IResult<&[u8], ProbeResponse> {
        let (input, (header, timestamp, beacon_interval, capability_info, station_info)) =
            tuple((parse_management_header, le_u64, le_u16, le_u16, parse_station_info))(input)?;

        Ok((
            input,
            ProbeResponse {
                header,
                timestamp,
                beacon_interval,
                capability_info,
                station_info,
            },
        ))
    }
}

impl HasHeader for ProbeResponse {
    fn get_header(&self) -> &ManagementHeader {
        &self.header
    }
}
