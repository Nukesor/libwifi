use crate::frame::components::*;
use crate::traits::*;

#[derive(Clone, Debug)]
pub struct AssociationResponse {
    pub header: ManagementHeader,
    pub capability_info: u16,
    pub status_code: u16,
    pub association_id: u16,
    pub station_info: StationInfo,
}

impl HasHeader for AssociationResponse {
    fn get_header(&self) -> &ManagementHeader {
        &self.header
    }
}
