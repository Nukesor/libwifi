use crate::frame::components::{MacAddress, ManagementHeader};
use libwifi_macros::AddressHeader;

#[derive(Clone, Debug, AddressHeader)]
pub struct Deauthentication {
    pub header: ManagementHeader,
    pub reason_code: u16,
}