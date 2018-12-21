use crate::http::{
    client::{Request, Selected},
    selector::Select,
    state::{ColorSetting, Duration},
};
use reqwest::Method;

/// Specifies a "breathe" effect, wherein the light color fades smoothly to transition.
#[derive(Serialize)]
pub struct Breathe<'a, T: Select> {
    #[serde(skip)]
    pub(crate) parent: &'a Selected<'a, T>,
    pub(crate) color: ColorSetting,
    pub(crate) selector: &'a T,
    #[serde(skip_serializing_if = "Option::is_none", rename = "from_color")]
    pub(crate) from: Option<ColorSetting>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) period: Option<Duration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) cycles: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) persist: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) power_on: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) peak: Option<f32>,
}

impl<'a, T: Select> Breathe<'a, T> {
    /// Sets the starting color.
    ///
    /// If left blank, the current color of the bulb is used.
    pub fn from(&mut self, color: ColorSetting) -> &'_ mut Self {
        self.from = Some(color);
        self
    }
    /// Sets the animation duration.
    pub fn period<D: Into<Duration>>(&mut self, period: D) -> &'_ mut Self {
        self.period = Some(period.into());
        self
    }
    /// Sets the number of cycles to execute.
    pub fn cycles(&mut self, count: u16) -> &'_ mut Self {
        self.cycles = Some(count);
        self
    }
    /// Sets whether to keep the bulb at the stopping color after completion.
    pub fn persist(&mut self, keep: bool) -> &'_ mut Self {
        self.persist = Some(keep);
        self
    }
    /// Sets whether to power on the light if currently off.
    pub fn power(&mut self, force: bool) -> &'_ mut Self {
        self.power_on = Some(force);
        self
    }
    /// Sets when the peak of the animation should be (0–1, proportion of period).
    pub fn peak(&mut self, frac: f32) -> &'_ mut Self {
        self.peak = Some(frac);
        self
    }
}

impl<'a, 'b: 'a, T: Select> From<&'b Breathe<'a, T>> for Request<'a, &'b Breathe<'a, T>> {
    fn from(effect: &'b Breathe<'a, T>) -> Self {
        Self {
            client: effect.parent.client,
            path: format!("/lights/{}/effects/breathe", effect.parent.selector),
            body: effect,
            method: Method::POST,
        }
    }
}

/// Specifies a "pulse" effect, wherein the light color abruptly changes.
#[derive(Serialize)]
pub struct Pulse<'a, T: Select> {
    #[serde(skip)]
    pub(crate) parent: &'a Selected<'a, T>,
    pub(crate) color: ColorSetting,
    pub(crate) selector: &'a T,
    #[serde(skip_serializing_if = "Option::is_none", rename = "from_color")]
    pub(crate) from: Option<ColorSetting>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) period: Option<Duration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) cycles: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) persist: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) power_on: Option<bool>,
}

impl<'a, T: Select> Pulse<'a, T> {
    /// Sets the starting color.
    ///
    /// If left blank, the current color of the bulb is used.
    pub fn from(&mut self, color: ColorSetting) -> &'_ mut Self {
        self.from = Some(color);
        self
    }
    /// Sets the animation duration.
    pub fn period<D: Into<Duration>>(&mut self, period: D) -> &'_ mut Self {
        self.period = Some(period.into());
        self
    }
    /// Sets the number of cycles to execute.
    pub fn cycles(&mut self, count: u16) -> &'_ mut Self {
        self.cycles = Some(count);
        self
    }
    /// Sets whether to keep the bulb at the stopping color after completion.
    pub fn persist(&mut self, keep: bool) -> &'_ mut Self {
        self.persist = Some(keep);
        self
    }
    /// Sets whether to power on the light if currently off.
    pub fn power(&mut self, force: bool) -> &'_ mut Self {
        self.power_on = Some(force);
        self
    }
}

impl<'a, 'b: 'a, T: Select> From<&'b Pulse<'a, T>> for Request<'a, &'b Pulse<'a, T>> {
    fn from(effect: &'b Pulse<'a, T>) -> Self {
        Self {
            client: effect.parent.client,
            path: format!("/lights/{}/effects/pulse", effect.parent.selector),
            body: effect,
            method: Method::POST,
        }
    }
}
