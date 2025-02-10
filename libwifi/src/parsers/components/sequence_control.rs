use nom::{bytes::complete::take, IResult};

use crate::frame::components::SequenceControl;

pub fn parse_sequence_control(input: &[u8]) -> IResult<&[u8], SequenceControl> {
    // Read exactly 2 bytes (16 bits)
    let (remaining, sequence_control_bytes) = take(2usize)(input)?;

    // Ensure that we have exactly two bytes
    let byte1 = sequence_control_bytes[0];
    let byte2 = sequence_control_bytes[1];

    // Extract fragment number (lower 4 bits of byte1)
    let fragment_number = byte1 & 0b00001111;

    // Extract the 12-bit sequence number:
    // - The upper 4 bits from byte1, shifted right by 4 bits
    // - The entire byte2 shifted left to fill the remaining 8 bits
    let sequence_number = ((byte1 as u16 & 0b11110000) >> 4) | ((byte2 as u16) << 4);

    Ok((
        remaining,
        SequenceControl {
            fragment_number,
            sequence_number,
        },
    ))
}
