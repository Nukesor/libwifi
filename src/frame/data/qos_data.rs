use libwifi_macros::AddressHeader;

use crate::frame::components::*;

#[derive(Clone, Debug, AddressHeader)]
pub struct QosData {
    pub header: DataHeader,
    pub data: Vec<u8>,
}
