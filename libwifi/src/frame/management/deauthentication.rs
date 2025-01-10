use libwifi_macros::AddressHeader;

use crate::frame::components::*;

#[derive(Clone, Debug, AddressHeader)]
pub struct Deauthentication {
    pub header: ManagementHeader,
    pub reason_code: u16,
}
