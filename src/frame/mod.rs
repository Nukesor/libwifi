pub mod control;
pub mod data;
pub mod management;

pub use control::*;
pub use data::*;
pub use management::*;

use crate::components::MacAddress;
use crate::traits::Addresses;

#[derive(Clone, Debug)]
/// This represents all currently supported payloads for frame subtypes.
/// Each variant is represented by its own struct.
/// The structs are organized in files by their frame type, i.e. `management`, `data`, `control`.
pub enum Frame {
    // Management frames
    Beacon(Beacon),
    ProbeRequest(ProbeRequest),
    ProbeResponse(ProbeResponse),
    AssociationRequest(AssociationRequest),
    AssociationResponse(AssociationResponse),

    // Control Frames
    Rts(Rts),
}

impl Frame {
    pub fn src(&self) -> Option<&MacAddress> {
        match &self {
            // Management
            Frame::Beacon(inner) => inner.src(),
            Frame::ProbeRequest(inner) => inner.src(),
            Frame::ProbeResponse(inner) => inner.src(),
            Frame::AssociationRequest(inner) => inner.src(),
            Frame::AssociationResponse(inner) => inner.src(),

            // Control
            Frame::Rts(inner) => inner.src(),
        }
    }

    pub fn dest(&self) -> &MacAddress {
        match &self {
            // Management
            Frame::Beacon(inner) => inner.dest(),
            Frame::ProbeRequest(inner) => inner.dest(),
            Frame::ProbeResponse(inner) => inner.dest(),
            Frame::AssociationRequest(inner) => inner.dest(),
            Frame::AssociationResponse(inner) => inner.dest(),

            // Control
            Frame::Rts(inner) => inner.dest(),
        }
    }

    pub fn bssid(&self) -> Option<&MacAddress> {
        match &self {
            // Management
            Frame::Beacon(inner) => inner.bssid(),
            Frame::ProbeRequest(inner) => inner.bssid(),
            Frame::ProbeResponse(inner) => inner.bssid(),
            Frame::AssociationRequest(inner) => inner.bssid(),
            Frame::AssociationResponse(inner) => inner.bssid(),

            // Control
            Frame::Rts(inner) => inner.bssid(),
        }
    }
}
