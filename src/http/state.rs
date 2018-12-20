use std::fmt;
use std::time::Duration;

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
                if s.starts_with("#") {
                    write!(f, "{}", s)
                } else {
                    write!(f, "#{}", s)
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

/// Encodes a desired final state.
///
/// This struct should only be used directly when using
/// [`Selected::set_states`](struct.Selected.html#method.set_states), and even then, it is
/// encouraged to use the builder methods instead of directly constructing a set of changes.
#[derive(Clone, Default)]
pub struct State {
    /// The desired power state, if appropriate.
    pub power: Option<bool>,
    /// The desired color setting, if appropriate.
    pub color: Option<ColorSetting>,
    /// The desired brightness level (0–1), if appropriate. Will take priority over any brightness
    /// specified in a color setting.
    pub brightness: Option<f32>,
    /// How long the transition should take.
    pub duration: Option<Duration>,
    /// If appropriate, the desired infrared light level (0–1).
    pub infrared: Option<f32>,
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
    pub fn power<'a>(&'a mut self, on: bool) -> &'a mut Self {
        self.power = Some(on);
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
    pub fn color<'a>(&'a mut self, color: ColorSetting) -> &'a mut Self {
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
    pub fn brightness<'a>(&'a mut self, brightness: f32) -> &'a mut Self {
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
    pub fn transition<'a>(&'a mut self, duration: Duration) -> &'a mut Self {
        self.duration = Some(duration);
        self
    }
    /// Builder function to set target maximum infrared level.
    ///
    /// ### Examples
    /// ```
    /// use lifx::http::State;
    /// let new: State = State::builder().infrared(0.8).finalize();
    /// ```
    pub fn infrared<'a>(&'a mut self, infrared: f32) -> &'a mut Self {
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
    pub fn finalize<'a>(&'a mut self) -> State {
        self.clone()
    }
}

/// Encodes a desired state change.
///
/// This struct is intended for use with
/// [`Selected::change_state`](struct.Selected.html#method.change_state), and it is encouraged to
/// use the builder methods instead of directly constructing a changeset.
#[derive(Clone, Default)]
pub struct StateChange {
    /// The desired power state.
    pub power: Option<bool>,
    /// How long the transition should take.
    pub duration: Option<Duration>,
    /// The desired change in infrared light level.
    pub infrared: Option<f32>,
    /// The desired change in hue.
    pub hue: Option<i16>,
    /// The desired change in saturation.
    pub saturation: Option<f32>,
    /// The desired change in brightness.
    pub brightness: Option<f32>,
    /// The desired change in color temperature.
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
    pub fn power<'a>(&'a mut self, on: bool) -> &'a mut Self {
        self.power = Some(on);
        self
    }
    /// Builder function to change transition duration.
    pub fn transition<'a>(&'a mut self, duration: Duration) -> &'a mut Self {
        self.duration = Some(duration);
        self
    }
    /// Builder function to set target change in hue.
    pub fn hue<'a>(&'a mut self, hue: i16) -> &'a mut Self {
        self.hue = Some(hue);
        self
    }
    /// Builder function to set target change in saturation.
    pub fn saturation<'a>(&'a mut self, saturation: f32) -> &'a mut Self {
        self.saturation = Some(saturation);
        self
    }
    /// Builder function to set target change in brightness.
    pub fn brightness<'a>(&'a mut self, brightness: f32) -> &'a mut Self {
        self.brightness = Some(brightness);
        self
    }
    /// Builder function to set target change in color temperature.
    pub fn kelvin<'a>(&'a mut self, temp: i16) -> &'a mut Self {
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
    pub fn finalize<'a>(&'a mut self) -> Self {
        self.clone()
    }
}
