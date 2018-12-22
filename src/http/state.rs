use std::fmt;
use std::num::{ParseFloatError, ParseIntError};
use std::str::FromStr;
use std::time::Duration as StdDuration;

use serde::{de::Error as DeError, Deserialize, Deserializer, Serialize, Serializer};

/// Specifies the desired color setting of a light.
///
/// HSBK is the preferred method of specifying colors (RGB represents color poorly); as such,
/// `Hue`, `Saturation`, `Brightness`, and `Kelvin` are among the more useful variants here.
///
/// RGB colors will automatically be converted by the API.
#[derive(Clone, Debug, PartialEq)]
pub enum Color {
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
    /// Uses a custom specifier string.
    ///
    /// This option exists for undocumented features. For instance, "cyan" is a valid color choice,
    /// but it is undocumented and therefore (theoretically) unstable, so it is not officially/
    /// supported by this crate.
    Custom(String),
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Color::Red => write!(f, "red"),
            Color::Orange => write!(f, "orange"),
            Color::Yellow => write!(f, "yellow"),
            Color::Green => write!(f, "green"),
            Color::Blue => write!(f, "blue"),
            Color::Purple => write!(f, "purple"),
            Color::Pink => write!(f, "pink"),
            Color::White => write!(f, "white"),
            Color::Hue(hue) => write!(f, "hue:{}", hue),
            Color::Saturation(sat) => write!(f, "saturation:{}", sat),
            Color::Brightness(b) => write!(f, "brightness:{}", b),
            Color::Kelvin(t) => write!(f, "kelvin:{}", t),
            Color::Rgb(rgb) => write!(f, "rgb:{},{},{}", rgb[0], rgb[1], rgb[2]),
            Color::RgbStr(s) => {
                if s.starts_with('#') {
                    write!(f, "{}", s)
                } else {
                    write!(f, "#{}", s)
                }
            }
            Color::Custom(s) => write!(f, "{}", s),
        }
    }
}

/// Represents an error encountered while deserializing a color.
#[derive(Clone, Debug, PartialEq)]
pub enum ColorParseError {
    /// No hue was given.
    ///
    /// ## Example
    /// ```
    /// use lifx::http::*;
    /// let color = "hue:".parse::<Color>();
    /// assert_eq!(color, Err(ColorParseError::NoHue));
    /// ```
    NoHue,
    /// The hue could not be parsed as an integer.
    ///
    /// ## Example
    /// ```
    /// use lifx::http::*;
    /// let color = "hue:j".parse::<Color>();
    /// assert!(color.is_err());
    /// ```
    NonNumericHue(ParseIntError),
    /// No saturation was given.
    ///
    /// ## Example
    /// ```
    /// use lifx::http::*;
    /// let color = "saturation:".parse::<Color>();
    /// assert_eq!(color, Err(ColorParseError::NoSaturation));
    /// ```
    NoSaturation,
    /// The saturation could not be parsed as a float.
    ///
    /// ## Example
    /// ```
    /// use lifx::http::*;
    /// let color = "saturation:j".parse::<Color>();
    /// assert!(color.is_err());
    /// ```
    NonNumericSaturation(ParseFloatError),
    /// No brightness was given.
    ///
    /// ## Example
    /// ```
    /// use lifx::http::*;
    /// let color = "brightness:".parse::<Color>();
    /// assert_eq!(color, Err(ColorParseError::NoBrightness));
    /// ```
    NoBrightness,
    /// The brightness could not be parsed as a float.
    ///
    /// ## Example
    /// ```
    /// use lifx::http::*;
    /// let color = "brightness:j".parse::<Color>();
    /// assert!(color.is_err());
    /// ```
    NonNumericBrightness(ParseFloatError),
    /// No color temperature was given.
    ///
    /// ## Example
    /// ```
    /// use lifx::http::*;
    /// let color = "kelvin:".parse::<Color>();
    /// assert_eq!(color, Err(ColorParseError::NoKelvin));
    /// ```
    NoKelvin,
    /// The color temperature could not be parsed as an integer.
    ///
    /// ## Example
    /// ```
    /// use lifx::http::*;
    /// let color = "kelvin:j".parse::<Color>();
    /// assert!(color.is_err());
    /// ```
    NonNumericKelvin(ParseIntError),
    /// No red component was given.
    ///
    /// ## Example
    /// ```
    /// use lifx::http::*;
    /// let color = "rgb:".parse::<Color>();
    /// assert_eq!(color, Err(ColorParseError::NoRed));
    /// ```
    NoRed,
    /// The red component could not be parsed as an integer.
    ///
    /// ## Example
    /// ```
    /// use lifx::http::*;
    /// let color = "rgb:j".parse::<Color>();
    /// assert!(color.is_err());
    /// ```
    NonNumericRed(ParseIntError),
    /// No green component was given.
    ///
    /// ## Example
    /// ```
    /// use lifx::http::*;
    /// let color = "rgb:0,".parse::<Color>();
    /// assert_eq!(color, Err(ColorParseError::NoGreen));
    /// ```
    NoGreen,
    /// The green component could not be parsed as an integer.
    ///
    /// ## Example
    /// ```
    /// use lifx::http::*;
    /// let color = "rgb:0,j".parse::<Color>();
    /// assert!(color.is_err());
    /// ```
    NonNumericGreen(ParseIntError),
    /// No blue component was given.
    ///
    /// ## Example
    /// ```
    /// use lifx::http::*;
    /// let color = "rgb:0,1,".parse::<Color>();
    /// assert_eq!(color, Err(ColorParseError::NoBlue));
    /// ```
    NoBlue,
    /// The blue component could not be parsed as an integer.
    ///
    /// ## Example
    /// ```
    /// use lifx::http::*;
    /// let color = "rgb:0,1,j".parse::<Color>();
    /// assert!(color.is_err());
    /// ```
    NonNumericBlue(ParseIntError),
    /// The string is too short to be an RGB string and was not recognized as a keyword.
    ///
    /// ## Example
    /// ```
    /// use lifx::http::*;
    /// let color = "foo".parse::<Color>();
    /// assert_eq!(color, Err(ColorParseError::ShortString));
    /// ```
    ShortString,
    /// The string is too long to be an RGB string and was not recognized as a keyword.
    ///
    /// ## Example
    /// ```
    /// use lifx::http::*;
    /// let color = "foobarbaz".parse::<Color>();
    /// assert_eq!(color, Err(ColorParseError::LongString));
    /// ```
    LongString,
}

impl fmt::Display for ColorParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::ColorParseError::*;
        match self {
            NoHue => write!(f, "Expected hue after hue: label."),
            NonNumericHue(e) => write!(f, "Failed to parse hue as integer: {}", e),
            NoSaturation => write!(f, "Expected saturation after saturation: label."),
            NonNumericSaturation(e) => write!(f, "Failed to parse saturation as float: {}", e),
            NoBrightness => write!(f, "Expected brightness after brightness: label."),
            NonNumericBrightness(e) => write!(f, "Failed to parse brightness as float: {}", e),
            NoKelvin => write!(f, "Expected color temperature after kelvin: label."),
            NonNumericKelvin(e) => write!(f, "Failed to parse color temperature as integer: {}", e),
            NoRed => write!(f, "Expected red component after rgb: label."),
            NonNumericRed(e) => write!(f, "Failed to parse red component as integer: {}", e),
            NoGreen => write!(f, "Expected green component after comma."),
            NonNumericGreen(e) => write!(f, "Failed to parse green component as integer: {}", e),
            NoBlue => write!(f, "Expected blue component after comma."),
            NonNumericBlue(e) => write!(f, "Failed to parse blue component as integer: {}", e),
            ShortString => write!(
                f,
                "String is too short to be an RGB string and was not recognized as a keyword."
            ),
            LongString => write!(
                f,
                "String is too long to be an RGB string and was not recognized as a keyword."
            ),
        }
    }
}

impl FromStr for Color {
    type Err = ColorParseError;
    /// Parses the color string into a color setting.
    ///
    /// ## Notes
    /// Custom colors cannot be made with this method; use `Color::Custom(s)` instead.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use self::Color::*;
        use self::ColorParseError::*;
        match s {
            "red" => Ok(Red),
            "orange" => Ok(Orange),
            "yellow" => Ok(Yellow),
            "green" => Ok(Green),
            "blue" => Ok(Blue),
            "purple" => Ok(Purple),
            "pink" => Ok(Pink),
            "white" => Ok(White),
            hue if hue.starts_with("hue:") => {
                if let Some(spec) = hue.split(':').nth(1) {
                    if spec.trim().is_empty() {
                        Err(NoHue)
                    } else {
                        let hue = spec.parse();
                        hue.map(Hue).map_err(NonNumericHue)
                    }
                } else {
                    Err(NoHue)
                }
            }
            s if s.starts_with("saturation:") => {
                if let Some(spec) = s.split(':').nth(1) {
                    if spec.trim().is_empty() {
                        Err(NoSaturation)
                    } else {
                        let s = spec.parse();
                        s.map(Saturation).map_err(NonNumericSaturation)
                    }
                } else {
                    Err(NoSaturation)
                }
            }
            b if b.starts_with("brightness:") => {
                if let Some(spec) = b.split(':').nth(1) {
                    if spec.trim().is_empty() {
                        Err(NoBrightness)
                    } else {
                        let b = spec.parse();
                        b.map(Brightness).map_err(NonNumericBrightness)
                    }
                } else {
                    Err(NoBrightness)
                }
            }
            k if k.starts_with("kelvin:") => {
                if let Some(spec) = k.split(':').nth(1) {
                    if spec.trim().is_empty() {
                        Err(NoKelvin)
                    } else {
                        let k = spec.parse();
                        k.map(Kelvin).map_err(NonNumericKelvin)
                    }
                } else {
                    Err(NoKelvin)
                }
            }
            // Let's revisit this with combinators and Try later.
            r if r.starts_with("rgb:") => {
                let mut split = r.split(':');
                if let Some(parts) = split.nth(1) {
                    let mut parts = parts.split(',');
                    if let Some(r) = parts.next() {
                        if r.trim().is_empty() {
                            return Err(NoRed);
                        }
                        if let Some(g) = parts.next() {
                            if g.trim().is_empty() {
                                return Err(NoGreen);
                            }
                            if let Some(b) = parts.next() {
                                if b.trim().is_empty() {
                                    return Err(NoBlue);
                                }
                                match r.parse() {
                                    Ok(r) => match g.parse() {
                                        Ok(g) => match b.parse() {
                                            Ok(b) => Ok(Rgb([r, g, b])),
                                            Err(e) => Err(NonNumericBlue(e)),
                                        },
                                        Err(e) => Err(NonNumericGreen(e)),
                                    },
                                    Err(e) => Err(NonNumericRed(e)),
                                }
                            } else {
                                Err(NoBlue)
                            }
                        } else {
                            Err(NoGreen)
                        }
                    } else {
                        Err(NoRed)
                    }
                } else {
                    Err(NoRed)
                }
            }
            s => {
                if s.starts_with('#') {
                    match s.len() {
                        x if x < 7 => Err(ShortString),
                        7 => Ok(RgbStr(s.to_string())),
                        _ => Err(LongString),
                    }
                } else {
                    match s.len() {
                        x if x < 6 => Err(ShortString),
                        6 => Ok(RgbStr(s.to_string())),
                        _ => Err(LongString),
                    }
                }
            }
        }
    }
}

/// Represents a (local) color validation error.
#[derive(Debug, PartialEq)]
pub enum Error {
    /// The given hue was greater than the maximum hue of 360.
    ///
    /// ## Example
    /// ```
    /// use lifx::http::*;
    /// let res = Color::Hue(361).validate();
    /// assert_eq!(res, Err(ColorValidationError::Hue(361)));
    /// ```
    Hue(u16),
    /// The given saturation was greater than 1.0.
    ///
    /// ## Example
    /// ```
    /// use lifx::http::*;
    /// let res = Color::Saturation(1.1).validate();
    /// assert_eq!(res, Err(ColorValidationError::SaturationHigh(1.1)));
    /// ```
    SaturationHigh(f32),
    /// The given saturation was less than 0.0.
    ///
    /// ## Example
    /// ```
    /// use lifx::http::*;
    /// let res = Color::Saturation(-0.1).validate();
    /// assert_eq!(res, Err(ColorValidationError::SaturationLow(-0.1)));
    /// ```
    SaturationLow(f32),
    /// The given brightness was greater than 1.0.
    ///
    /// ## Example
    /// ```
    /// use lifx::http::*;
    /// let res = Color::Brightness(1.1).validate();
    /// assert_eq!(res, Err(ColorValidationError::BrightnessHigh(1.1)));
    /// ```
    BrightnessHigh(f32),
    /// The given brightness was less than 0.0.
    ///
    /// ## Example
    /// ```
    /// use lifx::http::*;
    /// let res = Color::Brightness(-0.1).validate();
    /// assert_eq!(res, Err(ColorValidationError::BrightnessLow(-0.1)));
    /// ```
    BrightnessLow(f32),
    /// The given temperature was greater than 9000 K.
    ///
    /// ## Example
    /// ```
    /// use lifx::http::*;
    /// let res = Color::Kelvin(9001).validate();
    /// assert_eq!(res, Err(ColorValidationError::KelvinHigh(9001)));
    /// ```
    KelvinHigh(u16),
    /// The given temperature was less than 1500 K.
    ///
    /// ## Example
    /// ```
    /// use lifx::http::*;
    /// let res = Color::Kelvin(1499).validate();
    /// assert_eq!(res, Err(ColorValidationError::KelvinLow(1499)));
    /// ```
    KelvinLow(u16),
    /// The given RGB string was too short.
    ///
    /// ## Examples
    /// ```
    /// use lifx::http::*;
    /// let res = Color::RgbStr("12345".to_string()).validate();
    /// assert_eq!(res, Err(ColorValidationError::RgbStrShort(false, "12345".to_string())));
    /// let res = Color::RgbStr("#12345".to_string()).validate();
    /// assert_eq!(res, Err(ColorValidationError::RgbStrShort(true, "#12345".to_string())));
    /// ```
    RgbStrShort(bool, String),
    /// The given RGB string was too long.
    ///
    /// ## Examples
    /// ```
    /// use lifx::http::*;
    /// let res = Color::RgbStr("1234567".to_string()).validate();
    /// assert_eq!(res, Err(ColorValidationError::RgbStrLong(false, "1234567".to_string())));
    /// let res = Color::RgbStr("#1234567".to_string()).validate();
    /// assert_eq!(res, Err(ColorValidationError::RgbStrLong(true, "#1234567".to_string())));
    /// ```
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

impl Color {
    /// Checks whether the color is valid.
    ///
    /// ## Notes
    /// Custom color strings are not validated.
    ///
    /// ## Examples
    /// ```
    /// use lifx::http::Color;
    /// // Too short
    /// let setting = Color::RgbStr("".to_string());
    /// assert!(setting.validate().is_err());
    /// // Too long for no leading #
    /// let setting = Color::RgbStr("1234567".to_string());
    /// assert!(setting.validate().is_err());
    /// // Too high (max 9000)
    /// let setting = Color::Kelvin(10_000);
    /// assert!(setting.validate().is_err());
    /// // Too high (max 1.0)
    /// let setting = Color::Brightness(1.2);
    /// assert!(setting.validate().is_err());
    /// // Too low (min 0.0)
    /// let setting = Color::Saturation(-0.1);
    /// assert!(setting.validate().is_err());
    /// let setting = Color::Kelvin(2_000);
    /// assert!(setting.validate().is_ok());
    /// ```
    pub fn validate(&self) -> Result<(), Error> {
        use self::Color::*;
        use self::Error::*;
        match self {
            Red | Orange | Yellow | Green | Blue | Purple | Pink | White | Rgb(_) | Custom(_) => {
                Ok(())
            }
            self::Color::Hue(hue) => {
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

/// A thin wrapper for `std::time::Duration` to aid with {de,}serialization.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Duration(StdDuration);

impl From<StdDuration> for Duration {
    fn from(duration: StdDuration) -> Self {
        Duration(duration)
    }
}

/// A wrapper around a power state to make sure it is serialized properly.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Power(bool);

impl From<bool> for Power {
    fn from(on: bool) -> Self {
        Power(on)
    }
}

/// Encodes a desired final state.
///
/// This struct should only be used directly when using
/// [`Selected::set_states`](struct.Selected.html#method.set_states), and even then, it is
/// encouraged to use the builder methods instead of directly constructing a set of changes.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct State {
    /// The desired power state, if appropriate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub power: Option<Power>,
    /// The desired color setting, if appropriate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<Color>,
    /// The desired brightness level (0–1), if appropriate. Will take priority over any brightness
    /// specified in a color setting.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brightness: Option<f32>,
    /// How long the transition should take.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<Duration>,
    /// If appropriate, the desired infrared light level (0–1).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infrared: Option<f32>,
}

impl<'de> Deserialize<'de> for Color {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<(Self), D::Error> {
        let s = String::deserialize(deserializer)?;
        s.parse::<Self>().map_err(DeError::custom)
    }
}

impl Serialize for Color {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&format!("{}", self))
    }
}

impl Serialize for Duration {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let time = self.0;
        let secs = time.as_secs() as f64;
        let millis = f64::from(time.subsec_millis()) / 1000.0;
        let t = secs + millis;
        serializer.serialize_f64(t)
    }
}

impl<'de> Deserialize<'de> for Duration {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<(Self), D::Error> {
        f64::deserialize(deserializer).map(|f| {
            let secs = f.floor() as u64;
            let millis = ((f % 1.0) * 1000.0) as u32;
            Duration(StdDuration::new(secs, millis * 1_000_000))
        })
    }
}
impl Serialize for Power {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let on = self.0;
        serializer.serialize_str(if on { "on" } else { "off" })
    }
}

impl<'de> Deserialize<'de> for Power {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<(Self), D::Error> {
        let s = String::deserialize(deserializer)?;
        if s == "on" {
            Ok(Power(true))
        } else {
            Ok(Power(false))
        }
    }
}

impl State {
    /// Constructs an empty state.
    ///
    /// Identical to [`State::builder`](#method.builder).
    pub fn new() -> Self {
        Self::default()
    }
    /// Creates a new builder.
    pub fn builder() -> Self {
        Self::default()
    }
    /// Builder function to set target power setting.
    ///
    /// ## Example
    /// ```
    /// use std::time::Duration;
    /// use lifx::http::State;
    /// let new: State = State::builder().power(true).transition(Duration::from_millis(800));
    /// ```
    pub fn power<P: Into<Power>>(mut self, on: P) -> Self {
        self.power = Some(on.into());
        self
    }
    /// Builder function to set target color setting.
    ///
    /// ## Example
    /// ```
    /// use std::time::Duration;
    /// use lifx::http::{Color::*, State};
    /// let new: State = State::builder().color(Red);
    /// ```
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }
    /// Builder function to set target brightness setting.
    ///
    /// ## Example
    /// ```
    /// use std::time::Duration;
    /// use lifx::http::State;
    /// let new: State = State::builder().brightness(0.7).transition(Duration::from_millis(800));
    /// ```
    pub fn brightness(mut self, brightness: f32) -> Self {
        self.brightness = Some(brightness);
        self
    }
    /// Builder function to set animation duration.
    ///
    /// ## Example
    /// ```
    /// use std::time::Duration;
    /// use lifx::http::{Color::*, State};
    /// let new: State = State::builder().color(Red).transition(Duration::from_millis(800));
    /// ```
    pub fn transition<D: Into<Duration>>(mut self, duration: D) -> Self {
        self.duration = Some(duration.into());
        self
    }
    /// Builder function to set target maximum infrared level.
    ///
    /// ## Example
    /// ```
    /// use lifx::http::State;
    /// let new: State = State::builder().infrared(0.8);
    /// ```
    pub fn infrared(mut self, infrared: f32) -> Self {
        self.infrared = Some(infrared);
        self
    }
}

/// Encodes a desired state change.
///
/// This struct is intended for use with
/// [`Selected::change_state`](struct.Selected.html#method.change_state), and it is encouraged to
/// use the builder methods instead of directly constructing a changeset.
#[derive(Clone, Default, Deserialize, Serialize)]
pub struct StateChange {
    /// The desired power state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub power: Option<Power>,
    /// How long the transition should take.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<Duration>,
    /// The desired change in infrared light level.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infrared: Option<f32>,
    /// The desired change in hue.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hue: Option<i16>,
    /// The desired change in saturation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub saturation: Option<f32>,
    /// The desired change in brightness.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brightness: Option<f32>,
    /// The desired change in color temperature.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kelvin: Option<i16>,
}

impl StateChange {
    /// Constructs an empty state change.
    ///
    /// Identical to [`StateChange::new`](#method.builder).
    pub fn new() -> Self {
        Self::builder()
    }
    /// Creates a new builder.
    pub fn builder() -> Self {
        Self::default()
    }
    /// Builder function to change target power state.
    ///
    /// ## Example
    /// ```
    /// use lifx::http::StateChange;
    /// let new: StateChange = StateChange::builder().power(true);
    /// ```
    pub fn power<P: Into<Power>>(mut self, on: P) -> Self {
        self.power = Some(on.into());
        self
    }
    /// Builder function to change transition duration.
    ///
    /// ## Example
    /// ```
    /// use lifx::http::StateChange;
    /// let new: StateChange = StateChange::builder().transition(::std::time::Duration::from_secs(1));
    /// ```
    pub fn transition<T: Into<Duration>>(mut self, duration: T) -> Self {
        self.duration = Some(duration.into());
        self
    }
    /// Builder function to set target change in hue.
    ///
    /// ## Example
    /// ```
    /// use lifx::http::StateChange;
    /// let new: StateChange = StateChange::builder().hue(-60);
    /// ```
    pub fn hue(mut self, hue: i16) -> Self {
        self.hue = Some(hue);
        self
    }
    /// Builder function to set target change in saturation.
    ///
    /// ## Example
    /// ```
    /// use lifx::http::StateChange;
    /// let new: StateChange = StateChange::builder().saturation(-0.1);
    /// ```
    pub fn saturation(mut self, saturation: f32) -> Self {
        self.saturation = Some(saturation);
        self
    }
    /// Builder function to set target change in brightness.
    ///
    /// ## Example
    /// ```
    /// use lifx::http::StateChange;
    /// let new: StateChange = StateChange::builder().brightness(0.4);
    /// ```
    pub fn brightness(mut self, brightness: f32) -> Self {
        self.brightness = Some(brightness);
        self
    }
    /// Builder function to set target change in color temperature.
    ///
    /// ## Example
    /// ```
    /// use lifx::http::StateChange;
    /// let new: StateChange = StateChange::builder().kelvin(-200);
    /// ```
    pub fn kelvin(mut self, temp: i16) -> Self {
        self.kelvin = Some(temp);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    mod state {
        use super::*;
        #[test]
        fn builder() {
            let state = State::new()
                .power(true)
                .transition(::std::time::Duration::from_secs(1))
                .color(Color::White)
                .infrared(0.7)
                .brightness(0.3);
            assert_eq!(state.power, Some(Power(true)));
            assert_eq!(state.duration.map(|d| d.0.as_secs()), Some(1));
            assert_eq!(state.brightness, Some(0.3));
            assert_eq!(state.infrared, Some(0.7));
            assert_eq!(
                state.color.map(|c| format!("{}", c)),
                Some("white".to_string())
            );
        }
        mod change {
            use super::*;
            #[test]
            fn builder() {
                let change = StateChange::new()
                    .power(true)
                    .transition(::std::time::Duration::from_secs(3))
                    .hue(120)
                    .saturation(-0.3)
                    .brightness(0.1)
                    .kelvin(500);
                assert_eq!(change.power, Some(Power(true)));
                assert_eq!(change.duration.map(|d| d.0.as_secs()), Some(3));
                assert_eq!(change.hue, Some(120));
                assert_eq!(change.saturation, Some(-0.3));
                assert_eq!(change.brightness, Some(0.1));
                assert_eq!(change.kelvin, Some(500));
            }
        }
    }
    mod color {
        use super::*;
        #[test]
        fn serialize() {
            let color = Color::Red;
            assert_eq!(&format!("{}", color), "red");
            let color = Color::Orange;
            assert_eq!(&format!("{}", color), "orange");
            let color = Color::Yellow;
            assert_eq!(&format!("{}", color), "yellow");
            let color = Color::Green;
            assert_eq!(&format!("{}", color), "green");
            let color = Color::Blue;
            assert_eq!(&format!("{}", color), "blue");
            let color = Color::Purple;
            assert_eq!(&format!("{}", color), "purple");
            let color = Color::Pink;
            assert_eq!(&format!("{}", color), "pink");
            let color = Color::White;
            assert_eq!(&format!("{}", color), "white");
            let color = Color::Custom("cyan".to_string());
            assert_eq!(&format!("{}", color), "cyan");
            let color = Color::Hue(240);
            assert_eq!(&format!("{}", color), "hue:240");
            let color = Color::Saturation(0.531);
            assert_eq!(&format!("{}", color), "saturation:0.531");
            let color = Color::Brightness(0.3);
            assert_eq!(&format!("{}", color), "brightness:0.3");
            let color = Color::Kelvin(3500);
            assert_eq!(&format!("{}", color), "kelvin:3500");
            let color = Color::Rgb([0, 17, 36]);
            assert_eq!(&format!("{}", color), "rgb:0,17,36");
            let color = Color::RgbStr("123456".to_string());
            assert_eq!(&format!("{}", color), "#123456");
            let color = Color::RgbStr("#000000".to_string());
            assert_eq!(&format!("{}", color), "#000000");
        }
        #[test]
        fn deserialize() {
            let color = "red".parse();
            assert_eq!(color, Ok(Color::Red));
            let color = "orange".parse();
            assert_eq!(color, Ok(Color::Orange));
            let color = "yellow".parse();
            assert_eq!(color, Ok(Color::Yellow));
            let color = "green".parse();
            assert_eq!(color, Ok(Color::Green));
            let color = "blue".parse();
            assert_eq!(color, Ok(Color::Blue));
            let color = "purple".parse();
            assert_eq!(color, Ok(Color::Purple));
            let color = "pink".parse();
            assert_eq!(color, Ok(Color::Pink));
            let color = "white".parse();
            assert_eq!(color, Ok(Color::White));
            let color = "cyan".parse::<Color>();
            assert!(color.is_err());
            let color = "hue:240".parse();
            assert_eq!(color, Ok(Color::Hue(240)));
            let color = "saturation:0.531".parse();
            assert_eq!(color, Ok(Color::Saturation(0.531)));
            let color = "brightness:0.3".parse();
            assert_eq!(color, Ok(Color::Brightness(0.3)));
            let color = "kelvin:3500".parse();
            assert_eq!(color, Ok(Color::Kelvin(3500)));
            let color = "rgb:0,17,36".parse();
            assert_eq!(color, Ok(Color::Rgb([0, 17, 36])));
            let color = "#123456".parse();
            assert_eq!(color, Ok(Color::RgbStr("#123456".to_string())));
            let color = "#000000".parse();
            assert_eq!(color, Ok(Color::RgbStr("#000000".to_string())));
        }
        #[test]
        fn validate() {
            let color = Color::Red;
            assert!(color.validate().is_ok());
            let color = Color::Orange;
            assert!(color.validate().is_ok());
            let color = Color::Yellow;
            assert!(color.validate().is_ok());
            let color = Color::Green;
            assert!(color.validate().is_ok());
            let color = Color::Blue;
            assert!(color.validate().is_ok());
            let color = Color::Purple;
            assert!(color.validate().is_ok());
            let color = Color::White;
            assert!(color.validate().is_ok());
            let color = Color::Hue(370);
            assert_eq!(color.validate(), Err(Error::Hue(370)));
            let color = Color::Hue(300);
            assert!(color.validate().is_ok());
            let color = Color::Hue(0);
            assert!(color.validate().is_ok());
            let color = Color::Hue(0);
            assert!(color.validate().is_ok());
            let color = Color::Saturation(-1.0);
            assert_eq!(color.validate(), Err(Error::SaturationLow(-1.0)));
            let color = Color::Saturation(0.0);
            assert!(color.validate().is_ok());
            let color = Color::Saturation(1.0);
            assert!(color.validate().is_ok());
            let color = Color::Saturation(2.0);
            assert_eq!(color.validate(), Err(Error::SaturationHigh(2.0)));
            let color = Color::Brightness(-0.3);
            assert_eq!(color.validate(), Err(Error::BrightnessLow(-0.3)));
            let color = Color::Brightness(0.0);
            assert!(color.validate().is_ok());
            let color = Color::Brightness(3.4);
            assert_eq!(color.validate(), Err(Error::BrightnessHigh(3.4)));
            let color = Color::Kelvin(1000);
            assert_eq!(color.validate(), Err(Error::KelvinLow(1000)));
            let color = Color::Kelvin(1500);
            assert!(color.validate().is_ok());
            let color = Color::Kelvin(9000);
            assert!(color.validate().is_ok());
            let color = Color::Kelvin(9001);
            assert_eq!(color.validate(), Err(Error::KelvinHigh(9001)));
            let color = Color::RgbStr("#12345".to_string());
            assert_eq!(
                color.validate(),
                Err(Error::RgbStrShort(true, "#12345".to_string()))
            );
            let color = Color::RgbStr("12345".to_string());
            assert_eq!(
                color.validate(),
                Err(Error::RgbStrShort(false, "12345".to_string()))
            );
            let color = Color::RgbStr("123456".to_string());
            assert!(color.validate().is_ok());
            let color = Color::RgbStr("#123456".to_string());
            assert!(color.validate().is_ok());
            let color = Color::RgbStr("1234567".to_string());
            assert_eq!(
                color.validate(),
                Err(Error::RgbStrLong(false, "1234567".to_string()))
            );
            let color = Color::RgbStr("#1234567".to_string());
            assert_eq!(
                color.validate(),
                Err(Error::RgbStrLong(true, "#1234567".to_string()))
            );
        }
    }
}
