# Libwifi

First of all, this library is designed to be easily extendable. \
There's an architectural/contribution guide in `docs/Frame.md` and pull requests are highly welcome.
Covering the whole spectrum of possible 802.11 frames or all different implementations of wifi tools out there is an impossible task for a single person, let's try to tackle this together!


**This library requires nightly for now!** \
It uses the `destructuring_assignment` feature, which adds a lot of convenience when working with the `nom` library.

### What is Libwifi

The first goal of `libwifi` is to provide a **convenient** way of parsing raw IEEE 802.11 frames!
The emphasis is on **convenient**, since this library doesn't focus on providing a super high-performance implementation. \
The focus is rather on building an easy-to-use API and providing consistent and intuitive structs that represent the structure in a given frame.
This doesn't mean that this library isn't quite fast anyway ;).


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
