use std::io::Cursor;

use bytes::{Buf, Bytes};

#[derive(Clone, Debug, Default)]
pub struct AdditionalInfo {
    pub country: Country,
    pub current_channel: u8,
}

#[derive(Clone, Debug, Default)]
pub struct Country {
    pub country_code: String,
}

impl Country {
    fn parse(input: &[u8]) -> Country {
        let mut buf = Bytes::from(input);
        let country_code = buf.split_to(3); // Country code has 3 bytes
                                            // We should include the supported channels
        Country {
            country_code: String::from_utf8(country_code.to_vec())
                .unwrap_or_else(|_| "".to_string()),
        }
    }
}

impl AdditionalInfo {
    pub fn new() -> AdditionalInfo {
        let country = Country {
            ..Default::default()
        };

        AdditionalInfo {
            country,
            current_channel: 0,
        }
    }
}

pub fn get_info(input: &[u8]) -> AdditionalInfo {
    let mut cursor = Cursor::new(input);
    let mut info = AdditionalInfo::new();

    loop {
        let element_id = cursor.get_u8();
        let len = cursor.get_u8() as usize;

        // Skipping some fields
        match element_id {
            0x02 => cursor.advance(len), // FH Parameter Set
            0x03 => {
                // DS Parameter Set
                info.current_channel = cursor.get_u8();
            }
            0x04 => cursor.advance(len), // CF Parameter Set
            0x05 => cursor.advance(len), // TIM
            0x06 => cursor.advance(len), // IBSS
            0x07 => {
                info.country = Country::parse(cursor.bytes());
                cursor.advance(len);
            }
            0x32..=0x42 => cursor.advance(len), // Can appear before country
            _ => {
                break;
            }
        }
    }

    info
}
