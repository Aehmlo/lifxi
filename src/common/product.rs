/// Represents a LIFX product.
#[allow(missing_docs)]
pub enum Product {
    Original1000,
    Color650,
    White800LV,
    White800HV,
    White900BR30,
    Color1000BR30,
    Color1000,
    LIFXA19,
    LIFXBR30,
    LIFXPlusA19,
    LIFXPlusBR30,
    LIFXZ,
    LIFXZ2,
    LIFXDownlight,
    LIFXBeam,
    LIFXMini,
    LIFXMiniDayDusk,
    LIFXMiniWhite,
    LIFXGU10,
    LIFXTile,
}

impl Product {
    /// Gives the vendor ID of this product.
    pub const fn vid(&self) -> u32 {
        1
    }
    /// Gives the vendor ID of this product.
    pub fn pid(&self) -> u32 {
        use self::Product::*;
        match self {
            Original1000 => 1,
            Color650 => 3,
            White800LV => 10,
            White800HV => 11,
            White900BR30 => 18,
            Color1000BR30 => 20,
            Color1000 => 22,
            LIFXA19 => 27,
            LIFXBR30 => 28,
            LIFXPlusA19 => 29,
            LIFXPlusBR30 => 30,
            LIFXZ => 31,
            LIFXZ2 => 32,
            LIFXDownlight => 36,
            LIFXBeam => 38,
            LIFXMini => 49,
            LIFXMiniDayDusk => 50,
            LIFXMiniWhite => 51,
            LIFXGU10 => 52,
            LIFXTile => 55,
        }
    }
    /// Gives the consumer-friendly name of this product.
    pub fn name(&self) -> &'static str {
        use self::Product::*;
        match self {
            Original1000 => "Original 1000",
            Color650 => "Color 650",
            White800LV => "White 800 (Low Voltage)",
            White800HV => "White 800 (High Voltage)",
            White900BR30 => "White 900 BR30 (Low Voltage)",
            Color1000BR30 => "Color 1000 BR30",
            Color1000 => "Color 1000",
            LIFXA19 => "LIFX A19",
            LIFXBR30 => "LIFX BR30",
            LIFXPlusA19 => "LIFX+ A19",
            LIFXPlusBR30 => "LIFX+ BR30",
            LIFXZ => "LIFX Z",
            LIFXZ2 => "LIFX Z 2",
            LIFXDownlight => "LIFX Downlight",
            LIFXBeam => "LIFX Beam",
            LIFXMini => "LIFX Mini",
            LIFXMiniDayDusk => "LIFX Mini Day and Dusk",
            LIFXMiniWhite => "LIFX Mini White",
            LIFXGU10 => "LIFX GU10",
            LIFXTile => "LIFX Tile",
        }
    }
    /// Indicates whether this product has color support.
    pub fn color(&self) -> bool {
        use self::Product::*;
        match self {
            White800LV | White800HV | White900BR30 | LIFXMiniDayDusk | LIFXMiniWhite => false,
            _ => true,
        }
    }
    /// Indicates whether this product has infrared support.
    pub fn infrared(&self) -> bool {
        use self::Product::*;
        match self {
            LIFXPlusBR30 | LIFXPlusA19 => true,
            _ => false,
        }
    }
    /// Indicates whether this product supports multizoning.
    pub fn multizone(&self) -> bool {
        use self::Product::*;
        match self {
            LIFXZ | LIFXZ2 | LIFXBeam => true,
            _ => false,
        }
    }
}
