use std::collections::BTreeMap;

#[derive(Clone, Debug, Default)]
/// This struct contains all information provided in a ProbeResponse.
/// There are a lot of different elements that can be sent and only at most times, only
/// a few of those will be sent.
///
/// Since we cannot handle all all those elements, all unhandled elements will be saved
/// in the `data` field under the respectiv element id.
pub struct StationInfo {
    pub supported_rates: Vec<f32>,
    pub ssid: Option<String>,
    /// ProbeResponseInfos
    pub data: BTreeMap<u8, Vec<u8>>,
}
