use lazy_static::lazy_static;

/// Maximum number of fields in a RINEX observation record
pub(super) const MAX_FIELDS_COUNT: usize = 62;

lazy_static! {
    /// GPS code fields
    pub(super) static ref GPS_FIELDS: Vec<&'static str> = vec![
        "C1C", "L1C", "D1C", "S1C", "C1W", "S1W", "C2W", "L2W", "D2W", "S2W", "C2L", "L2L", "D2L",
        "S2L", "C5Q", "L5Q", "D5Q", "S5Q", "D1W", "L1W", "C2X", "C5X", "D2X", "D5X", "L2X", "L5X",
        "S2X", "S5X", "C2S", "L2S", "D2S", "S2S", "C1L", "L1L", "D1L", "S1L", "C1X", "L1X", "S1X",
        "D1X", "C1P", "L1P", "S1P", "C2C", "L2C", "S2C", "C2P", "L2P", "S2P", "C5I", "L5I", "S5I",
        "C2Y", "D2Y", "L2Y", "S2Y", "D5I", "D2C", "D2P"
    ];
    /// GLONASS code fields
    pub(super) static ref GLONASS_FIELDS: Vec<&'static str> = vec![
        "C1C", "L1C", "D1C", "S1C", "C1P", "L1P", "D1P", "S1P", "C2P", "L2P", "D2P", "S2P", "C2C",
        "L2C", "D2C", "S2C", "C3Q", "L3Q", "D3Q", "S3Q", "C3X", "L3X", "S3X", "D3X"
    ];
    /// BeiDou code fields
    pub(super) static ref BEIDOU_FIELDS: Vec<&'static str> = vec![
        "C2I", "L2I", "D2I", "S2I", "C7I", "L7I", "D7I", "S7I", "C6I", "L6I", "D6I", "S6I", "C1I",
        "L1I", "D1I", "S1I", "C1X", "L1X", "S1X", "C5X", "L5X", "S5X", "C1P", "L1P", "D1P", "S1P",
        "C5P", "L5P", "D5P", "S5P", "C7Z", "C8X", "D5X", "D7Z", "D8X", "L7Z", "L8X", "S7Z", "S8X",
        "C2X", "D2X", "L2X", "S2X", "D1X", "C6X", "L6X", "S6X", "C7X", "L7X", "S7X", "C1D", "C5D",
        "D1D", "D5D", "L1D", "L5D", "S1D", "S5D", "C7D", "L7D", "S7D", "D7D"
    ];
    /// SBAS code fields
    pub(super) static ref SBAS_FIELDS: Vec<&'static str> =
        vec!["C1C", "L1C", "D1C", "S1C", "C5I", "L5I", "D5I", "S5I", "C5X", "L5X", "S5X"];
    /// Galileo code fields
    pub(super) static ref GALILEO_FIELDS: Vec<&'static str> = vec![
        "C1C", "L1C", "D1C", "S1C", "C6C", "L6C", "D6C", "S6C", "C5Q", "L5Q", "D5Q", "S5Q", "C7Q",
        "L7Q", "D7Q", "S7Q", "C8Q", "L8Q", "D8Q", "S8Q", "C1X", "C5X", "C7X", "D1X", "D5X", "D7X",
        "L1X", "L5X", "L7X", "S1X", "S5X", "S7X", "C8X", "D8X", "L8X", "S8X", "C6X", "L6X", "S6X",
        "D6X", "C1B", "L1B", "S1B", "C5I", "L5I", "S5I", "C6B", "L6B", "S6B", "C8I", "L8I", "S8I",
        "C7I", "L7I", "S7I", "D1B", "D5I", "D7I"
    ];
    /// QZSS code fields
    pub(super) static ref QZSS_FIELDS: Vec<&'static str> = vec![
        "C1C", "L1C", "D1C", "S1C", "C2L", "L2L", "D2L", "S2L", "C5Q", "L5Q", "D5Q", "S5Q", "C2S",
        "L2S", "D2S", "S2S", "C2X", "L2X", "S2X", "S6X", "C5X", "L5X", "S5X", "C1X", "L1X", "S1X",
        "C1Z", "L1Z", "S1Z", "C6X", "L6X", "C1L", "L1L", "D1L", "S1L", "D1Z", "D2X", "D5X", "D1X",
        "C6L", "L6L", "S6L", "D6X", "C6Z", "L6Z", "S6Z", "D6Z", "C1B", "L1B", "S1B", "C5P", "L5P",
        "D5P", "S5P"
    ];
    /// IRNSS code fields
    pub(super) static ref IRNSS_FIELDS: Vec<&'static str> =
        vec!["C5A", "L5A", "D5A", "S5A", "C9A", "L9A", "S9A"];
}
