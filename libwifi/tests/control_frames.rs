use libwifi::frame::*;
use libwifi::parse_frame;

#[test]
fn test_rts() {
    let payload = [
        180, 0, // FrameControl
        158, 0, // Duration
        116, 66, 127, 77, 29, 45, // First Address
        20, 125, 218, 170, 84, 81, // Second Address
    ];

    let frame = parse_frame(&payload, false).expect("Payload should be valid");
    println!("{frame:?}");
    assert!(matches!(frame, Frame::Rts(_)));
}

#[test]
fn test_cts() {
    let payload = [
        196, 0, // FrameControl
        246, 14, // Duration
        224, 62, 68, 8, 195, 239, // First Address
    ];

    let frame = parse_frame(&payload, false).expect("Payload should be valid");
    println!("{frame:?}");
    assert!(matches!(frame, Frame::Cts(_)));
}

#[test]
fn test_ack() {
    let payload = [
        212, 0, // FrameControl
        0, 0, // Duration
        104, 217, 60, 214, 195, 239, // First Address
    ];

    let frame = parse_frame(&payload, false).expect("Payload should be valid");
    println!("{frame:?}");
    assert!(matches!(frame, Frame::Ack(_)));
}

#[test]
fn test_single_tid_compressed_block_ack_request() {
    let payload = [
        132, 0, // FrameControl
        58, 1, // Duration
        192, 238, 251, 75, 207, 58, // First Address
        24, 29, 234, 198, 62, 190, // Second Address
        4, 0, // BlockAckRequest Control
        160, 15, // Starting sequence number of the single TID
    ];

    let frame = parse_frame(&payload, false).expect("Payload should be valid");
    println!("{frame:?}");
    assert!(matches!(frame, Frame::BlockAckRequest(_)));

    if let Frame::BlockAckRequest(inner) = frame {
        assert!(matches!(inner.mode, BlockAckMode::CompressedBlockAck));
    } else {
        panic!("invalid frame type");
    }
}

#[test]
fn test_single_tid_compressed_block_ack_request_detailed() {
    let payload = [
        132, 0, // FrameControl
        58, 1, // Duration
        192, 238, 251, 75, 207, 58, // First Address
        24, 29, 234, 198, 62, 190, // Second Address
        4, 0x10, // BlockAckRequest Control with TID=1
        160, 15, // Starting sequence number of the single TID
    ];

    let frame = parse_frame(&payload, false).expect("Payload should be valid");
    println!("{frame:?}");
    assert!(matches!(frame, Frame::BlockAckRequest(_)));

    if let Frame::BlockAckRequest(inner) = frame {
        assert!(matches!(inner.mode, BlockAckMode::CompressedBlockAck));
        assert!(inner.requested_tids.len() == 1);
        let (tid, seq) = inner.requested_tids.first().unwrap();
        assert!(*tid == 1);
        assert!(seq.fragment_number == 0);
        assert!(seq.sequence_number == 250);
    } else {
        panic!("invalid frame type");
    }
}

#[test]
fn test_compressed_bitmap_block_ack() {
    let payload = [
        148, 0, // FrameControl
        0, 0, // Duration
        192, 238, 251, 75, 207, 58, // First Address
        248, 50, 228, 173, 71, 184, // Second Address
        5, 0, // BlockAck Control
        144, 4, // BlockAck starting sequence control
        1, 0, 0, 0, 0, 0, 0, 0, // BlockAck Bitmap
    ];

    let frame = parse_frame(&payload, false).expect("Payload should be valid");
    println!("{frame:?}");
    assert!(matches!(frame, Frame::BlockAck(_)));

    if let Frame::BlockAck(inner) = frame {
        assert!(matches!(inner.mode, BlockAckMode::CompressedBlockAck));
    } else {
        panic!("invalid frame type");
    }
}

#[test]
fn test_compressed_bitmap_block_ack_detailed() {
    let payload = [
        148, 0, // FrameControl
        0, 0, // Duration
        192, 238, 251, 75, 207, 58, // First Address
        248, 50, 228, 173, 71, 184, // Second Address
        5, 0x10, // BlockAck Control with tid=1
        144, 4, // BlockAck starting sequence control, seqnr 73, fragment 0
        0x3f, 0, 0, 0, 0, 0, 0, 0, // BlockAck Bitmap, ACK first 6 frames
    ];

    let frame = parse_frame(&payload, false).expect("Payload should be valid");
    println!("{frame:?}");
    assert!(matches!(frame, Frame::BlockAck(_)));

    if let Frame::BlockAck(inner) = frame {
        assert!(matches!(inner.mode, BlockAckMode::CompressedBlockAck));
        assert!(inner.policy); // immediate ack is set

        if let BlockAckInfo::Compressed(acks) = inner.acks {
            assert!(acks.len() == 1);

            let (tid, start_seq, bitmap) = acks.first().unwrap();
            assert!(*tid == 1);
            assert!(start_seq.fragment_number == 0);
            assert!(start_seq.sequence_number == 73);

            let mut acked: Vec<u16> = Vec::new();
            for i in 0..64 {
                if (bitmap & (1 << i)) > 0 {
                    acked.push(start_seq.sequence_number + i);
                }
            }
            assert!(acked == vec![73, 74, 75, 76, 77, 78]);
        } else {
            panic!("BlockAckInfo had wrong type");
        }
    } else {
        panic!("invalid frame type");
    }
}
