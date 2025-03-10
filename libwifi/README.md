# Libwifi

[![GitHub Actions Workflow](https://github.com/Nukesor/libwifi/workflows/Tests/badge.svg)](https://github.com/Nukesor/libwifi/actions)
[![docs](https://docs.rs/libwifi/badge.svg)](https://docs.rs/libwifi/)
[![license](http://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/nukesor/libwifi/blob/main/LICENSE)
[![Crates.io](https://img.shields.io/crates/v/libwifi.svg)](https://crates.io/crates/libwifi)
[![codecov](https://codecov.io/gh/nukesor/libwifi/branch/main/graph/badge.svg)](https://codecov.io/gh/nukesor/libwifi)

First of all, this library is designed to be easily extendable. \
There's an architectural/contribution guide in `docs/Frame.md` and pull requests are highly welcome.

Covering the whole spectrum of possible 802.11 frames or all different implementations of wifi tools out there is an impossible task for a single person, let's try to tackle this together!

Large parts of this library have been upstreamed from @Ragnt's [AngryOxide](https://github.com/Ragnt/AngryOxide).

## What is Libwifi

The goal of `libwifi` is to provide a **convenient** way of parsing raw IEEE 802.11 frames!

The emphasis is on **convenient**, as the focus is to provide an easy-to-use API that includes consistent and intuitive structs representing the structure of a given frame. \
Also this library [is very fast](#performance), despite the focus on convenience.

The project is still under heavy development, quite a few features and some documentation are missing, but it should be a good foundation for a proper IEE 802.11 library :).

## Contributing

I'm no longer actively using this library myself, so it relies on external contributions.

Writing documentation and tests are an easy way to start contributing and a huge help!

## How to use it

Parsing a frame is fairly straight forward:

```rs
use libwifi::parse_frame;

// A simple RTS frame
let bytes = [
    180, 0, // FrameControl
    158, 0, // Duration
    116, 66, 127, 77, 29, 45, // First Address
    20, 125, 218, 170, 84, 81, // Second Address
];

match libwifi::parse_frame(&bytes) {
    Ok(frame) => {
        println!("Got frame: {frame:?}");
    }
    Err(err) => {
        println!("Error during parsing :\n{err:?}");
    }
};
```

A full example on how to capture, process and parse wifi traffic can be found in the `examples` directory.

## Performance

There are a few benchmarks in the `benches` folder, which can be run by calling `cargo bench`.

Right now, parsing a `Beacon` frame, which is one of the more complex frames, takes `~300ns` on a `AMD Ryzen 7 7840HS`. \
Parsing a `Data` frame takes `~28 ns`.

If we take this as a rough guideline, you can roughly expect 3-35 million frames per second per core on that CPU, depending on frame type.

## Roadmap and TODOs

### Parser and Frames

- [ ] Implement basic parsers for all frame subtypes.
- [x] Add specialized parsers for fields that are currently generically handled by the `StationInfo` struct.
- [ ] Handle all edge-cases (there are a lot)

### Implementation status

- Management Frames
  - [x] AssociationRequest,
  - [x] AssociationResponse,
  - [x] ReassociationRequest,
  - [x] ReassociationResponse,
  - [x] Deauthentication,
  - [x] ProbeRequest,
  - [x] ProbeResponse,
  - [ ] TimingAdvertisement,
  - [x] Beacon,
  - [ ] Atim,
  - [x] Disassociation,
  - [x] Authentication,
  - [x] Action,
  - [ ] ActionNoAck,
- Control Frames
  - [ ] Trigger,
  - [ ] Tack,
  - [ ] BeamformingReportPoll,
  - [ ] NdpAnnouncement,
  - [ ] ControlFrameExtension,
  - [ ] ControlWrapper,
  - [x] BlockAckRequest,
  - [x] BlockAck,
  - [ ] PsPoll,
  - [x] Rts,
  - [x] Cts,
  - [x] Ack,
  - [ ] CfEnd,
  - [ ] CfEndCfAck,
- Data Frames
  - [x] Data,
  - [x] DataCfAck,
  - [x] DataCfPoll,
  - [x] DataCfAckCfPoll,
  - [x] NullData,
  - [x] CfAck,
  - [x] CfPoll,
  - [x] CfAckCfPoll,
  - [x] QosData,
  - [x] QosDataCfAck,
  - [x] QosDataCfPoll,
  - [x] QosDataCfAckCfPoll,
  - [x] QosNull,
  - [x] QosCfPoll,
  - [x] QosCfAckCfPoll,
- Frame Components
  - [x] Frame Control
  - [x] Sequence Control
  - [x] Management Header
  - [x] Dynamic Management Header fields
    - [x] SSID
    - [x] Supported rates
    - [x] Generic extraction of remaining fields
  - [x] Data Header
  - [x] QoS Data Header

There's a lot more to the IEE 802.11 spec and a lot of stuff needs to be done. \
If you find that something you need is missing, consider creating a ticket and contributing :).

### Fuzzing

`cargo-fuzz` can be used to check for potential crashes while processing unvalidated input data. After [installing cargo-fuzz](https://github.com/rust-fuzz/cargo-fuzz) (note: may require rust nightly), the frame parsing can be tested with `cargo fuzz run parse_frame`.
