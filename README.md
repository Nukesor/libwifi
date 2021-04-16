# Libwifi

[![GitHub Actions Workflow](https://github.com/Nukesor/libwifi/workflows/Tests/badge.svg)](https://github.com/Nukesor/libwifi/actions)
[![docs](https://docs.rs/libwifi/badge.svg)](https://docs.rs/libwifi/)
[![license](http://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/nukesor/libwifi/blob/master/LICENSE)
[![Crates.io](https://img.shields.io/crates/v/libwifi.svg)](https://crates.io/crates/libwifi)
<!--- [![codecov](https://codecov.io/gh/nukesor/libwifi/branch/master/graph/badge.svg)](https://codecov.io/gh/nukesor/libwifi) -->

First of all, this library is designed to be easily extendable. \
There's an architectural/contribution guide in `docs/Frame.md` and pull requests are highly welcome.

Covering the whole spectrum of possible 802.11 frames or all different implementations of wifi tools out there is an impossible task for a single person, let's try to tackle this together!

### What is Libwifi

The first goal of `libwifi` is to provide a **convenient** way of parsing raw IEEE 802.11 frames!

The emphasis is on **convenient**, since this library doesn't focus on providing a super high-performance implementation. \
The focus is rather on providing an easy-to-use API.
This includes consistent and intuitive structs representing the structure of a given frame. \
However, this doesn't mean that this library isn't quite fast anyway ;).

### How to use it

Parsing a frame is fairly straight forward:

```
use libwifi::parse;

let bytes = [
    180, 0, // FrameControl
    158, 0, // Duration
    116, 66, 127, 77, 29, 45, // First Address
    20, 125, 218, 170, 84, 81, // Second Address
];

match libwifi::parse(&bytes) {
    Ok(frame) => {
        println!("Got frame: {:?}", frame);
    }
    Err(err) => {
        println!("Error during parsing :\n{}", err);
    }
};
```

A full example on how to capture, process and parse wifi traffic can be found in the `examples` directory.

### Roadmap and TODOs

**Parser and Frames**

- [ ] Implement basic parsers for all frame subtypes.
- [ ] Add specialized parsers for fields that are currently generically handled by the `StationInfo` struct.
- [ ] Handle all edge-cases (there are a lot and I'll need help!)

**Interface handling**

I would love to add proper interface handling to the library.
This includes features to:

- [ ] Switch modes
- [ ] Switch channels
- [ ] Discover available channels

### Nightly

This library requires nightly for now!

It uses the `destructuring_assignment` feature, which adds a lot of convenience when working with the `nom` library. \

If you want to start using it in a professional environment, I'm open to PRs that refactor the usages of this feature.
Shouldn't take longer than a few minutes to do. \
Until then, I'm hoping that this feature will be included to stable soon :).
