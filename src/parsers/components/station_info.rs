use nom::bytes::complete::take;
use nom::number::complete::u8 as get_u8;
use nom::sequence::tuple;
use nom::IResult;
use hex::encode;

use crate::frame::components::StationInfo;

/// Parse variable length and variable field information.
/// The general structure of the data looks like this:
///
/// 1 byte: Element id
/// 1 byte: Element length (up to 255 bytes)
/// $element_length bytes: Element data
///
/// This format is only used in management frames.
///
/// There might be multiple elements with the same element id,
/// which is why StationInfo uses a Vec instead of BTreeMap as a data structure.
pub fn parse_station_info(mut input: &[u8]) -> IResult<&[u8], StationInfo> {
    let mut station_info = StationInfo::default();

    let mut element_id;
    let mut length;
    let mut data;
    loop {
        (input, (element_id, length)) = tuple((get_u8, get_u8))(input)?;
        //println!("Element id {}, Length: {}", element_id, length);
        (input, data) = take(length)(input)?;
        //println!("Extracted data: {:?}", data);

        // Create list of tagged parameters.
        match element_id {
            // Don't match extended tags.
            id if id != 255 => station_info.tagged_parameters.push(element_id),
            _ => {},
        };

        match element_id {
            0 => {
                let mut ssid = String::from_utf8_lossy(data).to_string();
                // Remove null chars. Some APs seem to enjoy sending those.
                ssid = ssid.replace('\0', " ");
                station_info.ssid = Some(ssid);
            }
            1 => station_info.supported_rates = parse_supported_rates(data),
            45 => {
                station_info.ht_capabilities_info = Some(network_endian_format(&data[0..2]));
                station_info.ht_a_mpdu_parameters = Some(network_endian_format(&data[2..3]));
                station_info.ht_rx_mcs = Some(network_endian_format(&data[3..7]));
            }   
            _ => {
                station_info.data.push((element_id, data.to_vec()));
            }
        };

        if input.len() <= 4 {
            break;
        }
    }

    Ok((input, station_info))
}

/// This is used in the ProbeResponse frame.
/// It indicates which transmission rates (in Mbps) are supported by this AP.
fn parse_supported_rates(input: &[u8]) -> Vec<f32> {
    let mut rates: Vec<f32> = Vec::new();
    for rate in input {
        match rate {
            0x82 => rates.push(1.0),
            0x84 => rates.push(2.0),
            0x8b => rates.push(5.5),
            0x0c => rates.push(6.0),
            0x12 => rates.push(9.0),
            0x96 => rates.push(11.0),
            0x18 => rates.push(12.0),
            0x24 => rates.push(18.0),
            0x2c => rates.push(22.0),
            0x30 => rates.push(24.0),
            0x42 => rates.push(33.0),
            0x48 => rates.push(36.0),
            0x60 => rates.push(48.0),
            0x6c => rates.push(54.0),
            _ => continue,
        }
    }

    rates
}

// Used to translate a borrowed u8 slice into a network endian (big) formatted hexadecimal string.
fn network_endian_format(input: &[u8]) -> String {
    let mut reversed_info: Vec<u8> = Vec::new();
    reversed_info.extend(input.iter().rev());

    format!("0x{}", encode(&reversed_info))
}
