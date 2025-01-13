# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.4.0] - 2024-01-13

This release encompasses a huge amount of upstreamed changes from @Ragnt, which vendored `libwifi` in [AngryOxide](https://github.com/Ragnt/AngryOxide) and continued development over there.

@Ragnt added a vast amount of additional features and parsers for data structures.
They allowed the upstream of their changes and republished their vendored library under a permissable license [over here](https://github.com/Ragnt/libwifi).

The most prominent features being:

- Frame encoding. Parsed or constructed frames can now be brought back into byte representation.
- CRC Frame validation.
- Many more parsed `StationInfo` fields.
- Various parsers for new control and data frames, including
  - `CTS` and `Ack` frames for the `RTS -> CTS -> Data -> ACK` flow.
  - Complete `Data` frame parsing.
  - `DataCfAck` `DataCfPoll`, `DataCfAckCfPoll`, `CfPoll` and `CfAckCfPoll` frames
  - `QosDataCfAck`, `QosDataCfPoll`, `QosDataCfAckCfPoll`, `QosCfPoll` and `QosCfAckCfPoll` frames
- Deauthentication reason parsing
- `ReassociationRequest`, `ReassociationResponse` and `Disassociation` frames
- `Action` frames
- `Authentication` and `Deauthentication` frames
- More `MacAddress` helper functions and parsed formats.
- `MacAddressGlob` to match certain MacAddress spaces.

## [0.3.1] - 2022-06-23

### Changes

- The crate no longer depends on the nightly toolchain.

### Other

- Project cleanup
- Move `libwifi_macros` crate into the same repository
- Streamline our CI setup and add code coverage statistics
