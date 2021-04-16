use anyhow::Result;
use pcap::Packet;
use radiotap::Radiotap;

pub fn handle_packet(packet: Packet) -> Result<()> {
    // At first, we look at the
    let radiotap = match Radiotap::from_bytes(packet.data) {
        Ok(radiotap) => radiotap,
        Err(error) => {
            println!(
                "Couldn't read packet data with Radiotap: {:?}, error {:?}",
                &packet.data, error
            );
            return Ok(());
        }
    };

    let payload = &packet.data[radiotap.header.length..];
    match libwifi::parse(&payload) {
        Ok(frame) => {
            println!("Got frame: {:?}", frame);
        }
        Err(err) => {
            println!("Error during parsing :\n{}", err);
            match err {
                libwifi::error::Error::Failure(_, data) => println!("{:?}", data),
                _ => (),
            }
        }
    };

    Ok(())
}
