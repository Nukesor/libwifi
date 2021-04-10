use crate::components::*;
use crate::traits::*;

#[derive(Clone, Debug)]
pub struct ProbeRequest {
    pub header: ManagementHeader,
    pub station_info: StationInfo,
}

impl HasHeader for ProbeRequest {
    fn get_header(&self) -> &ManagementHeader {
        &self.header
    }
}
