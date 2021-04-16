# Frames

`libwifi` doesn't support parsing all existing 802.11 frames yet!
But its design and parsing strategy allows to easily extend the current logic and add new frames :).

## Creating a New Frame

All frames are represented by the `Frame` enum, which is located in `src/frame/mod.rs`.
Every frame sub-type has it's own variant and a struct representing the data in that variant.

The structs contained in those variants are located in the subdirectories of `/src/frame/`.
They are ordered by the frame type, namely `management`, `data` and `control`.

### New Variant

Every frame sub-type has its own variant in the `Frame` enum.
Thereby, to add a new frame sub-type, you have to add add a new variant to `Frame`.
It should have the same name as the `FrameSubtype` enum equivalent.

This variant is also going to contain the struct that will represent the new frame.

### The Struct

Lets look at this by example.

```rust
#[derive(Clone, Debug)]
pub struct AssociationRequest {
    pub header: ManagementHeader,
    pub beacon_interval: u16,
    pub capability_info: u16,
    pub station_info: StationInfo,
}
```

This is the struct for association requests.
It consists of:

1. The `ManagementHeader`.
    All management frames have the same header format.
    Data frames hava consistent header format as well.
    Control frames on the other hand are super variable and thereby don't have a proper representation of a header.
2. `beacon_interval` is a field that's always present, which is why it has it's own field in the struct
3. `capability_info` is such a field as well.
4. `station_info` is used to parse and store variable length fields that are often sent with management frames.
    These fields always have an `id`, the length of the bytes for this field, and then the payload of the field.
    Since there's a large number of possible fields and many propriatary vendor-specific usages of these fields, a generic solution is used to capture all of them.
    Take a look at the `StationInfo` struct for more info.


### Components

It often doesn't make sense to store parts of a frame in their raw form or individually. \
For instance, you might want some convenience functions on mac addresses, such as proper string formatting. \
Another example is the highly complex header of management frames.
This data only makes sense if you look at it in a cohesive way.

For this reason, non-trivial data is stored in their own structs, which are called _components_.
All possible components are located in `src/components`.

Each component has their own parser in `/src/parsers/components/`, which will later be used by the actual frame parser functions.

### The Frame Parser

Now we get to the juicy parts.
Create a new function in the respective `src/parsers/frame_types/$FRAME_TYPE.rs`.
You can take some inspiration by the parsers in the `management.rs` file.

Each parser function gets the already parsed `FrameControl` struct and the raw bytes of the (nearly) whole frame.
Keep in mind, that the first two bytes are already missing, since `FrameControl` has alreay been parsed.

All parsing is done using the `nom` crate, which is excellent for byte- and bit-level parsing.

When it comes to parsing, there are a few guidelines:

- All stand-alone fields that are always present, should be added to the frame struct.
- Parsing logic for data that is complex or only makes sense if looked at as a unit, should be moved to its own component.
- Try to keep your data structures convenient!
    This library isn't build to be a super high-performance beast.
    It's designed to be convenient to use.

All parser functions have to have this signature format:

```
pub fn parse_FRAME_SUBTYPE (
    frame_control: FrameControl,
    input: &[u8],
) -> Result<Frame, Error> {...} 
```
