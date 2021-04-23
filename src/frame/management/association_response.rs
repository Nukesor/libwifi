use libwifi_macros::AddressHeader;

use crate::frame::components::*;

#[derive(Clone, Debug, AddressHeader)]
pub struct AssociationResponse {
    pub header: ManagementHeader,
    pub capability_info: u16,
    pub status_code: u16,
    pub association_id: u16,
    pub station_info: StationInfo,
}
