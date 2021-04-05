use nom::number::complete::le_u16;
use nom::sequence::tuple;
use nom::IResult;

use crate::components::*;
use crate::parsers::{parse_header, parse_station_info};
use crate::traits::*;

#[derive(Clone, Debug)]
pub struct AssociationResponse {
    pub header: Header,
    pub capability_info: u16,
    pub status_code: u16,
    pub association_id: u16,
    pub station_info: StationInfo,
}

impl AssociationResponse {
    pub fn parse(input: &[u8]) -> IResult<&[u8], AssociationResponse> {
        let (input, (header, capability_info, status_code, association_id, station_info)) =
            tuple((parse_header, le_u16, le_u16, le_u16, parse_station_info))(input)?;

        Ok((
            input,
            AssociationResponse {
                header,
                capability_info,
                status_code,
                association_id,
                station_info,
            },
        ))
    }
}

impl HasHeader for AssociationResponse {
    fn get_header(&self) -> &Header {
        &self.header
    }
}
