use libwifi_macros::AddressHeader;

use crate::frame::components::*;

#[derive(Clone, Debug, AddressHeader)]
pub struct AssociationRequest {
    pub header: ManagementHeader,
    pub beacon_interval: u16,
    pub capability_info: u16,
    pub station_info: StationInfo,
}
