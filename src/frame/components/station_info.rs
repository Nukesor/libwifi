use std::collections::BTreeMap;

#[derive(Clone, Debug, Default)]
/// StationInfo is used to parse and store variable length fields that are often sent
/// with management frames.
///
/// Each field has an `id`, the length of the bytes for this field, and then payload of the field.
/// Since there's a large number of possible fields and many propriatary vendor-specific usages
/// of these fields, this generic solution is used to capture all of them.
///
/// It is also important to note that most of these fields won't be sent most of the time. \
/// All fields that are already handled by this library get their own field in the StationInfo
/// struct.
///
/// Since we cannot handle all all those elements, the bytes of all unhandled elements will
/// be saved in the `data` field under the respectiv element id.
pub struct StationInfo {
    /// The transmission rates that are supported by the station.
    /// Empty if no rates were transmitted.
    pub supported_rates: Vec<f32>,
    /// If the sender included a SSID, it will be in here.
    pub ssid: Option<String>,
    /// This map contains all fields that aren't explicitly parsed by us.
    /// The format is Vec<(FieldId, PayloadBytes)>.
    ///
    /// Please consider to create a PR, if you write a parser for a new field :).
    pub data: Vec<(u8, Vec<u8>)>,
}
