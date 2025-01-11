# Libwifi

[![GitHub Actions Workflow](https://github.com/Nukesor/libwifi/workflows/Tests/badge.svg)](https://github.com/Nukesor/libwifi/actions)
[![docs](https://docs.rs/libwifi/badge.svg)](https://docs.rs/libwifi/)
[![license](http://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/nukesor/libwifi/blob/main/LICENSE)
[![Crates.io](https://img.shields.io/crates/v/libwifi.svg)](https://crates.io/crates/libwifi)
[![codecov](https://codecov.io/gh/nukesor/libwifi/branch/main/graph/badge.svg)](https://codecov.io/gh/nukesor/libwifi)

First of all, this library is designed to be easily extendable. \
There's an architectural/contribution guide in `docs/Frame.md` and pull requests are highly welcome.

Covering the whole spectrum of possible 802.11 frames or all different implementations of wifi tools out there is an impossible task for a single person, let's try to tackle this together!

Large parts of this library have been backported from @Ragnt's [AngryOxide](https://github.com/Ragnt/AngryOxide).

## What is Libwifi

The first goal of `libwifi` is to provide a **convenient** way of parsing raw IEEE 802.11 frames!

The emphasis is on **convenient**, since this library doesn't focus on providing an ultra-high-performance implementation. The focus is rather on providing an easy-to-use API. \
This includes consistent and intuitive structs that represent the structure of a given frame. \
However, this doesn't mean that this library [isn't quite fast anyway ;)](https://github.com/Nukesor/libwifi#performance).

The second goal is to provide a unified API to:

1. query information about your wifi interfaces (`iwlist` equivalent).
2. set attributes and configure your wifi interfaces (`iwconfig` equivalent).

As a prototype implementation, it's planned to call existing binaries and parse their output.
However, a native re-implementation of those tools is most definitely desirable in the long term. \
For instance, the [wireless-tools](https://github.com/HewlettPackard/wireless-tools) are a great C project with a lot of documentation and very well structured code.
This could be used as a guideline when working on a re-implementation.

The project is still under heavy development, and a lot of features are missing, but it should be a good foundation for a proper wifi library :).

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

Right now, parsing a `Beacon` frame, which is one of the more complex frames, takes `~1 µs` on a i7-8550U. \
Parsing a `Data` frame takes `~84 ns`.

If we take this as a rough guideline, you can roughly expect a million frames per second per core on that CPU.

**Disclaimer:** This will most likely become slower, as more missing features/parsers will be added to the library.
Anyhow, I don't expect this to drop below 100k frames/s.

## Roadmap and TODOs

### Parser and Frames

- [ ] Implement basic parsers for all frame subtypes.
- [ ] Add specialized parsers for fields that are currently generically handled by the `StationInfo` struct.
- [ ] Handle all edge-cases (there are a lot and I'll need help!)

### Interface handling

I would love to add proper interface handling to the library.
This includes features to:

- [ ] Switch modes
- [ ] Switch channels
- [ ] Discover available channels

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
