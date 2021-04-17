use strum_macros::Display;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Display)]
pub enum FrameType {
    Management,
    Control,
    Data,
    Unknown,
}

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
    DataCfPull,
    DataCfAckCfPull,
    NullData,
    CfAck,
    CfPull,
    CfAckCfPull,
    QoS,
    QoSCfPull,
    QoSCfAckCfPull,
    QoSNullData,

    // Special subtypes
    Reserved,
    UnHandled,
}
