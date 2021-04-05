#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Ssid {
    pub element_id: u8,
    pub ssid_len: usize,
    pub value: String,
}
