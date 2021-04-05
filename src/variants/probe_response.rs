use std::io::Cursor;

use bytes::Buf;

use crate::components::*;
use crate::parsers::parse_header;
use crate::traits::*;
use crate::variants::extractors::*;

#[derive(Clone, Debug)]
pub struct ProbeResponse {
    pub header: Header,
    pub timestamp: u64,
    pub interval: u16,
    pub cap_info: u16,
    pub ssid: SSID,
    pub supported_rates: Vec<f32>,
    pub current_channel: u8,
    pub country: Country,
}

impl ProbeResponse {
    pub fn parse(input: &[u8]) -> ProbeResponse {
        let (input, header) = parse_header(input).unwrap();
        let mut cursor = Cursor::new(input);

        let timestamp = cursor.get_u64_le();
        let interval = cursor.get_u16_le();
        let cap_info = cursor.get_u16_le();

        let ssid = SSID::parse(cursor.bytes());
        cursor.advance(ssid.ssid_len + 2); // 2 accounts for Id + Len
        let supported_rates = supported_rates(cursor.bytes());
        cursor.advance(supported_rates.len() + 2); // 2 accounts for Id + Len
        let info = get_info(cursor.bytes());

        ProbeResponse {
            header,
            timestamp,
            interval,
            cap_info,
            ssid,
            supported_rates,
            current_channel: info.current_channel,
            country: info.country,
        }
    }
}

impl HasHeader for ProbeResponse {
    fn get_header(&self) -> &Header {
        &self.header
    }
}
