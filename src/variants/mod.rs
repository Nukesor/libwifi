pub mod management;

pub use management::*;

use crate::components::MacAddress;
use crate::traits::Addresses;

#[derive(Clone, Debug)]
/// This represents all currently supported payloads for various frame types/subtypes.
/// Each variant is represented by its own struct, which can be found in the [variants] module.
pub enum Frame {
    // Management frames
    Beacon(Beacon),
    ProbeRequest(ProbeRequest),
    ProbeResponse(ProbeResponse),
    AssociationRequest(AssociationRequest),
    AssociationResponse(AssociationResponse),
}

impl Frame {
    pub fn src(&self) -> Option<&MacAddress> {
        match &self {
            Frame::Beacon(inner) => inner.src(),
            Frame::ProbeRequest(inner) => inner.src(),
            Frame::ProbeResponse(inner) => inner.src(),
            Frame::AssociationRequest(inner) => inner.src(),
            Frame::AssociationResponse(inner) => inner.src(),
        }
    }

    pub fn dest(&self) -> &MacAddress {
        match &self {
            Frame::Beacon(inner) => inner.dest(),
            Frame::ProbeRequest(inner) => inner.dest(),
            Frame::ProbeResponse(inner) => inner.dest(),
            Frame::AssociationRequest(inner) => inner.dest(),
            Frame::AssociationResponse(inner) => inner.dest(),
        }
    }

    pub fn bssid(&self) -> Option<&MacAddress> {
        match &self {
            Frame::Beacon(inner) => inner.bssid(),
            Frame::ProbeRequest(inner) => inner.bssid(),
            Frame::ProbeResponse(inner) => inner.bssid(),
            Frame::AssociationRequest(inner) => inner.bssid(),
            Frame::AssociationResponse(inner) => inner.bssid(),
        }
    }
}
