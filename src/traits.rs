use crate::components::{MacAddress, ManagementHeader};

pub trait HasHeader {
    fn get_header(&self) -> &ManagementHeader;
}

pub trait Addresses {
    /// Returns the sender of the Frame.
    /// This isn't always send in every frame (e.g. CTS).
    fn src(&self) -> Option<&MacAddress>;

    fn dest(&self) -> &MacAddress;

    /// This isn't always send in every frame (e.g. RTS).
    fn bssid(&self) -> Option<&MacAddress>;
}
