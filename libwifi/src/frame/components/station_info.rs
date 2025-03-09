use std::fmt::{self, Display};

use bitvec::{field::BitField, order::Lsb0, vec::BitVec};

#[derive(Clone, Debug, Default)]
/// StationInfo is used to parse and store variable length fields that are often sent
/// with management frames.
///
/// Each field has an `id`, the length of the bytes for this field, and then payload of the field.
/// Since there's a large number of possible fields and many propriatary vendor-specific usages
/// of these fields, this generic solution is used to capture all of them.
///
/// It is also important to note that most of these fields won't be sent most of the time. \
/// All fields that are already handled by this library get their own field in the StationInfo
/// struct.
///
/// Since we cannot handle all all those elements, the bytes of all unhandled elements will
/// be saved in the `data` field under the respectiv element id.
pub struct StationInfo {
    pub supported_rates: Vec<SupportedRate>,
    pub extended_supported_rates: Option<Vec<SupportedRate>>,
    pub ssid: Option<String>,
    pub ssid_length: Option<usize>,
    pub ds_parameter_set: Option<u8>,
    pub ibss_parameter_set: Option<u16>,
    pub tim: Option<Vec<u8>>,
    pub country_info: Option<Vec<u8>>,
    pub power_constraint: Option<u8>,
    pub ht_capabilities: Option<Vec<u8>>,
    pub ht_information: Option<HTInformation>,
    pub multiple_bssid: Option<MultipleBSSID>,
    pub vht_capabilities: Option<Vec<u8>>,
    pub rsn_information: Option<RsnInformation>,
    pub wpa_info: Option<WpaInformation>,
    pub wps_info: Option<WpsInformation>,
    pub vendor_specific: Vec<VendorSpecificInfo>,
    pub extended_capabilities: Option<ExtendedCapabilities>,
    pub channel_switch: Option<ChannelSwitchAnnouncment>,
    /// Contains all fields that aren't explicitly parsed by us.
    /// The format is Vec<(FieldId, PayloadBytes)>.
    ///
    /// Fields may occur multiple times.
    pub data: Vec<(u8, Vec<u8>)>,
}

impl StationInfo {
    pub fn encode(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        // Encode SSID (if present)
        if let Some(ssid) = &self.ssid {
            bytes.push(0); // ID
            bytes.push(ssid.len() as u8); // Length of SSID
            bytes.extend_from_slice(ssid.as_bytes()); // SSID as bytes
        }

        if !self.supported_rates.is_empty() {
            // Encode Supported Rates
            bytes.push(1); // ID
            bytes.push(self.supported_rates.len() as u8);
            for rate in &self.supported_rates {
                // Convert rate from Mbps to 500 kbps units and then to a byte
                let rate_byte = (rate.rate * 2.0) as u8;
                let rate_byte_with_flag = rate_byte | 0x80; // Setting MSB
                if rate.mandatory {
                    bytes.push(rate_byte_with_flag);
                } else {
                    bytes.push(rate_byte);
                }
            }
        }

        if let Some(ext_rates) = &self.extended_supported_rates {
            // Encode Supported Rates
            bytes.push(50); // ID
            bytes.push(ext_rates.len() as u8);
            for rate in ext_rates {
                // Convert rate from Mbps to 500 kbps units and then to a byte
                let rate_byte = (rate.rate * 2.0) as u8;
                let rate_byte_with_flag = rate_byte | 0x80; // Setting MSB
                if rate.mandatory {
                    bytes.push(rate_byte_with_flag);
                } else {
                    bytes.push(rate_byte);
                }
            }
        }

        // Encode DS Parameter Set (if present)
        if let Some(ds_param) = self.ds_parameter_set {
            bytes.push(3); // DS Parameter Set tag number
            bytes.push(1); // Length is always 1 byte
            bytes.push(ds_param);
        }

        // Encode TIM (if present) - Tag Number: 5
        if let Some(tim) = &self.tim {
            bytes.push(5); // TIM tag number
            bytes.push(tim.len() as u8); // Length of TIM
            bytes.extend(tim);
        }

        // Encode IBSS element set - Tag Number: 6
        if let Some(ibss_parameter_set) = self.ibss_parameter_set {
            bytes.push(6);
            bytes.push(2);
            bytes.extend(u16::to_le_bytes(ibss_parameter_set));
        }

        // Encode Country Info (if present) - Tag Number: 7
        if let Some(country_info) = &self.country_info {
            bytes.push(7); // Country Info tag number
            bytes.push(country_info.len() as u8); // Length of Country Info
            bytes.extend(country_info);
        }

        // Encode Power Constraint (if present) - Tag Number: 32
        if let Some(power_constraint) = self.power_constraint {
            bytes.push(32); // Power Constraint tag number
            bytes.push(1); // Length is always 1 byte
            bytes.push(power_constraint);
        }

        // Encode HT Capabilities (if present) - Tag Number: 45
        if let Some(ht_capabilities) = &self.ht_capabilities {
            bytes.push(45); // HT Capabilities tag number
            bytes.push(ht_capabilities.len() as u8); // Length of HT Capabilities
            bytes.extend(ht_capabilities);
        }

        // Encode HT Information (if present) - Tag Number: 61
        if let Some(ht_info) = &self.ht_information {
            let ht_info_data = ht_info.encode();
            bytes.push(61); // HT Capabilities tag number
            bytes.push(ht_info_data.len() as u8); // Length of HT Capabilities
            bytes.extend(ht_info_data);
        }

        // Encode Multiple BSSID (if present) - Tag Number: 71
        if let Some(multiple_bssid) = &self.multiple_bssid {
            let multiple_bssid_data = multiple_bssid.encode();
            bytes.push(71);
            bytes.push(multiple_bssid_data.len() as u8);
            bytes.extend(multiple_bssid_data);
        }

        // Encode VHT Capabilities (if present) - Tag Number: 191
        if let Some(vht_capabilities) = &self.vht_capabilities {
            bytes.push(191); // VHT Capabilities tag number
            bytes.push(vht_capabilities.len() as u8); // Length of VHT Capabilities
            bytes.extend(vht_capabilities);
        }

        // Encode RSN Information (if present) - Tag Number: 48
        if let Some(rsn_info) = &self.rsn_information {
            bytes.push(48); // RSN Information tag number
            let rsn_encoded = rsn_info.encode();
            bytes.push(rsn_encoded.len() as u8); // Length of RSN Information
            bytes.extend(rsn_encoded);
        }

        // Encode WPA Information (if present) - This is usually vendor-specific
        // WPA Information uses the vendor-specific tag number (221) with the specific OUI for WPA
        if let Some(wpa_info) = &self.wpa_info {
            bytes.push(221); // Vendor-Specific tag number
            let wpa_encoded = wpa_info.encode();
            bytes.push(wpa_encoded.len() as u8); // Length of WPA Information
            bytes.extend(wpa_encoded);
        }

        // Encode Vendor Specific Info
        for vendor_info in &self.vendor_specific {
            bytes.push(vendor_info.element_id);
            bytes.push(vendor_info.length);
            bytes.extend_from_slice(&vendor_info.oui);
            bytes.push(vendor_info.oui_type);
            bytes.extend(&vendor_info.data);
        }

        // Encode Extended Capabilities (if present)
        if let Some(ext_caps) = &self.extended_capabilities {
            bytes.push(127);
            let data = ext_caps.encode();
            bytes.push(data.len() as u8);
            bytes.extend(data);
        }

        if let Some(chan_switch) = &self.channel_switch {
            let encoded = chan_switch.encode();
            bytes.push(37);
            bytes.push(encoded.len() as u8);
            bytes.extend(encoded);
        }

        // Encode additional data
        for (id, data) in &self.data {
            bytes.push(*id);
            bytes.push(data.len() as u8);
            bytes.extend(data);
        }

        bytes
    }

    // Get the SSID from the station_info
    pub fn ssid(&self) -> String {
        match &self.ssid {
            Some(ssid) if !ssid.is_empty() => ssid.clone(),
            Some(_) if self.ssid_length.is_some_and(|s| s > 0) => {
                format!("<hidden: {}>", self.ssid_length.unwrap_or(0))
            }
            Some(_) => "<hidden>".to_string(),
            None => "".to_string(),
        }
    }

    // Handle ESSID where it could be empty (return Option<String> instead of String)
    pub fn essid(&self) -> Option<String> {
        match &self.ssid {
            Some(ssid) if !ssid.is_empty() => Some(ssid.clone()),
            Some(_) if self.ssid_length.is_some_and(|s| s > 0) => {
                Some(format!("<hidden: {}>", self.ssid_length.unwrap_or(0)))
            }
            Some(_) => Some("<hidden>".to_string()),
            None => None,
        }
    }

    // Get the channel this station is broadcasting on.
    pub fn channel(&self) -> Option<u8> {
        if let Some(ds) = self.ds_parameter_set {
            Some(ds)
        } else {
            self.ht_information
                .as_ref()
                .map(|ht_info| ht_info.primary_channel)
        }
    }

    // Get the WPA information
    pub fn wpa_info(&self) -> Option<&WpaInformation> {
        self.wpa_info.as_ref()
    }
}

#[derive(Clone, Debug)]
pub struct SupportedRate {
    pub mandatory: bool,
    pub rate: f32,
}

pub enum Category {
    Computer(Computers),
    InputDevice(InputDevices),
    PrintersScannersFaxCopier(PrintersEtAl),
    Camera(Cameras),
    Storage(Storage),
    NetworkInfrastructure(NetworkInfrastructure),
    Displays(Displays),
    MultimediaDevices(MultimediaDevices),
    GamingDevices(GamingDevices),
    Telephone(Telephone),
    AudioDevices(AudioDevices),
    DockingDevices(DockingDevices),
    Others,
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Category::Computer(sub) => write!(f, "{}", sub),
            Category::InputDevice(sub) => write!(f, "{}", sub),
            Category::PrintersScannersFaxCopier(sub) => {
                write!(f, "{}", sub)
            }
            Category::Camera(sub) => write!(f, "{}", sub),
            Category::Storage(sub) => write!(f, "{}", sub),
            Category::NetworkInfrastructure(sub) => write!(f, "{}", sub),
            Category::Displays(sub) => write!(f, "{}", sub),
            Category::MultimediaDevices(sub) => write!(f, "{}", sub),
            Category::GamingDevices(sub) => write!(f, "{}", sub),
            Category::Telephone(sub) => write!(f, "{}", sub),
            Category::AudioDevices(sub) => write!(f, "{}", sub),
            Category::DockingDevices(sub) => write!(f, "{}", sub),
            Category::Others => write!(f, "Others"),
        }
    }
}

pub enum Computers {
    PC,
    Server,
    MediaCenter,
    UltraMobilePC,
    Notebook,
    Desktop,
    MID,
    Netbook,
    Tablet,
    Ultrabook,
}

pub enum InputDevices {
    Keyboard,
    Mouse,
    Joystick,
    Trackball,
    GamingController,
    Remote,
    Touchscreen,
    BiometricReader,
    BarcodeReader,
}

pub enum PrintersEtAl {
    Printer,
    Scanner,
    Fax,
    Copier,
    AllInOne,
}

pub enum Cameras {
    DigitalCamera,
    VideoCamera,
    Webcam,
    SecurityCamera,
}

pub enum Storage {
    NAS,
}

pub enum NetworkInfrastructure {
    AP,
    Router,
    Switch,
    Gateway,
    Bridge,
}

pub enum Displays {
    Television,
    ElectronicPictureFrame,
    Projector,
    Monitor,
}

pub enum MultimediaDevices {
    DAR,
    PVR,
    MCX,
    SetTopBox,
    MediaServer,
    ProtableVideoPlayer,
}

pub enum GamingDevices {
    Xbox,
    Xbox360,
    Playstation,
    GameConsole,
    PortableGamingDevice,
}

pub enum Telephone {
    WindowsMobile,
    PhoneSingleMode,
    PhoneDualMode,
    SmartphoneSingleMode,
    SmartphoneDualMode,
}

pub enum AudioDevices {
    AutioTunerReceiver,
    Speakers,
    PortableMusicPlayer,
    Headset,
    Headphones,
    Microphone,
    HomeTheaterSystems,
}

pub enum DockingDevices {
    ComputerDockingStation,
    MediaKiosk,
}

impl fmt::Display for Computers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Computers::PC => write!(f, "PC"),
            Computers::Server => write!(f, "Server"),
            Computers::MediaCenter => write!(f, "Media Center"),
            Computers::UltraMobilePC => write!(f, "Ultra Mobile PC"),
            Computers::Notebook => write!(f, "Notebook"),
            Computers::Desktop => write!(f, "Desktop"),
            Computers::MID => write!(f, "Mobile Internet Device"),
            Computers::Netbook => write!(f, "Netbook"),
            Computers::Tablet => write!(f, "Tablet"),
            Computers::Ultrabook => write!(f, "Ultrabook"),
        }
    }
}

impl fmt::Display for InputDevices {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InputDevices::Keyboard => write!(f, "Keyboard"),
            InputDevices::Mouse => write!(f, "Mouse"),
            InputDevices::Joystick => write!(f, "Joystick"),
            InputDevices::Trackball => write!(f, "Trackball"),
            InputDevices::GamingController => write!(f, "Gaming Controller"),
            InputDevices::Remote => write!(f, "Input Remote"),
            InputDevices::Touchscreen => write!(f, "Touchscreen"),
            InputDevices::BiometricReader => write!(f, "Biometric Reader"),
            InputDevices::BarcodeReader => write!(f, "Barcode Reader"),
        }
    }
}

impl fmt::Display for PrintersEtAl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PrintersEtAl::Printer => write!(f, "Printer"),
            PrintersEtAl::Scanner => write!(f, "Scanner"),
            PrintersEtAl::Fax => write!(f, "Fax Machine"),
            PrintersEtAl::Copier => write!(f, "Copier"),
            PrintersEtAl::AllInOne => write!(f, "All-In-One Printer"),
        }
    }
}

impl fmt::Display for Cameras {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Cameras::DigitalCamera => write!(f, "Digital Camera"),
            Cameras::VideoCamera => write!(f, "Video Camera"),
            Cameras::Webcam => write!(f, "Webcam"),
            Cameras::SecurityCamera => write!(f, "Security Camera"),
        }
    }
}

impl fmt::Display for Storage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Storage::NAS => write!(f, "NAS"),
        }
    }
}

impl fmt::Display for NetworkInfrastructure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NetworkInfrastructure::AP => write!(f, "Access Point"),
            NetworkInfrastructure::Router => write!(f, "Router"),
            NetworkInfrastructure::Switch => write!(f, "Network Switch"),
            NetworkInfrastructure::Gateway => write!(f, "Network Gateway"),
            NetworkInfrastructure::Bridge => write!(f, "Network Bridge"),
        }
    }
}

impl fmt::Display for Displays {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Displays::Television => write!(f, "Television"),
            Displays::ElectronicPictureFrame => write!(f, "Electronic Picture Frame"),
            Displays::Projector => write!(f, "Projector"),
            Displays::Monitor => write!(f, "Monitor"),
        }
    }
}

impl fmt::Display for MultimediaDevices {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MultimediaDevices::DAR => write!(f, "Digital Audio Recorder"),
            MultimediaDevices::PVR => write!(f, "Personal Video Recorder"),
            MultimediaDevices::MCX => write!(f, "Media Center Extender"),
            MultimediaDevices::SetTopBox => write!(f, "Set-Top Box"),
            MultimediaDevices::MediaServer => write!(f, "Media Server"),
            MultimediaDevices::ProtableVideoPlayer => write!(f, "Portable Video Player"),
        }
    }
}

impl fmt::Display for GamingDevices {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GamingDevices::Xbox => write!(f, "Xbox"),
            GamingDevices::Xbox360 => write!(f, "Xbox 360"),
            GamingDevices::Playstation => write!(f, "Playstation"),
            GamingDevices::GameConsole => write!(f, "Game Console"),
            GamingDevices::PortableGamingDevice => write!(f, "Portable Gaming Device"),
        }
    }
}

impl fmt::Display for Telephone {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Telephone::WindowsMobile => write!(f, "Windows Mobile"),
            Telephone::PhoneSingleMode => write!(f, "Phone Single Mode"),
            Telephone::PhoneDualMode => write!(f, "Phone Dual Mode"),
            Telephone::SmartphoneSingleMode => write!(f, "Smartphone Single Mode"),
            Telephone::SmartphoneDualMode => write!(f, "Smartphone Dual Mode"),
        }
    }
}

impl fmt::Display for AudioDevices {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AudioDevices::AutioTunerReceiver => write!(f, "Audio Tuner Receiver"),
            AudioDevices::Speakers => write!(f, "Speakers"),
            AudioDevices::PortableMusicPlayer => write!(f, "Portable Music Player"),
            AudioDevices::Headset => write!(f, "Headset"),
            AudioDevices::Headphones => write!(f, "Headphones"),
            AudioDevices::Microphone => write!(f, "Microphone"),
            AudioDevices::HomeTheaterSystems => write!(f, "Home Theater Systems"),
        }
    }
}

impl fmt::Display for DockingDevices {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DockingDevices::ComputerDockingStation => write!(f, "Computer Docking Station"),
            DockingDevices::MediaKiosk => write!(f, "Media Kiosk"),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct VendorSpecificInfo {
    pub element_id: u8,
    pub length: u8,
    pub oui: [u8; 3],
    pub oui_type: u8,
    pub data: Vec<u8>,
}

impl VendorSpecificInfo {
    pub fn encode(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.push(self.element_id);
        bytes.push(self.length);
        bytes.extend_from_slice(&self.oui);
        bytes.push(self.oui_type);
        bytes.extend(&self.data);

        bytes
    }
}

#[derive(Clone, Debug, Default)]
pub struct WpsInformation {
    pub setup_state: WpsSetupState,
    pub manufacturer: String,
    pub model: String,
    pub model_number: String,
    pub serial_number: String,
    pub primary_device_type: String,
    pub device_name: String,
}

impl WpsInformation {
    pub fn update_with(&mut self, other: &WpsInformation) {
        if other.setup_state != WpsSetupState::NotConfigured {
            self.setup_state = other.setup_state;
        }

        if !other.manufacturer.is_empty() {
            self.manufacturer = other.manufacturer.clone();
        }

        if !other.model.is_empty() {
            self.model = other.model.clone();
        }
        if !other.model_number.is_empty() {
            self.model_number = other.model_number.clone();
        }
        if !other.serial_number.is_empty() {
            self.serial_number = other.serial_number.clone();
        }

        if !other.primary_device_type.is_empty() {
            self.primary_device_type = other.primary_device_type.clone();
        }

        if !other.device_name.is_empty() {
            self.device_name = other.device_name.clone();
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum WpsSetupState {
    #[default]
    NotConfigured = 0x01,
    Configured = 0x02,
}

impl Display for WpsSetupState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let output = match self {
            WpsSetupState::NotConfigured => "Not Configured",
            WpsSetupState::Configured => "Configured",
        };
        write!(f, "{output}")
    }
}

#[derive(Clone, Debug, Default)]
pub struct WpaInformation {
    pub version: u16,
    pub multicast_cipher_suite: WpaCipherSuite,
    pub unicast_cipher_suites: Vec<WpaCipherSuite>,
    pub akm_suites: Vec<WpaAkmSuite>,
}

impl WpaInformation {
    pub fn encode(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        // Encode version
        bytes.extend_from_slice(&self.version.to_le_bytes());

        // Encode Multicast Cipher Suite
        bytes.extend(self.multicast_cipher_suite.encode());

        // Encode Unicast Cipher Suites
        bytes.extend_from_slice(&(self.unicast_cipher_suites.len() as u16).to_le_bytes());
        for suite in &self.unicast_cipher_suites {
            bytes.extend(suite.encode());
        }

        // Encode AKM Suites
        bytes.extend_from_slice(&(self.akm_suites.len() as u16).to_le_bytes());
        for suite in &self.akm_suites {
            bytes.extend(suite.encode());
        }

        bytes
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum WpaCipherSuite {
    Wep40,
    Wep104,
    Tkip,
    #[default]
    Ccmp,
    Unknown(Vec<u8>),
}

impl WpaCipherSuite {
    pub fn encode(&self) -> Vec<u8> {
        match self {
            WpaCipherSuite::Wep40 => vec![0x00, 0x50, 0xF2, 0x01],
            WpaCipherSuite::Wep104 => vec![0x00, 0x50, 0xF2, 0x05],
            WpaCipherSuite::Tkip => vec![0x00, 0x50, 0xF2, 0x02],
            WpaCipherSuite::Ccmp => vec![0x00, 0x50, 0xF2, 0x04],
            WpaCipherSuite::Unknown(data) => data.clone(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum WpaAkmSuite {
    #[default]
    Psk, // Typically represented as 00-50-F2-1
    Eap,              // Typically represented as 00-50-F2-2
    Unknown(Vec<u8>), // For any unrecognized AKM suites
}

impl WpaAkmSuite {
    pub fn encode(&self) -> Vec<u8> {
        match self {
            WpaAkmSuite::Psk => vec![0x00, 0x50, 0xF2, 0x01],
            WpaAkmSuite::Eap => vec![0x00, 0x50, 0xF2, 0x02],
            WpaAkmSuite::Unknown(data) => data.clone(),
        }
    }
}

// Define the RsnInformation struct to hold the parsed data
#[derive(Clone, Debug, Default)]
pub struct RsnInformation {
    pub version: u16,
    pub group_cipher_suite: RsnCipherSuite,
    pub pairwise_cipher_suites: Vec<RsnCipherSuite>,
    pub akm_suites: Vec<RsnAkmSuite>,
    // RSN Capabilities Flags
    pub pre_auth: bool,
    pub no_pairwise: bool,
    pub ptksa_replay_counter: u8,
    pub gtksa_replay_counter: u8,
    pub mfp_required: bool,
    pub mfp_capable: bool,
    pub joint_multi_band_rsna: bool,
    pub peerkey_enabled: bool,
    pub extended_key_id: bool,
    pub ocvc: bool,
}

impl RsnInformation {
    pub fn encode(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        // Encode version
        bytes.extend_from_slice(&self.version.to_le_bytes());

        // Encode Group Cipher Suite
        bytes.extend(self.group_cipher_suite.encode());

        // Encode Pairwise Cipher Suites
        bytes.extend_from_slice(&(self.pairwise_cipher_suites.len() as u16).to_le_bytes());
        for suite in &self.pairwise_cipher_suites {
            bytes.extend(suite.encode());
        }

        // Encode AKM Suites
        bytes.extend_from_slice(&(self.akm_suites.len() as u16).to_le_bytes());
        for suite in &self.akm_suites {
            bytes.extend(suite.encode());
        }

        // Encode RSN Capabilities
        let mut rsn_capabilities: u16 = 0;
        rsn_capabilities |= self.pre_auth as u16;
        rsn_capabilities |= (self.no_pairwise as u16) << 1;
        rsn_capabilities |= ((self.ptksa_replay_counter & 0x03) as u16) << 2;
        rsn_capabilities |= ((self.gtksa_replay_counter & 0x03) as u16) << 3;
        rsn_capabilities |= (self.mfp_required as u16) << 6;
        rsn_capabilities |= (self.mfp_capable as u16) << 7;
        rsn_capabilities |= (self.joint_multi_band_rsna as u16) << 8;
        rsn_capabilities |= (self.peerkey_enabled as u16) << 9;
        rsn_capabilities |= (self.extended_key_id as u16) << 13;
        rsn_capabilities |= (self.ocvc as u16) << 14;

        bytes.extend_from_slice(&rsn_capabilities.to_le_bytes());

        bytes
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum RsnAkmSuite {
    #[default]
    PSK,
    EAP,
    PSKFT,
    EAPFT,
    SAE,
    SUITEBEAP256,
    PSK256,
    EAP256,
    Unknown(Vec<u8>),
}

impl RsnAkmSuite {
    pub fn encode(&self) -> Vec<u8> {
        match self {
            RsnAkmSuite::EAP => vec![0x00, 0x0F, 0xAC, 0x01],
            RsnAkmSuite::PSK => vec![0x00, 0x0F, 0xAC, 0x02],
            RsnAkmSuite::EAPFT => vec![0x00, 0x0F, 0xAC, 0x03],
            RsnAkmSuite::PSKFT => vec![0x00, 0x0F, 0xAC, 0x04],
            RsnAkmSuite::EAP256 => vec![0x00, 0x0F, 0xAC, 0x05],
            RsnAkmSuite::PSK256 => vec![0x00, 0x0F, 0xAC, 0x06],
            RsnAkmSuite::SAE => vec![0x00, 0x0F, 0xAC, 0x08],
            RsnAkmSuite::SUITEBEAP256 => vec![0x00, 0x0F, 0xAC, 0x0b],
            RsnAkmSuite::Unknown(data) => data.clone(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum RsnCipherSuite {
    None,
    WEP,
    TKIP,
    WRAP,
    #[default]
    CCMP,
    WEP104,
    Unknown(Vec<u8>),
}

impl RsnCipherSuite {
    pub fn encode(&self) -> Vec<u8> {
        match self {
            RsnCipherSuite::None => vec![0x00, 0x0F, 0xAC, 0x00],
            RsnCipherSuite::WEP => vec![0x00, 0x0F, 0xAC, 0x01],
            RsnCipherSuite::TKIP => vec![0x00, 0x0F, 0xAC, 0x02],
            RsnCipherSuite::WRAP => vec![0x00, 0x0F, 0xAC, 0x03],
            RsnCipherSuite::CCMP => vec![0x00, 0x0F, 0xAC, 0x04],
            RsnCipherSuite::WEP104 => vec![0x00, 0x0F, 0xAC, 0x05],
            RsnCipherSuite::Unknown(data) => data.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct HTInformation {
    pub primary_channel: u8,
    pub other_data: Vec<u8>, // TODO
}

impl HTInformation {
    pub fn encode(&self) -> Vec<u8> {
        let mut data: Vec<u8> = Vec::new();
        data.push(self.primary_channel);
        data.extend(self.other_data.clone());
        data
    }
}

#[derive(Debug, Clone)]
pub struct MultipleBSSID {
    pub max_bssid_indicator: u8,
    pub other_data: Vec<u8>,
}

impl MultipleBSSID {
    pub fn encode(&self) -> Vec<u8> {
        let mut data: Vec<u8> = Vec::new();
        data.push(self.max_bssid_indicator);
        data.extend(&self.other_data);
        data
    }
}

#[derive(Debug, Clone)]
pub struct ChannelSwitchAnnouncment {
    pub mode: ChannelSwitchMode,
    pub new_channel: u8,
    pub count: u8,
}

impl ChannelSwitchAnnouncment {
    pub fn encode(&self) -> Vec<u8> {
        vec![self.mode.clone() as u8, self.new_channel, self.count]
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChannelSwitchMode {
    Restrict = 1,
    Unrestricted = 0,
}

impl ChannelSwitchMode {
    pub fn from_u8(value: u8) -> ChannelSwitchMode {
        match value {
            1 => ChannelSwitchMode::Restrict,
            _ => ChannelSwitchMode::Unrestricted,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtendedCapabilities {
    // Extended capabilities elements per 80211ax-2021
    pub bss_coexistence_management_support: bool, // bit 0
    pub glk: bool,                                // 1
    pub extended_channel_switching: bool,         // 2
    pub glk_gcr: bool,                            // 3
    pub psmp_capability: bool,                    // 4
    //pub reserved5: bool,                          // 5
    pub s_psmp_capability: bool,                 // 6
    pub event: bool,                             // 7
    pub diagnostics: bool,                       // 8
    pub multicast_diagnostics: bool,             // 9
    pub location_tracking: bool,                 // 10
    pub fms: bool,                               // 11
    pub proxy_arp_service: bool,                 // 12
    pub collocated_interference_reporting: bool, // 13
    pub civic_location: bool,                    // 14
    pub geospatial_location: bool,               // 15
    pub tfs: bool,                               // 16
    pub wnm_sleep_mode: bool,                    // 17
    pub tim_broadcast: bool,                     // 18
    pub bss_transition: bool,                    // 19
    pub qos_traffic_capability: bool,            // 20
    pub ac_station_count: bool,                  // 21
    pub multiple_bssid: bool,                    // 22
    pub timing_measurement: bool,                // 23
    pub channel_usage: bool,                     // 24
    pub ssid_list: bool,                         // 25
    pub dms: bool,                               // 26
    pub utc_tsf_offset: bool,                    // 27
    pub tpu_buffer_sta_support: bool,            // 28
    pub tdls_peer_psm_support: bool,             // 29
    pub tdls_channel_switching: bool,            // 30
    pub internetworking: bool,                   // 31
    pub qos_map: bool,                           // 32
    pub ebr: bool,                               // 33
    pub sspn_interface: bool,                    // 34
    //pub reserved35: bool,                         // 35
    pub msgcf_capability: bool,                  // 36
    pub tdls_support: bool,                      // 37
    pub tdls_prohibited: bool,                   // 38
    pub tdls_channel_switching_prohibited: bool, // 39
    pub reject_unadmitted_frame: bool,           // 40
    pub service_interval_granularity: u8,        // 41,42,43
    pub identifier_location: bool,               // 44
    pub uapsd_coexistence: bool,                 // 45
    pub wnm_notification: bool,                  // 46
    pub qab_capability: bool,                    // 47
    pub utf8_ssid: bool,                         // 48
    pub qmf_activated: bool,                     // 49
    pub qmf_reconfiguration_activated: bool,     // 50
    pub robust_av_streaming: bool,               // 51
    pub advanced_gcr: bool,                      // 52
    pub mesh_gcr: bool,                          // 53
    pub scs: bool,                               // 54
    pub qload_report: bool,                      // 55
    pub alternate_edca: bool,                    // 56
    pub unprotected_txop_negotiation: bool,      // 57
    pub protected_txop_negotiation: bool,        // 58
    //pub reserved59: bool,                         // 59
    pub protected_qload_report: bool,               // 60
    pub tdls_wider_bandwidth: bool,                 // 61
    pub operating_mode_notification: bool,          // 62
    pub max_number_of_msdus_in_amsdu: u8,           // 63, 64
    pub channel_schedule_management: bool,          // 65
    pub geodatabase_inband_enabling_signal: bool,   // 66
    pub network_channel_control: bool,              // 67
    pub white_space_map: bool,                      // 68
    pub channel_availability_query: bool,           // 69
    pub fine_timing_measurement_responder: bool,    // 70
    pub fine_timing_measurement_initiator: bool,    // 71
    pub fils_capability: bool,                      // 72
    pub extended_spectrum_management_capable: bool, // 73
    pub future_channel_guidance: bool,              // 74
    pub pad: bool,                                  // 75
    //pub reserved76: bool,                         // 76
    pub twt_requester_support: bool, // 77
    pub twt_responder_support: bool, // 78
    pub obss_narrow_bandwidth_ru_in_odfma_tolerance_support: bool, // 79
    pub complete_list_of_nontxbssid_profiles: bool, // 80
    pub sae_password_in_use: bool,   // 81
    pub sae_password_used_exclusively: bool, // 82
    pub enhanced_multibssid_advertisement_support: bool, // 83
    pub beacon_protection_enabled: bool, // 84
    pub mirrored_scs: bool,          // 85
    pub oct: bool,                   // 86
    pub local_mac_address_policy: bool, // 87
    //pub reserved88: bool,
    pub twt_parameters_range_support: bool, // 89
}

impl ExtendedCapabilities {
    pub fn encode(&self) -> Vec<u8> {
        let mut b = BitVec::<usize, Lsb0>::repeat(false, 90);

        b.set(0, self.bss_coexistence_management_support);
        b.set(1, self.glk);
        b.set(2, self.extended_channel_switching);
        b.set(3, self.glk_gcr);
        b.set(4, self.psmp_capability);
        //b.set(5,self.reserved5);
        b.set(6, self.s_psmp_capability);
        b.set(7, self.event);
        b.set(8, self.diagnostics);
        b.set(9, self.multicast_diagnostics);
        b.set(10, self.location_tracking);
        b.set(11, self.fms);
        b.set(12, self.proxy_arp_service);
        b.set(13, self.collocated_interference_reporting);
        b.set(14, self.civic_location);
        b.set(15, self.geospatial_location);
        b.set(16, self.tfs);
        b.set(17, self.wnm_sleep_mode);
        b.set(18, self.tim_broadcast);
        b.set(19, self.bss_transition);
        b.set(20, self.qos_traffic_capability);
        b.set(21, self.ac_station_count);
        b.set(22, self.multiple_bssid);
        b.set(23, self.timing_measurement);
        b.set(24, self.channel_usage);
        b.set(25, self.ssid_list);
        b.set(26, self.dms);
        b.set(27, self.utc_tsf_offset);
        b.set(28, self.tpu_buffer_sta_support);
        b.set(29, self.tdls_peer_psm_support);
        b.set(30, self.tdls_channel_switching);
        b.set(31, self.internetworking);
        b.set(32, self.qos_map);
        b.set(33, self.ebr);
        b.set(34, self.sspn_interface);
        //b.set(35,self.reserved35);
        b.set(36, self.msgcf_capability);
        b.set(37, self.tdls_support);
        b.set(38, self.tdls_prohibited);
        b.set(39, self.tdls_channel_switching_prohibited);
        b.set(40, self.reject_unadmitted_frame);
        b[41..43].store_le::<u8>(self.service_interval_granularity);
        b.set(44, self.identifier_location);
        b.set(45, self.uapsd_coexistence);
        b.set(46, self.wnm_notification);
        b.set(47, self.qab_capability);
        b.set(48, self.utf8_ssid);
        b.set(49, self.qmf_activated);
        b.set(50, self.qmf_reconfiguration_activated);
        b.set(51, self.robust_av_streaming);
        b.set(52, self.advanced_gcr);
        b.set(53, self.mesh_gcr);
        b.set(54, self.scs);
        b.set(55, self.qload_report);
        b.set(56, self.alternate_edca);
        b.set(57, self.unprotected_txop_negotiation);
        b.set(58, self.protected_txop_negotiation);
        //b.set(59]=self.reserved59);
        b.set(60, self.protected_qload_report);
        b.set(61, self.tdls_wider_bandwidth);
        b.set(62, self.operating_mode_notification);
        b[63..64].store_le::<u8>(self.max_number_of_msdus_in_amsdu);
        b.set(65, self.channel_schedule_management);
        b.set(66, self.geodatabase_inband_enabling_signal);
        b.set(67, self.network_channel_control);
        b.set(68, self.white_space_map);
        b.set(69, self.channel_availability_query);
        b.set(70, self.fine_timing_measurement_responder);
        b.set(71, self.fine_timing_measurement_initiator);
        b.set(72, self.fils_capability);
        b.set(73, self.extended_spectrum_management_capable);
        b.set(74, self.future_channel_guidance);
        b.set(75, self.pad);
        //b.set(76]=self.reserved76);
        b.set(77, self.twt_requester_support);
        b.set(78, self.twt_responder_support);
        b.set(79, self.obss_narrow_bandwidth_ru_in_odfma_tolerance_support);
        b.set(80, self.complete_list_of_nontxbssid_profiles);
        b.set(81, self.sae_password_in_use);
        b.set(82, self.sae_password_used_exclusively);
        b.set(83, self.enhanced_multibssid_advertisement_support);
        b.set(84, self.beacon_protection_enabled);
        b.set(85, self.mirrored_scs);
        b.set(86, self.oct);
        b.set(87, self.local_mac_address_policy);
        //b.set(88]=self.reserved88);
        b.set(89, self.twt_parameters_range_support);

        // Remove trailing zeros and convert to [u8], then vec
        vec![b[0..b.len() - b.trailing_zeros()].load_le::<u8>()]
    }
}
