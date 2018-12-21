use crate::http::{
    client::{AsRequest, Client, Selected},
    selector::Select,
    state::{Color, Duration},
};
use reqwest::Method;

#[derive(Clone, Serialize)]
#[doc(hidden)]
pub struct BreathePayload<'a, T: Select> {
    color: Color,
    selector: &'a T,
    #[serde(skip_serializing_if = "Option::is_none", rename = "from_color")]
    from: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    period: Option<Duration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cycles: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    persist: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    power_on: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    peak: Option<f32>,
}

impl<'a, T: Select> BreathePayload<'a, T> {
    fn new(selector: &'a T, color: Color) -> Self {
        Self {
            selector,
            color,
            from: None,
            period: None,
            cycles: None,
            persist: None,
            power_on: None,
            peak: None,
        }
    }
}

/// Specifies a "breathe" effect, wherein the light color fades smoothly to transition.
pub struct Breathe<'a, T: Select> {
    pub(crate) parent: &'a Selected<'a, T>,
    inner: BreathePayload<'a, T>,
}

impl<'a, T: Select> Breathe<'a, T> {
    pub(crate) fn new(parent: &'a Selected<'a, T>, color: Color) -> Self {
        Self {
            parent,
            inner: BreathePayload::new(&parent.selector, color),
        }
    }
    /// Sets the starting color.
    ///
    /// If left blank, the current color of the bulb is used.
    pub fn from(&mut self, color: Color) -> &'_ mut Self {
        self.inner.from = Some(color);
        self
    }
    /// Sets the animation duration.
    pub fn period<D: Into<Duration>>(&mut self, period: D) -> &'_ mut Self {
        self.inner.period = Some(period.into());
        self
    }
    /// Sets the number of cycles to execute.
    pub fn cycles(&mut self, count: u16) -> &'_ mut Self {
        self.inner.cycles = Some(count);
        self
    }
    /// Sets whether to keep the bulb at the stopping color after completion.
    pub fn persist(&mut self, keep: bool) -> &'_ mut Self {
        self.inner.persist = Some(keep);
        self
    }
    /// Sets whether to power on the light if currently off.
    pub fn power(&mut self, force: bool) -> &'_ mut Self {
        self.inner.power_on = Some(force);
        self
    }
    /// Sets when the peak of the animation should be (0–1, proportion of period).
    pub fn peak(&mut self, frac: f32) -> &'_ mut Self {
        self.inner.peak = Some(frac);
        self
    }
}

impl<'a, T: Select> AsRequest<BreathePayload<'a, T>> for Breathe<'a, T> {
    fn method() -> reqwest::Method {
        Method::POST
    }
    fn client(&self) -> &'_ Client {
        self.parent.client
    }
    fn path(&self) -> String {
        format!("/lights/{}/effects/breathe", self.parent.selector)
    }
    fn body(&self) -> &'_ BreathePayload<'a, T> {
        &self.inner
    }
}

#[derive(Clone, Serialize)]
#[doc(hidden)]
pub struct PulsePayload<'a, T: Select> {
    color: Color,
    selector: &'a T,
    #[serde(skip_serializing_if = "Option::is_none", rename = "from_color")]
    from: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    period: Option<Duration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cycles: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    persist: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    power_on: Option<bool>,
}

impl<'a, T: Select> PulsePayload<'a, T> {
    fn new(selector: &'a T, color: Color) -> Self {
        Self {
            selector,
            color,
            from: None,
            period: None,
            cycles: None,
            persist: None,
            power_on: None,
        }
    }
}

/// Specifies a "pulse" effect, wherein the light color abruptly changes.
pub struct Pulse<'a, T: Select> {
    parent: &'a Selected<'a, T>,
    inner: PulsePayload<'a, T>,
}

impl<'a, T: Select> Pulse<'a, T> {
    pub(crate) fn new(parent: &'a Selected<'a, T>, color: Color) -> Self {
        Self {
            parent,
            inner: PulsePayload::new(&parent.selector, color),
        }
    }
    /// Sets the starting color.
    ///
    /// If left blank, the current color of the bulb is used.
    pub fn from(&mut self, color: Color) -> &'_ mut Self {
        self.inner.from = Some(color);
        self
    }
    /// Sets the animation duration.
    pub fn period<D: Into<Duration>>(&mut self, period: D) -> &'_ mut Self {
        self.inner.period = Some(period.into());
        self
    }
    /// Sets the number of cycles to execute.
    pub fn cycles(&mut self, count: u16) -> &'_ mut Self {
        self.inner.cycles = Some(count);
        self
    }
    /// Sets whether to keep the bulb at the stopping color after completion.
    pub fn persist(&mut self, keep: bool) -> &'_ mut Self {
        self.inner.persist = Some(keep);
        self
    }
    /// Sets whether to power on the light if currently off.
    pub fn power(&mut self, force: bool) -> &'_ mut Self {
        self.inner.power_on = Some(force);
        self
    }
}

impl<'a, T: Select> AsRequest<PulsePayload<'a, T>> for Pulse<'a, T> {
    fn method() -> reqwest::Method {
        Method::POST
    }
    fn client(&self) -> &'_ Client {
        self.parent.client
    }
    fn path(&self) -> String {
        format!("/lights/{}/effects/pulse", self.parent.selector)
    }
    fn body(&self) -> &'_ PulsePayload<'a, T> {
        &self.inner
    }
}
