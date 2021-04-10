use nom::number::complete::{le_u16, le_u64};
use nom::sequence::tuple;

use crate::error::Error;
use crate::frame::*;
use crate::{
    components::FrameControl,
    parsers::{parse_management_header, parse_station_info},
};

pub fn parse_association_request(
    frame_control: FrameControl,
    input: &[u8],
) -> Result<Frame, Error> {
    let (input, header) = parse_management_header(frame_control, input)?;
    let (_, (beacon_interval, capability_info, station_info)) =
        tuple((le_u16, le_u16, parse_station_info))(input)?;

    Ok(Frame::AssociationRequest(AssociationRequest {
        header,
        beacon_interval,
        capability_info,
        station_info,
    }))
}

pub fn parse_association_response(
    frame_control: FrameControl,
    input: &[u8],
) -> Result<Frame, Error> {
    let (input, header) = parse_management_header(frame_control, input)?;
    let (_, (capability_info, status_code, association_id, station_info)) =
        tuple((le_u16, le_u16, le_u16, parse_station_info))(input)?;

    Ok(Frame::AssociationResponse(AssociationResponse {
        header,
        capability_info,
        status_code,
        association_id,
        station_info,
    }))
}

pub fn parse_beacon(frame_control: FrameControl, input: &[u8]) -> Result<Frame, Error> {
    let (input, header) = parse_management_header(frame_control, input)?;
    let (_, (timestamp, beacon_interval, capability_info, station_info)) =
        tuple((le_u64, le_u16, le_u16, parse_station_info))(input)?;

    Ok(Frame::Beacon(Beacon {
        header,
        timestamp,
        beacon_interval,
        capability_info,
        station_info,
    }))
}

pub fn parse_probe_request(frame_control: FrameControl, input: &[u8]) -> Result<Frame, Error> {
    let (input, header) = parse_management_header(frame_control, input)?;
    let (_, station_info) = parse_station_info(input)?;

    Ok(Frame::ProbeRequest(ProbeRequest {
        header,
        station_info,
    }))
}

pub fn parse_probe_response(frame_control: FrameControl, input: &[u8]) -> Result<Frame, Error> {
    let (input, header) = parse_management_header(frame_control, input)?;
    let (_, (timestamp, beacon_interval, capability_info, station_info)) =
        tuple((le_u64, le_u16, le_u16, parse_station_info))(input)?;

    Ok(Frame::ProbeResponse(ProbeResponse {
        header,
        timestamp,
        beacon_interval,
        capability_info,
        station_info,
    }))
}
