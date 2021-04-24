use strum_macros::Display;

/// Representation of all possible the frame types.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Display)]
pub enum FrameType {
    Management,
    Control,
    Data,
    Unknown,
}

/// Representation of the exact frame subtypes.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Display)]
pub enum FrameSubType {
    // Management subtypes
    AssociationRequest,
    AssociationResponse,
    ReassociationRequest,
    ReassociationResponse,
    ProbeRequest,
    ProbeResponse,
    Beacon,
    Atim,
    Disassociation,
    Authentication,
    Deauthentication,

    // Control subtypes
    Trigger,
    Tack,
    BeamformingReportPoll,
    NdpAnnouncement,
    ControlFrameExtension,
    ControlWrapper,
    BlockAckRequest,
    BlockAck,
    PsPoll,
    Rts,
    Cts,
    Ack,
    CfEnd,
    CfEndCfAck,

    // Data subtypes
    Data,
    DataCfAck,
    DataCfPoll,
    DataCfAckCfPoll,
    NullData,
    CfAck,
    CfPoll,
    CfAckCfPoll,
    QosData,
    QosDataCfAck,
    QosDataCfPoll,
    QosDataCfAckCfPoll,
    QosNull,
    QosCfPoll,
    QosCfAckCfPoll,

    // Special subtypes
    Reserved,
    UnHandled,
}

impl FrameSubType {
    pub fn is_qos(&self) -> bool {
        match self {
            FrameSubType::QosData
            | FrameSubType::QosDataCfAck
            | FrameSubType::QosDataCfPoll
            | FrameSubType::QosDataCfAckCfPoll
            | FrameSubType::QosNull
            | FrameSubType::QosCfPoll
            | FrameSubType::QosCfAckCfPoll => true,
            _ => false,
        }
    }
}
