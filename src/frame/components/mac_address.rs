use std::fmt;

/// This is our representation of a MAC-address
///
/// ```
/// use libwifi::frame::components::MacAddress;
///
/// let address = MacAddress([255, 255, 255, 255, 255, 255]);
/// println!("{}", address.is_broadcast());
/// // -> true
/// ```
///
#[derive(Clone, Debug)]
pub struct MacAddress(pub [u8; 6]);

impl MacAddress {
    /// Check whether this MAC addresses the whole network.
    pub fn is_broadcast(&self) -> bool {
        self.0 == [255, 255, 255, 255, 255, 255]
    }

    /// 33:33::::0 is used for ipv6 neighborhood discovery.
    pub fn is_ipv6_neighborhood_discovery(&self) -> bool {
        self.0 == [51, 51, 0, 0, 0, 0]
    }

    /// The 33:33::::0/16 space is reserved for ipv6 multicast
    pub fn is_ipv6_multicast(&self) -> bool {
        self.0[0] == 51 && self.0[1] == 51
    }
}

impl fmt::Display for MacAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
            self.0[0], self.0[1], self.0[2], self.0[3], self.0[4], self.0[5]
        )
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum MacParseError {
    InvalidDigit,
    InvalidLength,
}

impl fmt::Display for MacParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Encountered an error while parsing a mac address.")
    }
}

impl std::error::Error for MacParseError {}

impl std::str::FromStr for MacAddress {
    type Err = MacParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut array = [0u8; 6];

        let bytes: Vec<&str> = input.split(|c| c == ':').collect();
        if bytes.len() != 6 {
            return Err(MacParseError::InvalidLength);
        }

        let mut count = 0;
        for byte in bytes {
            array[count] = u8::from_str_radix(byte, 16).map_err(|_| MacParseError::InvalidDigit)?;

            count += 1;
        }

        Ok(MacAddress(array))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_broadcast() {
        let mac = MacAddress([255, 255, 255, 255, 255, 255]);
        assert!(mac.is_broadcast())
    }

    #[test]
    fn test_format() {
        let mac = MacAddress([12, 157, 146, 197, 170, 127]);
        assert_eq!("0c:9d:92:c5:aa:7f", mac.to_string())
    }
}
