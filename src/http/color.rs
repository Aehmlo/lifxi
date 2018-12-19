use std::fmt;

/// Specifies the desired color setting of a light.
///
/// HSBK is the preferred method of specifying colors (RGB represents color poorly); as such,
/// `Hue`, `Saturation`, `Brightness`, and `Kelvin` are among the more useful variants here.
///
/// RGB colors will automatically be converted by the API.
pub enum ColorSetting {
    /// Sets the hue and saturation components necessary to change the color to red, leaving
    /// brightness untouched.
    Red,
    /// Sets the hue and saturation components necessary to change the color to orange, leaving
    /// brightness untouched.
    Orange,
    /// Sets the hue and saturation components necessary to change the color to yellow, leaving
    /// brightness untouched.
    Yellow,
    /// Sets the hue and saturation components necessary to change the color to green, leaving
    /// brightness untouched.
    Green,
    /// Sets the hue and saturation components necessary to change the color to blue, leaving
    /// brightness untouched.
    Blue,
    /// Sets the hue and saturation components necessary to change the color to purple, leaving
    /// brightness untouched.
    Purple,
    /// Sets the hue and saturation components necessary to change the color to pink, leaving
    /// brightness untouched.
    Pink,
    /// Sets the hue and saturation components necessary to change the color to white, leaving
    /// brightness untouched.
    White,
    /// Sets the hue, leaving all else untouched.
    ///
    /// The hue should be between 0 and 360.
    Hue(u16),
    /// Sets the saturation, leaving all else untouched.
    ///
    /// The saturation should be between 0 and 1.
    Saturation(f32),
    /// Sets the brightness, leaving all else untouched.
    ///
    /// The brightness should be between 0 and 1.
    Brightness(f32),
    /// Sets the temperature to the given value and saturation to 0, leaving all else untouched.
    ///
    /// The temperature should be between 1500 and 9000.
    Kelvin(u16),
    /// Sets the color to an RGB color using the given numeric components.
    ///
    /// It is preferred to use this over `RgbStr` where posssible.
    Rgb([u8; 3]),
    /// Sets the color to an RGB color using the given specifier string.
    ///
    /// Strings may be of the form `#ff0000` or `ff0000`; outputs will be normalized to the former.
    ///
    /// It is preferred to use [`Rgb`](#variant.Rgb) instead of this where posssible.
    RgbStr(String),
}

/// Represents a (local) color validation error.
#[derive(Debug)]
pub enum Error {
    /// The given hue was greater than the maximum hue of 360.
    Hue(u16),
    /// The given saturation was greater than 1.0.
    SaturationHigh(f32),
    /// The given saturation was less than 0.0.
    SaturationLow(f32),
    /// The given brightness was greater than 1.0.
    BrightnessHigh(f32),
    /// The given brightness was less than 0.0.
    BrightnessLow(f32),
    /// The given temperature was greater than 9000 K.
    KelvinHigh(u16),
    /// The given temperature was less than 1500 K.
    KelvinLow(u16),
    /// The given RGB string was too short.
    RgbStrShort(bool, String),
    /// The given RGB string was too long.
    RgbStrLong(bool, String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Hue(hue) => write!(f, "Hue {} is too large (max: 360).", hue),
            Error::SaturationHigh(sat) => write!(f, "Saturation {} is too large (max: 1.0).", sat),
            Error::SaturationLow(sat) => write!(f, "Saturation {} is negative.", sat),
            Error::BrightnessHigh(b) => write!(f, "Brightness {} is too large (max: 1.0).", b),
            Error::BrightnessLow(b) => write!(f, "Brightness {} is negative.", b),
            Error::KelvinHigh(t) => write!(f, "Temperature {} K is too large (max: 9000 K).", t),
            Error::KelvinLow(t) => write!(f, "Temperature {} K is too small (min: 1500 K).", t),
            Error::RgbStrShort(h, s) => write!(
                f,
                "RGB string {} is too short ({} chars; expected {}).",
                s,
                s.len(),
                if *h { 7 } else { 6 }
            ),
            Error::RgbStrLong(h, s) => write!(
                f,
                "RGB string {} is too long ({} chars; expected {}).",
                s,
                s.len(),
                if *h { 7 } else { 6 }
            ),
        }
    }
}

impl ::std::error::Error for Error {}

impl ColorSetting {
    /// Checks whether the color is valid.
    ///
    /// ### Examples
    /// ```
    /// use lifx::http::ColorSetting;
    /// // Too short
    /// let setting = ColorSetting::RgbStr("".to_string());
    /// assert!(setting.validate().is_err());
    /// // Too long for no leading #
    /// let setting = ColorSetting::RgbStr("1234567".to_string());
    /// assert!(setting.validate().is_err());
    /// // Too high (max 9000)
    /// let setting = ColorSetting::Kelvin(10_000);
    /// assert!(setting.validate().is_err());
    /// // Too high (max 1.0)
    /// let setting = ColorSetting::Brightness(1.2);
    /// assert!(setting.validate().is_err());
    /// // Too low (min 0.0)
    /// let setting = ColorSetting::Saturation(-0.1);
    /// assert!(setting.validate().is_err());
    /// let setting = ColorSetting::Kelvin(2_000);
    /// assert!(setting.validate().is_ok());
    /// ```
    pub fn validate(&self) -> Result<(), Error> {
        use self::ColorSetting::*;
        use self::Error::*;
        match self {
            Red | Orange | Yellow | Green | Blue | Purple | Pink | White | Rgb(_) => Ok(()),
            self::ColorSetting::Hue(hue) => {
                if *hue > 360 {
                    Err(self::Error::Hue(*hue))
                } else {
                    Ok(())
                }
            }
            Saturation(s) => {
                if *s > 1.0 {
                    Err(SaturationHigh(*s))
                } else if *s < 0.0 {
                    Err(SaturationLow(*s))
                } else {
                    Ok(())
                }
            }
            Brightness(b) => {
                if *b > 1.0 {
                    Err(BrightnessHigh(*b))
                } else if *b < 0.0 {
                    Err(BrightnessLow(*b))
                } else {
                    Ok(())
                }
            }
            Kelvin(t) => {
                if *t < 1500 {
                    Err(KelvinLow(*t))
                } else if *t > 9000 {
                    Err(KelvinHigh(*t))
                } else {
                    Ok(())
                }
            }
            RgbStr(s) => {
                if s.starts_with('#') {
                    if s.len() > 7 {
                        Err(RgbStrLong(true, s.clone()))
                    } else if s.len() < 7 {
                        Err(RgbStrShort(true, s.clone()))
                    } else {
                        Ok(())
                    }
                } else if s.len() > 6 {
                    Err(RgbStrLong(false, s.clone()))
                } else if s.len() < 6 {
                    Err(RgbStrShort(false, s.clone()))
                } else {
                    Ok(())
                }
            }
        }
    }
}
