use std::io::Cursor;

use bytes::Buf;

use crate::components::*;
use crate::parsers::parse_header;
use crate::traits::*;
use crate::variants::extractors::supported_rates;

#[derive(Clone, Debug)]
pub struct ProbeRequest {
    pub header: Header,
    pub ssid: SSID,
    pub supported_rates: Vec<f32>,
}

impl ProbeRequest {
    pub fn parse(input: &[u8]) -> ProbeRequest {
        let (input, header) = parse_header(input).unwrap();
        let mut cursor = Cursor::new(input);

        let ssid = SSID::parse(cursor.bytes());
        cursor.advance(ssid.ssid_len + 2);

        ProbeRequest {
            header,
            ssid,
            supported_rates: supported_rates(cursor.bytes()),
        }
    }
}

impl HasHeader for ProbeRequest {
    fn get_header(&self) -> &Header {
        &self.header
    }
}
