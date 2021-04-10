use crate::components::*;
use crate::traits::*;

#[derive(Clone, Debug)]
pub struct Beacon {
    pub header: ManagementHeader,
    pub timestamp: u64,
    pub beacon_interval: u16,
    pub capability_info: u16,
    pub station_info: StationInfo,
}

impl HasHeader for Beacon {
    fn get_header(&self) -> &ManagementHeader {
        &self.header
    }
}
