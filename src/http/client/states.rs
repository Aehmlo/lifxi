use crate::http::{
    client::{unity, AsRequest, Attempts, Client, Request, Selected},
    state::{Color, Duration, Power, State, StateChange},
    Select,
};
use reqwest::Method;
use std::num::NonZeroU8;

/// A scoped request to toggle specific lights which may be further customized.
///
/// ## Examples
/// ### Transition
/// ```
/// use lifxi::http::prelude::*;
/// # fn run() {
/// let client = Client::new("foo");
/// let result = client
///     .select(Selector::All)
///     .toggle()
///     .transition(::std::time::Duration::new(2, 0))
///     .send();
/// # }
/// ```
/// ### Immediate
/// ```
/// use lifxi::http::prelude::*;
/// # fn run() {
/// let client = Client::new("foo");
/// let result = client
///     .select(Selector::All)
///     .toggle()
///     .send();
/// # }
pub struct Toggle<'a, T: Select> {
    parent: &'a Selected<'a, T>,
    attempts: Option<NonZeroU8>,
}

impl<'a, T: Select> Toggle<'a, T> {
    pub(crate) fn new(parent: &'a Selected<'a, T>) -> Self {
        Self {
            parent,
            attempts: None,
        }
    }
    /// Sets the transition time for the toggle.
    ///
    /// ## Example
    /// ```
    /// use lifxi::http::prelude::*;
    /// # fn run() {
    /// let client = Client::new("foo");
    /// let result = client
    ///     .select(Selector::All)
    ///     .toggle()
    ///     .transition(::std::time::Duration::new(2, 0))
    ///     .send();
    /// # }
    pub fn transition<D: Into<Duration>>(&self, duration: D) -> Request<'_, Duration> {
        Request {
            client: self.parent.client,
            path: format!("/lights/{}/toggle", self.parent.selector),
            body: duration.into(),
            method: Method::POST,
            attempts: self.attempts.unwrap_or_else(unity),
        }
    }
}

impl<'a, T: Select> Attempts for Toggle<'a, T> {
    fn set_attempts(&mut self, attempts: NonZeroU8) {
        self.attempts = Some(attempts);
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
    fn attempts(&self) -> NonZeroU8 {
        self.attempts.unwrap_or_else(unity)
    }
}

/// A payload for setting a state.
#[doc(hidden)]
#[derive(Default, Serialize)]
pub struct SetStatePayload {
    #[serde(flatten)]
    state: State,
    #[serde(skip_serializing_if = "Option::is_none")]
    fast: Option<bool>,
}

/// A scoped request to uniformly set the state for all selected bulbs.
///
/// ## Example
/// ```
/// use lifxi::http::prelude::*;
/// # fn run() {
/// let client = Client::new("foo");
/// let office = Selector::Label("Office".to_string());
/// let basement = Selector::Label("Basement".to_string());
/// let result = client
///     .select(office.combine(basement))
///     .set_state()
///     .power(true)
///     .color(Color::Red)
///     .brightness(0.4)
///     .transition(::std::time::Duration::new(7, 0))
///     .infrared(0.3)
///     .send();
/// # }
/// ```
pub struct SetState<'a, T: Select> {
    parent: &'a Selected<'a, T>,
    attempts: Option<NonZeroU8>,
    payload: SetStatePayload,
}

impl<'a, T: Select> SetState<'a, T> {
    pub(crate) fn new(parent: &'a Selected<'a, T>) -> Self {
        Self {
            parent,
            payload: SetStatePayload::default(),
            attempts: None,
        }
    }
    /// Sets the power state of all selected bulbs.
    ///
    /// ## Example
    /// ```
    /// use lifxi::http::prelude::*;
    /// # fn run() {
    /// let client = Client::new("foo");
    /// let result = client
    ///     .select(Selector::All)
    ///     .set_state()
    ///     .power(true)
    ///     .send();
    /// # }
    /// ```
    pub fn power<P: Into<Power>>(&mut self, on: P) -> &'_ mut SetState<'a, T> {
        self.payload.state.power = Some(on.into());
        self
    }
    /// Sets the color of all selected bulbs.
    ///
    /// ## Example
    /// ```
    /// use lifxi::http::prelude::*;
    /// # fn run() {
    /// let client = Client::new("foo");
    /// let result = client
    ///     .select(Selector::All)
    ///     .set_state()
    ///     .color(Color::Red)
    ///     .send();
    /// # }
    /// ```
    pub fn color(&mut self, color: Color) -> &'_ mut SetState<'a, T> {
        self.payload.state.color = Some(color);
        self
    }
    /// Sets the brightness of all selected bulbs (overriding color settings).
    ///
    /// ## Example
    /// ```
    /// use lifxi::http::prelude::*;
    /// # fn run() {
    /// let client = Client::new("foo");
    /// let result = client
    ///     .select(Selector::All)
    ///     .set_state()
    ///     .brightness(0.4)
    ///     .send();
    /// # }
    /// ```
    pub fn brightness(&mut self, brightness: f32) -> &'_ mut Self {
        self.payload.state.brightness = Some(brightness);
        self
    }
    /// Sets the transition time (duration) for the change.
    ///
    /// ## Example
    /// ```
    /// use lifxi::http::prelude::*;
    /// # fn run() {
    /// let client = Client::new("foo");
    /// let result = client
    ///     .select(Selector::All)
    ///     .set_state()
    ///     .transition(::std::time::Duration::new(7, 0))
    ///     .send();
    /// # }
    /// ```
    pub fn transition<D: Into<Duration>>(&mut self, duration: D) -> &'_ mut Self {
        self.payload.state.duration = Some(duration.into());
        self
    }
    /// Sets the infrared level, if applicable.
    ///
    /// ## Example
    /// ```
    /// use lifxi::http::prelude::*;
    /// # fn run() {
    /// let client = Client::new("foo");
    /// let result = client
    ///     .select(Selector::All)
    ///     .set_state()
    ///     .infrared(0.3)
    ///     .send();
    /// # }
    /// ```
    pub fn infrared(&mut self, ir: f32) -> &'_ mut Self {
        self.payload.state.infrared = Some(ir);
        self
    }
    /// Sets whether to perform the action quickly (skipping checks and verification).
    ///
    /// ## Example
    /// ```
    /// use lifxi::http::prelude::*;
    /// # fn run() {
    /// let client = Client::new("foo");
    /// let result = client
    ///     .select(Selector::All)
    ///     .set_state()
    ///     .infrared(0.3)
    ///     .fast(true)
    ///     .send();
    /// # }
    /// ```
    pub fn fast(&mut self, quickly: bool) -> &'_ mut Self {
        self.payload.fast = Some(quickly);
        self
    }
}

impl<'a, T: Select> Attempts for SetState<'a, T> {
    fn set_attempts(&mut self, attempts: NonZeroU8) {
        self.attempts = Some(attempts);
    }
}

impl<'a, T: Select> AsRequest<SetStatePayload> for SetState<'a, T> {
    fn method() -> reqwest::Method {
        Method::PUT
    }
    fn client(&self) -> &'_ Client {
        self.parent.client
    }
    fn path(&self) -> String {
        format!("/lights/{}/state", self.parent.selector)
    }
    fn body(&self) -> &'_ SetStatePayload {
        &self.payload
    }
    fn attempts(&self) -> NonZeroU8 {
        self.attempts.unwrap_or_else(unity)
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
/// The message constructed by the `SetStates` request builder.
pub struct SetStatesPayload {
    #[serde(rename = "states", skip_serializing_if = "Vec::is_empty")]
    new: Vec<StateExt>,
    #[serde(rename = "defaults", skip_serializing_if = "Option::is_none")]
    default: Option<State>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fast: Option<bool>,
}

/// A scoped request to uniformly set the state for all selected bulbs.
///
/// ##Example
/// ```
/// use lifxi::http::prelude::*;
/// # fn run() {
/// let client = Client::new("foo");
/// let red = State::builder().color(Color::Red);
/// let purple = State::builder().color(Color::Purple);
/// let result = client
///     .set_states()
///     .add(Selector::Label("Desk".to_string()), red)
///     .add(Selector::Label("Ceiling".to_string()), purple)
///     .default(State::builder().power(true).brightness(0.8))
///     .send();
/// # }
/// ```
#[derive(Clone)]
pub struct SetStates<'a> {
    parent: &'a Client,
    inner: SetStatesPayload,
    attempts: Option<NonZeroU8>,
}

impl<'a> SetStates<'a> {
    pub(crate) fn new(parent: &'a Client) -> Self {
        Self {
            parent,
            inner: SetStatesPayload::default(),
            attempts: None,
        }
    }
    /// Adds the given state to the list.
    #[allow(clippy::needless_pass_by_value)]
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
    /// Sets whether to perform the action quickly (skipping checks and verification).
    pub fn fast(&mut self, fast: bool) -> &'_ mut Self {
        self.inner.fast = Some(fast);
        self
    }
}

impl<'a> Attempts for SetStates<'a> {
    fn set_attempts(&mut self, attempts: NonZeroU8) {
        self.attempts = Some(attempts);
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
    fn attempts(&self) -> NonZeroU8 {
        self.attempts.unwrap_or_else(unity)
    }
}

/// A scoped request to uniformly change the state for all selected bulbs.
///
/// ## Example
/// ```
/// use lifxi::http::prelude::*;
/// # fn run() {
/// let client = Client::new("foo");
/// let result = client
///     .select(Selector::All)
///     .change_state()
///     .power(true)
///     .hue(-10)
///     .saturation(0.1)
///     .brightness(0.4)
///     .kelvin(100)
///     .transition(::std::time::Duration::new(7, 0))
///     .infrared(-0.1)
///     .send();
/// # }
/// ```
pub struct ChangeState<'a, T: Select> {
    parent: &'a Selected<'a, T>,
    change: StateChange,
    attempts: Option<NonZeroU8>,
}

impl<'a, T: Select> ChangeState<'a, T> {
    pub(crate) fn new(parent: &'a Selected<'a, T>) -> Self {
        Self {
            parent,
            change: StateChange::default(),
            attempts: None,
        }
    }
    /// Sets target power state.
    ///
    /// ## Example
    /// ```
    /// use lifxi::http::prelude::*;
    /// # fn run() {
    /// let client = Client::new("foo");
    /// let result = client
    ///     .select(Selector::All)
    ///     .change_state()
    ///     .power(true)
    ///     .brightness(-0.3)
    ///     .send();
    /// # }
    /// ```
    pub fn power<P: Into<Power>>(&mut self, on: P) -> &'_ mut Self {
        self.change.power = Some(on.into());
        self
    }
    /// Sets transition duration.
    ///
    /// ## Example
    /// ```
    /// use lifxi::http::prelude::*;
    /// # fn run() {
    /// let client = Client::new("foo");
    /// let result = client
    ///     .select(Selector::All)
    ///     .change_state()
    ///     .power(true)
    ///     .transition(::std::time::Duration::new(7, 0))
    ///     .send();
    /// # }
    /// ```
    pub fn transition<D: Into<Duration>>(&mut self, duration: D) -> &'_ mut Self {
        self.change.duration = Some(duration.into());
        self
    }
    /// Sets change in hue.
    ///
    /// ## Example
    /// ```
    /// use lifxi::http::prelude::*;
    /// # fn run() {
    /// let client = Client::new("foo");
    /// let result = client
    ///     .select(Selector::All)
    ///     .change_state()
    ///     .power(true)
    ///     .hue(-10)
    ///     .send();
    /// # }
    /// ```
    pub fn hue(&mut self, hue: i16) -> &'_ mut Self {
        self.change.hue = Some(hue);
        self
    }
    /// Sets change in saturation.
    ///
    /// ## Example
    /// ```
    /// use lifxi::http::prelude::*;
    /// # fn run() {
    /// let client = Client::new("foo");
    /// let result = client
    ///     .select(Selector::All)
    ///     .change_state()
    ///     .power(true)
    ///     .saturation(0.1)
    ///     .send();
    /// # }
    /// ```
    pub fn saturation(&mut self, saturation: f32) -> &'_ mut Self {
        self.change.saturation = Some(saturation);
        self
    }
    /// Sets change in brightness.
    ///
    /// ## Example
    /// ```
    /// use lifxi::http::prelude::*;
    /// # fn run() {
    /// let client = Client::new("foo");
    /// let result = client
    ///     .select(Selector::All)
    ///     .change_state()
    ///     .brightness(-0.2)
    ///     .send();
    /// # }
    /// ```
    pub fn brightness(&mut self, brightness: f32) -> &'_ mut Self {
        self.change.brightness = Some(brightness);
        self
    }
    /// Sets change in color temperature.
    ///
    /// ## Example
    /// ```
    /// use lifxi::http::prelude::*;
    /// # fn run() {
    /// let client = Client::new("foo");
    /// let result = client
    ///     .select(Selector::All)
    ///     .change_state()
    ///     .kelvin(-500)
    ///     .send();
    /// # }
    /// ```
    pub fn kelvin(&mut self, temp: i16) -> &'_ mut Self {
        self.change.kelvin = Some(temp);
        self
    }
    /// Sets change in infrared level.
    ///
    /// ## Example
    /// ```
    /// use lifxi::http::prelude::*;
    /// # fn run() {
    /// let client = Client::new("foo");
    /// let result = client
    ///     .select(Selector::Label("Outside".to_string()))
    ///     .change_state()
    ///     .infrared(-0.1)
    ///     .send();
    /// # }
    /// ```
    pub fn infrared(&mut self, ir: f32) -> &'_ mut Self {
        self.change.infrared = Some(ir);
        self
    }
}

impl<'a, T: Select> Attempts for ChangeState<'a, T> {
    fn set_attempts(&mut self, attempts: NonZeroU8) {
        self.attempts = Some(attempts);
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
    fn attempts(&self) -> NonZeroU8 {
        self.attempts.unwrap_or_else(unity)
    }
}

/// Specifies a list of effects to cycle through. Each request causes the cycle to advance.
///
///
/// ## Example
/// ```
/// use lifxi::http::prelude::*;
/// fn client() -> Client {
///     // TODO: Add lazy-static dependency and use it to make a shared client.
///     unimplemented!()
/// }
/// // Let's make a light show we can advance by pressing a button!
/// // Each press of our internet-connected button calls this function.
/// fn next() {
///     let red = State::builder().color(Color::Red);
///     let green = State::builder().color(Color::Green);
///     let white = State::builder().color(Color::White);
///     let shared = State::builder().color(Color::Brightness(1.0)).power(true);
///     let result = client()
///         .select(Selector::All)
///         .cycle()
///         .add(red)
///         .add(green)
///         .add(white)
///         .rev() // Let's mix it up a little!
///         .default(shared)
///         .send();
/// }
pub struct Cycle<'a, T: Select> {
    parent: &'a Selected<'a, T>,
    inner: CyclePayload<'a, T>,
    attempts: Option<NonZeroU8>,
}

impl<'a, T: Select> Cycle<'a, T> {
    pub(crate) fn new(parent: &'a Selected<'a, T>) -> Self {
        Self {
            parent,
            inner: CyclePayload::new(&parent.selector),
            attempts: None,
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
/// The message constructed by the `Cycle` request builder.
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

impl<'a, T: Select> Attempts for Cycle<'a, T> {
    fn set_attempts(&mut self, attempts: NonZeroU8) {
        self.attempts = Some(attempts);
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
    fn attempts(&self) -> NonZeroU8 {
        self.attempts.unwrap_or_else(unity)
    }
}
