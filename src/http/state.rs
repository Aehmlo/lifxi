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
#[derive(Clone)]
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
    /// Uses a custom specifier string.
    ///
    /// This option exists for undocumented features. For instance, "cyan" is a valid color choice,
    /// but it is undocumented and therefore (theoretically) unstable, so it is not officially/
    /// supported by this crate.
    Custom(String),
}

impl fmt::Display for ColorSetting {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ColorSetting::Red => write!(f, "red"),
            ColorSetting::Orange => write!(f, "orange"),
            ColorSetting::Yellow => write!(f, "yellow"),
            ColorSetting::Green => write!(f, "green"),
            ColorSetting::Blue => write!(f, "blue"),
            ColorSetting::Purple => write!(f, "purple"),
            ColorSetting::Pink => write!(f, "pink"),
            ColorSetting::White => write!(f, "white"),
            ColorSetting::Hue(hue) => write!(f, "hue:{}", hue),
            ColorSetting::Saturation(sat) => write!(f, "saturation:{:0.1}", sat),
            ColorSetting::Brightness(b) => write!(f, "brightness:{:0.1}", b),
            ColorSetting::Kelvin(t) => write!(f, "kelvin:{}", t),
            ColorSetting::Rgb(rgb) => write!(f, "rgb:{},{},{}", rgb[0], rgb[1], rgb[2]),
            ColorSetting::RgbStr(s) => {
                if s.starts_with('#') {
                    write!(f, "{}", s)
                } else {
                    write!(f, "#{}", s)
                }
            }
            ColorSetting::Custom(s) => write!(f, "{}", s),
        }
    }
}

/// Represents an error encountered while deserializing a color.
pub enum ColorParseError {
    /// No hue was given.
    NoHue,
    /// The hue could not be parsed as an integer.
    NonNumericHue(ParseIntError),
    /// No saturation was given.
    NoSaturation,
    /// The saturation could not be parsed as a float.
    NonNumericSaturation(ParseFloatError),
    /// No brightness was given.
    NoBrightness,
    /// The brightness could not be parsed as a float.
    NonNumericBrightness(ParseFloatError),
    /// No color temperature was given.
    NoKelvin,
    /// The color temperature could not be parsed as an integer.
    NonNumericKelvin(ParseIntError),
    /// No red component was given.
    NoRed,
    /// The red component could not be parsed as an integer.
    NonNumericRed(ParseIntError),
    /// No green component was given.
    NoGreen,
    /// The green component could not be parsed as an integer.
    NonNumericGreen(ParseIntError),
    /// No blue component was given.
    NoBlue,
    /// The blue component could not be parsed as an integer.
    NonNumericBlue(ParseIntError),
    /// The string is too short to be an RGB string and was not recognized as a keyword.
    ShortString,
    /// The string is too long to be an RGB string and was not recognized as a keyword.
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

impl FromStr for ColorSetting {
    type Err = ColorParseError;
    /// Parses the color string into a color setting.
    ///
    /// ### Notes
    /// Custom colors cannot be made with this method; use `ColorSetting::Custom(s)` instead.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use self::ColorParseError::*;
        use self::ColorSetting::*;
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
                    let hue = spec.parse();
                    hue.map(Hue).map_err(NonNumericHue)
                } else {
                    Err(NoHue)
                }
            }
            s if s.starts_with("saturation:") => {
                if let Some(spec) = s.split(':').nth(1) {
                    let s = spec.parse();
                    s.map(Saturation).map_err(NonNumericSaturation)
                } else {
                    Err(NoSaturation)
                }
            }
            b if b.starts_with("brightness:") => {
                if let Some(spec) = b.split(':').nth(1) {
                    let b = spec.parse();
                    b.map(Brightness).map_err(NonNumericBrightness)
                } else {
                    Err(NoBrightness)
                }
            }
            k if k.starts_with("kelvin:") => {
                if let Some(spec) = k.split(':').nth(1) {
                    let k = spec.parse();
                    k.map(Kelvin).map_err(NonNumericKelvin)
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
                        if let Some(g) = parts.next() {
                            if let Some(b) = parts.next() {
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
    /// ### Notes
    /// Custom color strings are not validated.
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
            Custom(_) => Ok(()),
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
    pub color: Option<ColorSetting>,
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

impl<'de> Deserialize<'de> for ColorSetting {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<(ColorSetting), D::Error> {
        let s = String::deserialize(deserializer)?;
        s.parse::<ColorSetting>().map_err(DeError::custom)
    }
}

impl Serialize for ColorSetting {
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
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<(Duration), D::Error> {
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
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<(Power), D::Error> {
        let s = String::deserialize(deserializer)?;
        if s == "on" {
            Ok(Power(true))
        } else {
            Ok(Power(false))
        }
    }
}

impl State {
    /// Creates a new builder.
    ///
    /// Finalize the new state settings with [`State::finalize`](#method.finalize).
    pub fn builder() -> Self {
        Self::default()
    }
    /// Builder function to set target power setting.
    ///
    /// ### Examples
    /// ```
    /// use std::time::Duration;
    /// use lifx::http::State;
    /// let new: State = State::builder().power(true).transition(Duration::from_millis(800)).finalize();
    /// ```
    pub fn power<P: Into<Power>>(&mut self, on: P) -> &'_ mut Self {
        self.power = Some(on.into());
        self
    }
    /// Builder function to set target color setting.
    ///
    /// ### Examples
    /// ```
    /// use std::time::Duration;
    /// use lifx::http::{ColorSetting::*, State};
    /// let new: State = State::builder().color(Red).finalize();
    /// ```
    pub fn color(&mut self, color: ColorSetting) -> &'_ mut Self {
        self.color = Some(color);
        self
    }
    /// Builder function to set target brightness setting.
    ///
    /// ### Examples
    /// ```
    /// use std::time::Duration;
    /// use lifx::http::State;
    /// let new: State = State::builder().brightness(0.7).transition(Duration::from_millis(800)).finalize();
    /// ```
    pub fn brightness(&mut self, brightness: f32) -> &'_ mut Self {
        self.brightness = Some(brightness);
        self
    }
    /// Builder function to set animation duration.
    ///
    /// ### Examples
    /// ```
    /// use std::time::Duration;
    /// use lifx::http::{ColorSetting::*, State};
    /// let new: State = State::builder().color(Red).transition(Duration::from_millis(800)).finalize();
    /// ```
    pub fn transition<D: Into<Duration>>(&mut self, duration: D) -> &'_ mut Self {
        self.duration = Some(duration.into());
        self
    }
    /// Builder function to set target maximum infrared level.
    ///
    /// ### Examples
    /// ```
    /// use lifx::http::State;
    /// let new: State = State::builder().infrared(0.8).finalize();
    /// ```
    pub fn infrared(&mut self, infrared: f32) -> &'_ mut Self {
        self.infrared = Some(infrared);
        self
    }
    /// Finalizes the builder, returning the final state configuration.
    ///
    /// ### Example
    /// ```
    /// use lifx::http::{ColorSetting::*, State};
    /// let new: State = State::builder().power(true).brightness(0.5).finalize();
    /// let new: State = State::builder().color(Red).finalize();
    /// ```
    pub fn finalize(&mut self) -> State {
        self.clone()
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
    /// Creates a new builder.
    ///
    /// Finalize the state change settings with [`State::finalize`](#method.finalize).
    pub fn builder() -> Self {
        Self::default()
    }
    /// Builder function to change target power state.
    pub fn power<P: Into<Power>>(&mut self, on: P) -> &'_ mut Self {
        self.power = Some(on.into());
        self
    }
    /// Builder function to change transition duration.
    pub fn transition<T: Into<Duration>>(&mut self, duration: T) -> &'_ mut Self {
        self.duration = Some(duration.into());
        self
    }
    /// Builder function to set target change in hue.
    pub fn hue(&mut self, hue: i16) -> &'_ mut Self {
        self.hue = Some(hue);
        self
    }
    /// Builder function to set target change in saturation.
    pub fn saturation(&mut self, saturation: f32) -> &'_ mut Self {
        self.saturation = Some(saturation);
        self
    }
    /// Builder function to set target change in brightness.
    pub fn brightness(&mut self, brightness: f32) -> &'_ mut Self {
        self.brightness = Some(brightness);
        self
    }
    /// Builder function to set target change in color temperature.
    pub fn kelvin(&mut self, temp: i16) -> &'_ mut Self {
        self.kelvin = Some(temp);
        self
    }
    /// Finalizes the builder, returning the final state change configuration.
    ///
    /// ### Example
    /// ```
    /// use lifx::http::StateChange;
    /// let change: StateChange = StateChange::builder().power(true).brightness(0.5).finalize();
    /// let change: StateChange = StateChange::builder().hue(-120).finalize();
    /// ```
    pub fn finalize(&mut self) -> Self {
        self.clone()
    }
}
