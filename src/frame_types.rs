#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum FrameType {
    Management,
    Control,
    Data,
    Unknown,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum FrameSubType {
    // Management subtypes
    AssoReq,
    AssoResp,
    ReassoReq,
    ReassoResp,
    ProbeReq,
    ProbeResp,
    Beacon,
    Atim,
    Disasso,
    Auth,
    Deauth,

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
