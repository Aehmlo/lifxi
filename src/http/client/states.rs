use crate::http::{
    client::{AsRequest, Client, Request, Selected},
    state::{Color, Duration, Power, State, StateChange},
    Select,
};
use reqwest::Method;

/// A scoped request to toggle specific lights which may be further customized.
pub struct Toggle<'a, T: Select> {
    parent: &'a Selected<'a, T>,
}

impl<'a, T: Select> Toggle<'a, T> {
    pub(crate) fn new(parent: &'a Selected<'a, T>) -> Self {
        Self { parent }
    }
    /// Sets the transition time for the toggle.
    pub fn transition<D: Into<Duration>>(&self, duration: D) -> Request<'_, Duration> {
        Request {
            client: self.parent.client,
            path: format!("/lights/{}/toggle", self.parent.selector),
            body: duration.into(),
            method: Method::POST,
        }
    }
}

impl<'a, T: Select> AsRequest<()> for Toggle<'a, T> {
    fn method() -> reqwest::Method {
        Method::POST
    }
    fn client(&self) -> &'_ Client {
        self.parent.client
    }
    fn path(&self) -> String {
        format!("/lights/{}/toggle", self.parent.selector)
    }
    fn body(&self) -> &'_ () {
        &()
    }
}

/// A scoped request to uniformly set the state for all selected bulbs.
pub struct SetState<'a, T: Select> {
    pub(crate) parent: &'a Selected<'a, T>,
    pub(crate) new: State,
}

impl<'a, T: Select> SetState<'a, T> {
    pub(crate) fn new(parent: &'a Selected<'a, T>) -> Self {
        Self {
            parent,
            new: State::default(),
        }
    }
    /// Sets the power state of all selected bulbs.
    pub fn power<P: Into<Power>>(&'a mut self, on: P) -> &'a mut SetState<'a, T> {
        self.new.power = Some(on.into());
        self
    }
    /// Sets the color of all selected bulbs.
    pub fn color(&'a mut self, color: Color) -> &'a mut SetState<'a, T> {
        self.new.color = Some(color);
        self
    }
    /// Sets the brightness of all selected bulbs (overriding color settings).
    pub fn brightness(&'a mut self, brightness: f32) -> &'a mut SetState<'a, T> {
        self.new.brightness = Some(brightness);
        self
    }
    /// Sets the transition time (duration) for the change.
    pub fn transition<D: Into<Duration>>(&'a mut self, duration: D) -> &'a mut SetState<'a, T> {
        self.new.duration = Some(duration.into());
        self
    }
    /// Sets the infrared level, if applicable.
    pub fn infrared(&'a mut self, ir: f32) -> &'a mut SetState<'a, T> {
        self.new.infrared = Some(ir);
        self
    }
}

impl<'a, T: Select> AsRequest<State> for SetState<'a, T> {
    fn method() -> reqwest::Method {
        Method::PUT
    }
    fn client(&self) -> &'_ Client {
        self.parent.client
    }
    fn path(&self) -> String {
        format!("/lights/{}/state", self.parent.selector)
    }
    fn body(&self) -> &'_ State {
        &self.new
    }
}

#[derive(Clone, Serialize)]
struct StateExt {
    pub(crate) selector: String,
    #[serde(flatten)]
    pub(crate) state: State,
}

#[derive(Clone, Default, Serialize)]
#[doc(hidden)]
pub struct SetStatesPayload {
    #[serde(rename = "states", skip_serializing_if = "Vec::is_empty")]
    new: Vec<StateExt>,
    #[serde(rename = "defaults", skip_serializing_if = "Option::is_none")]
    default: Option<State>,
}

/// A scoped request to uniformly set the state for all selected bulbs.
#[derive(Clone)]
pub struct SetStates<'a> {
    parent: &'a Client,
    inner: SetStatesPayload,
}

impl<'a> SetStates<'a> {
    pub(crate) fn new(parent: &'a Client) -> Self {
        Self {
            parent,
            inner: SetStatesPayload::default(),
        }
    }
    /// Adds the given state to the list.
    pub fn add<T: Select>(&mut self, selector: T, state: State) -> &'_ mut Self {
        self.inner.new.push(StateExt {
            selector: format!("{}", selector),
            state,
        });
        self
    }
    /// Sets the default properties to use if left unspecified.
    pub fn default(&mut self, state: State) -> &'_ mut Self {
        self.inner.default = Some(state);
        self
    }
}

impl<'a> AsRequest<SetStatesPayload> for SetStates<'a> {
    fn method() -> reqwest::Method {
        Method::PUT
    }
    fn client(&self) -> &'_ Client {
        self.parent
    }
    fn path(&self) -> String {
        "/lights/states".to_string()
    }
    fn body(&self) -> &'_ SetStatesPayload {
        &self.inner
    }
}

/// A scoped request to uniformly change the state for all selected bulbs.
pub struct ChangeState<'a, T: Select> {
    parent: &'a Selected<'a, T>,
    change: StateChange,
}

impl<'a, T: Select> ChangeState<'a, T> {
    pub(crate) fn new(parent: &'a Selected<'a, T>) -> Self {
        Self {
            parent,
            change: StateChange::default(),
        }
    }
    /// Sets target power state.
    pub fn power<P: Into<Power>>(&'a mut self, on: P) -> &'a mut Self {
        self.change.power = Some(on.into());
        self
    }
    /// Sets transition duration.
    pub fn transition<D: Into<Duration>>(&'a mut self, duration: D) -> &'a mut Self {
        self.change.duration = Some(duration.into());
        self
    }
    /// Sets change in hue.
    pub fn hue(&'a mut self, hue: i16) -> &'a mut Self {
        self.change.hue = Some(hue);
        self
    }
    /// Sets change in saturation.
    pub fn saturation(&'a mut self, saturation: f32) -> &'a mut Self {
        self.change.saturation = Some(saturation);
        self
    }
    /// Sets change in brightness.
    pub fn brightness(&'a mut self, brightness: f32) -> &'a mut Self {
        self.change.brightness = Some(brightness);
        self
    }
    /// Sets change in color temperature.
    pub fn kelvin(&'a mut self, temp: i16) -> &'a mut Self {
        self.change.kelvin = Some(temp);
        self
    }
    /// Sets change in infrared level.
    pub fn infrared(&'a mut self, ir: f32) -> &'a mut Self {
        self.change.infrared = Some(ir);
        self
    }
}

impl<'a, T: Select> AsRequest<StateChange> for ChangeState<'a, T> {
    fn method() -> reqwest::Method {
        Method::POST
    }
    fn client(&self) -> &'_ Client {
        self.parent.client
    }
    fn path(&self) -> String {
        format!("/lights/{}/state/delta", self.parent.selector)
    }
    fn body(&self) -> &'_ StateChange {
        &self.change
    }
}

/// Specifies a list of effects to cycle through. Each request causes the cycle to advance.
pub struct Cycle<'a, T: Select> {
    parent: &'a Selected<'a, T>,
    inner: CyclePayload<'a, T>,
}

impl<'a, T: Select> Cycle<'a, T> {
    pub(crate) fn new(parent: &'a Selected<'a, T>) -> Self {
        Self {
            parent,
            inner: CyclePayload::new(&parent.selector),
        }
    }
    /// Adds a state to the cycle.
    pub fn add(&mut self, next: State) -> &'_ mut Self {
        self.inner.states.push(next);
        self
    }
    /// Sets the default values to use when not specified.
    pub fn default(&mut self, state: State) -> &'_ mut Self {
        self.inner.default = Some(state);
        self
    }
    /// Reverses the direction of the cycle.
    pub fn rev(&mut self) -> &'_ mut Self {
        self.inner.direction = if self.inner.direction == "forward" {
            "backward"
        } else {
            "forward"
        };
        self
    }
}

#[derive(Clone, Serialize)]
#[doc(hidden)]
pub struct CyclePayload<'a, T: Select> {
    pub(crate) selector: &'a T,
    pub(crate) direction: &'static str,
    pub(crate) states: Vec<State>,
    #[serde(rename = "defaults", skip_serializing_if = "Option::is_none")]
    pub(crate) default: Option<State>,
}

impl<'a, T: Select> CyclePayload<'a, T> {
    fn new(selector: &'a T) -> Self {
        Self {
            selector,
            direction: "forward",
            states: Vec::new(),
            default: None,
        }
    }
}

impl<'a, T: Select> AsRequest<CyclePayload<'a, T>> for Cycle<'a, T> {
    fn method() -> reqwest::Method {
        Method::POST
    }
    fn client(&self) -> &'_ Client {
        self.parent.client
    }
    fn path(&self) -> String {
        format!("/lights/{}/cycle", self.parent.selector)
    }
    fn body(&self) -> &'_ CyclePayload<'a, T> {
        &self.inner
    }
}
