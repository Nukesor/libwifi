use std::io::Cursor;

use bytes::{Buf, Bytes};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct SSID {
    pub element_id: u8,
    pub ssid_len: usize,
    pub value: String,
}

impl SSID {
    pub fn parse(input: &[u8]) -> SSID {
        let mut cursor = Cursor::new(input);

        let element_id = cursor.get_u8();
        let ssid_len = cursor.get_u8() as usize;
        let mut buf = Bytes::from(cursor.bytes());
        let ssid = buf.split_to(ssid_len);

        SSID {
            element_id,
            ssid_len,
            value: String::from_utf8(ssid.to_vec()).unwrap_or_else(|_| "".to_string()),
        }
    }
}
