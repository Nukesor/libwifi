use libwifi::frame::Frame;
use libwifi::parse_frame;

#[test]
fn test_rts() {
    let payload = [
        180, 0, // FrameControl
        158, 0, // Duration
        116, 66, 127, 77, 29, 45, // First Address
        20, 125, 218, 170, 84, 81, // Second Address
    ];

    let frame = parse_frame(&payload).expect("Payload should be valid");
    println!("{:?}", frame);
    assert!(matches!(frame, Frame::Rts(_)));
}

#[test]
fn test_cts() {
    // 2B FrameControl + 2B Duration + 6B Address1 (Missing CRC)
    let _payload = [
        196, 0, // FrameControl
        246, 14, // Duration
        224, 62, 68, 8, 195, 239, // First Address
    ];
}

#[test]
fn test_ack() {
    // 2b FrameControl + 2B Duration + 6B Address1 (Missing CRC)
    let _payload = [
        212, 0, // FrameControl
        0, 0, // Duration
        104, 217, 60, 214, 195, 239, // First Address
    ];
}

#[test]
fn test_block_ack_request() {
    let _payload = [
        132, 0, // FrameControl
        58, 1, // Duration
        192, 238, 251, 75, 207, 58, // First Address
        24, 29, 234, 198, 62, 190, // Second Address
        4, 0, 160, 15, // FCS
    ];
}

#[test]
fn test_block_ack() {
    let _payload = [
        148, 0, // FrameControl
        0, 0, // Duration
        192, 238, 251, 75, 207, 58, // First Address
        248, 50, 228, 173, 71, 184, // Second Address
        5, 0, 144, 4, 1, 0, 0, 0, 0, 0, 0, 0,
    ];
}
