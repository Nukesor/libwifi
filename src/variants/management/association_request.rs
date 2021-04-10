use crate::components::*;
use crate::traits::*;

#[derive(Clone, Debug)]
pub struct AssociationRequest {
    pub header: ManagementHeader,
    pub beacon_interval: u16,
    pub capability_info: u16,
    pub station_info: StationInfo,
}

impl HasHeader for AssociationRequest {
    fn get_header(&self) -> &ManagementHeader {
        &self.header
    }
}
