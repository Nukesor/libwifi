use libwifi::{
    frame_types::{FrameSubType, FrameType},
    variants::Payload,
    Frame,
};

#[test]
fn test_beacon() {
    let payload = [
        // Header
        128, 0, // FrameControl
        0, 0, // Duration id
        255, 255, 255, 255, 255, 255, // First address
        248, 50, 228, 173, 71, 184, // Second address
        248, 50, 228, 173, 71, 184, // Third address
        96, 119, // SequencControl
        // Data start
        151, 161, 39, 206, 165, 0, 0, 0, // timestamp
        100, 0, // interval
        17, 4, // capability
        0, 15, 77, 121, 32, 102, 97, 99, 101, 32, 119, 104, 101, 110, 32, 73, 80, // SSID
        1, 8, 130, 132, 139, 150, 36, 48, 72, 108, // Supported rates
        3, 1, 9, 5, 4, 0, 3, 1, 0, 42, 1, 4, 47, 1, 4, 48, 20, 1, 0, 0, 15, 172, 4, 1, 0, 0, 15,
        172, 4, 1, 0, 0, 15, 172, 2, 12, 0, 50, 4, 12, 18, 24, 96, 45, 26, 189, 25, 23, 255, 255,
        255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 61, 22, 9, 8, 4, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 74, 14, 20, 0, 10, 0, 44, 1, 200, 0, 20, 0,
        5, 0, 25, 0, 127, 8, 1, 0, 0, 0, 0, 0, 0, 64, 221, 49, 0, 80, 242, 4, 16, 74, 0, 1, 16, 16,
        68, 0, 1, 2, 16, 71, 0, 16, 190, 15, 245, 213, 137, 177, 64, 140, 203, 243, 77, 29, 90,
        130, 118, 247, 16, 60, 0, 1, 3, 16, 73, 0, 6, 0, 55, 42, 0, 1, 32, 221, 9, 0, 16, 24, 2, 5,
        0, 28, 0, 0, 221, 24, 0, 80, 242, 2, 1, 1, 132, 0, 3, 164, 0, 0, 39, 164, 0, 0, 66, 67, 94,
        0, 98, 50, 47, 0,
    ];

    let frame = Frame::parse(&payload).expect("Payload should be valid");
    println!("{:?}", frame);
    assert!(matches!(frame.control.frame_type, FrameType::Management));
    assert!(matches!(frame.control.frame_subtype, FrameSubType::Beacon));

    if let Payload::Beacon(beacon) = frame.payload {
        assert_eq!("My face when IP", beacon.station_info.ssid.unwrap());
    }
}

#[test]
fn test_probe_request() {
    let payload = [
        // Header
        64, 0, // FrameControl
        0, 0, // Duration id
        255, 255, 255, 255, 255, 255, // First address
        192, 238, 251, 75, 207, 58, // Second address
        255, 255, 255, 255, 255, 255, // Thrid address
        48, 89, // Sequence Control
        // Data start
        0, 0, 1, 4, 2, 4, 11, 22, 50, 8, 12, 18, 24, 36, 48, 72, 96, 108, 221, 7, 0, 80, 242, 8, 0,
        36, 0, 3, 1, 9, 45, 26, 111, 1, 31, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 191, 12, 178, 97, 128, 51, 254, 255, 134, 1, 254, 255, 134, 1,
    ];

    let frame = Frame::parse(&payload).expect("Payload should be valid");
    println!("{:?}", frame);
    assert!(matches!(frame.control.frame_type, FrameType::Management));
    assert!(matches!(
        frame.control.frame_subtype,
        FrameSubType::ProbeRequest
    ));
}

#[test]
fn test_probe_response() {
    let payload = [
        // Header
        80, 0, // FrameControl
        58, 1, // Duration id
        192, 238, 251, 75, 207, 58, // First address
        248, 50, 228, 173, 71, 184, // Second address
        248, 50, 228, 173, 71, 184, // Third address
        144, 1, // SequenceControl
        129, 106, 187, 25, 166, 0, 0, 0, // Timesetamp
        100, 0, //  beacon interval
        17, 4, // capability info
        0, 15, 77, 121, 32, 102, 97, 99, 101, 32, 119, 104, 101, 110, 32, 73, 80, // SSID
        1, 8, 130, 132, 139, 150, 36, 48, 72, 108, 3, 1, 9, 42, 1, 4, 47, 1, 4, 48, 20, 1, 0, 0,
        15, 172, 4, 1, 0, 0, 15, 172, 4, 1, 0, 0, 15, 172, 2, 12, 0, 50, 4, 12, 18, 24, 96, 45, 26,
        189, 25, 23, 255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 61,
        22, 9, 8, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 74, 14, 20, 0, 10, 0,
        44, 1, 200, 0, 20, 0, 5, 0, 25, 0, 127, 8, 1, 0, 0, 0, 0, 0, 0, 64, 221, 172, 0, 80, 242,
        4, 16, 74, 0, 1, 16, 16, 68, 0, 1, 2, 16, 59, 0, 1, 3, 16, 71, 0, 16, 190, 15, 245, 213,
        137, 177, 64, 140, 203, 243, 77, 29, 90, 130, 118, 247, 16, 33, 0, 21, 65, 83, 85, 83, 84,
        101, 75, 32, 67, 111, 109, 112, 117, 116, 101, 114, 32, 73, 110, 99, 46, 16, 35, 0, 28, 87,
        105, 45, 70, 105, 32, 80, 114, 111, 116, 101, 99, 116, 101, 100, 32, 83, 101, 116, 117,
        112, 32, 82, 111, 117, 116, 101, 114, 16, 36, 0, 7, 82, 84, 45, 78, 54, 54, 85, 16, 66, 0,
        17, 102, 56, 58, 51, 50, 58, 101, 52, 58, 97, 100, 58, 52, 55, 58, 98, 56, 16, 84, 0, 8, 0,
        6, 0, 80, 242, 4, 0, 1, 16, 17, 0, 7, 82, 84, 45, 78, 54, 54, 85, 16, 8, 0, 2, 32, 8, 16,
        60, 0, 1, 3, 16, 73, 0, 6, 0, 55, 42, 0, 1, 32, 221, 9, 0, 16, 24, 2, 5, 0, 28, 0, 0, 221,
        24, 0, 80, 242, 2, 1, 1, 132, 0, 3, 164, 0, 0, 39, 164, 0, 0, 66, 67, 94, 0, 98, 50, 47, 0,
    ];

    let frame = Frame::parse(&payload).expect("Payload should be valid");
    println!("{:?}", frame);
    assert!(matches!(frame.control.frame_type, FrameType::Management));
    assert!(matches!(
        frame.control.frame_subtype,
        FrameSubType::ProbeResponse
    ));

    if let Payload::ProbeResponse(response) = frame.payload {
        assert_eq!("My face when IP", response.station_info.ssid.unwrap());
    }
}

#[test]
fn test_authentication() {
    let _payload = [
        176, 0, // FrameControl
        58, 1, // Duration id
        248, 50, 228, 173, 71, 184, // First Address
        192, 238, 251, 75, 207, 58, // Second Address
        248, 50, 228, 173, 71, 184, 0, 147, 0, 0, 1, 0, 0, 0,
    ];
}

#[test]
fn test_deauthentication() {
    let _payload = [
        192, 0, // FrameControl
        58, 1, // Duration id
        248, 50, 228, 173, 71, 184, // First Address
        192, 238, 251, 75, 207, 58, // Second Address
        248, 50, 228, 173, 71, 184, 224, 146, 3, 0,
    ];
}

#[test]
fn test_null_data() {
    let _payload = [
        72, 17, 60, 0, 156, 128, 223, 131, 16, 180, 252, 25, 16, 16, 128, 171, 156, 128, 223, 131,
        16, 180, 128, 43,
    ];
}

#[test]
fn test_qos_null_data() {
    let _payload = [
        200, 1, // FrameControl
        58, 1, // Duration id
        248, 50, 228, 173, 71, 184, // First Address
        192, 238, 251, 75, 207, 58, // Second Address
        248, 50, 228, 173, 71, 184, // Third Address
        80, 106, 0, 0,
    ];
}

#[test]
fn test_data() {
    let _payload = [
        8, 98, 0, 0, 51, 51, 255, 75, 207, 58, 248, 50, 228, 173, 71, 184, 192, 238, 251, 75, 207,
        58, 80, 2, 90, 7, 0, 96, 0, 0, 0, 0, 239, 46, 109, 235, 61, 58, 89, 37, 181, 238, 23, 98,
        108, 29, 99, 170, 28, 132, 136, 248, 109, 194, 64, 139, 35, 219, 22, 195, 40, 100, 32, 6,
        7, 230, 5, 102, 8, 116, 33, 165, 132, 177, 44, 2, 247, 88, 213, 77, 12, 122, 49, 105, 29,
        74, 55, 207, 160, 46, 181, 65, 63, 123, 109, 117, 156, 77, 0, 65, 14, 72, 91, 169, 153, 0,
        55, 68, 180, 178, 230, 66,
    ];
}

#[test]
fn test_rts() {
    let _payload = [
        180, 0, 158, 0, 116, 66, 127, 77, 29, 45, 20, 125, 218, 170, 84, 81,
    ];
}

#[test]
fn test_cts() {
    // 2B FrameControl + 2B Duration + 6B Address1 (Missing CRC)
    let _payload = [196, 0, 246, 14, 224, 62, 68, 8, 195, 239];
}

#[test]
fn test_ack() {
    // 2b FrameControl + 2B Duration + 6B Address1 (Missing CRC)
    let _payload = [212, 0, 0, 0, 104, 217, 60, 214, 195, 239];
}

#[test]
fn test_block_ack_request() {
    // 2B FrameControl + 2B Duration + 6B Address1 + 6B Address2 + 4B CRC
    let _payload = [
        132, 0, 58, 1, 192, 238, 251, 75, 207, 58, 24, 29, 234, 198, 62, 190, 4, 0, 160, 15,
    ];
}

#[test]
fn test_block_ack() {
    let _payload = [
        148, 0, 0, 0, 192, 238, 251, 75, 207, 58, 248, 50, 228, 173, 71, 184, 5, 0, 144, 4, 1, 0,
        0, 0, 0, 0, 0, 0,
    ];
}
