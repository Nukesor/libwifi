use std::io::Cursor;

use bytes::Buf;

use crate::components::*;
use crate::parsers::parse_header;
use crate::traits::*;
use crate::variants::extractors::supported_rates;

#[derive(Clone, Debug)]
pub struct AssociationResponse {
    pub header: Header,
    pub cap_info: u16,
    pub status_code: u16,
    pub association_id: u16,
    pub supported_rates: Vec<f32>,
}

impl AssociationResponse {
    pub fn parse(input: &[u8]) -> AssociationResponse {
        let (input, header) = parse_header(input).unwrap();
        let mut cursor = Cursor::new(input);

        let cap_info = cursor.get_u16_le();
        let status_code = cursor.get_u16_le();
        let association_id = cursor.get_u16_le();

        AssociationResponse {
            header,
            cap_info,
            status_code,
            association_id,
            supported_rates: supported_rates(cursor.bytes()),
        }
    }
}

impl HasHeader for AssociationResponse {
    fn get_header(&self) -> &Header {
        &self.header
    }
}
