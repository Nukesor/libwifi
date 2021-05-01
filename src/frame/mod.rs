/// Contains structs representing recurring sets of structured data.
/// For instance, MAC-Addresses, default headers, etc.
pub mod components;
/// Control frame structs
mod control;
/// Data frames structs
mod data;
/// Management frame structs
mod management;

pub use control::*;
pub use data::*;
pub use management::*;

use self::components::MacAddress;
use crate::traits::Addresses;

#[derive(Clone, Debug)]
/// This represents all currently supported payloads for frame subtypes.
/// Each variant is represented by its own struct.
///
/// The structs are organized by their frame type in submodules.
pub enum Frame {
    // Management frames
    Beacon(Beacon),
    ProbeRequest(ProbeRequest),
    ProbeResponse(ProbeResponse),
    AssociationRequest(AssociationRequest),
    AssociationResponse(AssociationResponse),

    // Control Frames
    Rts(Rts),
    Cts(Cts),
    Ack(Ack),
    BlockAckRequest(BlockAckRequest),
    BlockAck(BlockAck),

    // Data Frames
    Data(Data),
    NullData(NullData),
    QosData(QosData),
    QosNull(QosNull),
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
            Frame::Cts(inner) => inner.src(),
            Frame::Ack(inner) => inner.src(),
            Frame::BlockAck(inner) => inner.src(),
            Frame::BlockAckRequest(inner) => inner.src(),

            // Data
            Frame::Data(inner) => inner.src(),
            Frame::NullData(inner) => inner.src(),
            Frame::QosData(inner) => inner.src(),
            Frame::QosNull(inner) => inner.src(),
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
            Frame::Cts(inner) => inner.dest(),
            Frame::Ack(inner) => inner.dest(),
            Frame::BlockAck(inner) => inner.dest(),
            Frame::BlockAckRequest(inner) => inner.dest(),

            // Data
            Frame::Data(inner) => inner.dest(),
            Frame::NullData(inner) => inner.dest(),
            Frame::QosData(inner) => inner.dest(),
            Frame::QosNull(inner) => inner.dest(),
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
            Frame::Cts(inner) => inner.bssid(),
            Frame::Ack(inner) => inner.bssid(),
            Frame::BlockAck(inner) => inner.bssid(),
            Frame::BlockAckRequest(inner) => inner.bssid(),

            // Data
            Frame::Data(inner) => inner.bssid(),
            Frame::NullData(inner) => inner.bssid(),
            Frame::QosData(inner) => inner.bssid(),
            Frame::QosNull(inner) => inner.bssid(),
        }
    }
}
