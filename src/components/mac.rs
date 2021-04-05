/// This is our representation of a MAC-address
#[derive(Clone, Debug)]
pub struct MacAddress(pub [u8; 6]);

impl MacAddress {
    /// Return the MacAddress' bytes in easily readable Hex-code
    pub fn to_string(&self) -> String {
        format!(
            "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
            self.0[0], self.0[1], self.0[2], self.0[3], self.0[4], self.0[5]
        )
    }

    /// Check whether this MAC addresses the whole network.
    pub fn is_broadcast(&self) -> bool {
        self.0 == [255, 255, 255, 255, 255, 255]
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
