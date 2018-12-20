use std::collections::HashMap;
use std::time::Duration;

use crate::http::{
    selector::Select,
    state::ColorSetting,
    state::{State, StateChange},
};
use reqwest::Client as ReqwestClient;

/// The crux of the HTTP API. Start here.
///
/// The client is the entry point for the web API interface. First construct a client, then use it
/// to perform whatever tasks necessary.
pub struct Client {
    client: ReqwestClient,
    token: String,
}

impl Client {
    /// Constructs a new `Client` with the given access token.
    pub fn new(token: String) -> Self {
        Self {
            client: ReqwestClient::new(),
            token,
        }
    }
    /// Specifies the lights upon which to act.
    pub fn select<'a, T: Select>(&'a self, selector: T) -> Selected<'a, T> {
        Selected {
            client: self,
            selector,
        }
    }
    /// Creates a request to validate the given color.
    pub fn validate<'a>(&'a self, color: &ColorSetting) -> Request<'a> {
        Request {
            client: self,
            path: format!("/color?string={}", color),
            body: None,
        }
    }
    /// Entry point for working with scenes.
    pub fn scenes<'a>(&'a self) -> Scenes<'a> {
        Scenes { client: self }
    }
}

/// Represents a terminal request.
///
/// The only thing to be done with this request is send it; no further configuration is possible.
pub struct Request<'a> {
    client: &'a Client,
    path: String,
    body: Option<HashMap<&'static str, String>>,
}

impl<'a> Request<'a> {
    fn send(self) {
        let client = self.client;
        let _ = client.client;
        let _ = client.token;
        let _ = self.path;
        let _ = self.body;
        unimplemented!()
    }
}

/// Trait for configurable (non-terminal) requests to be sent conveniently.
pub trait Send {
    /// Sends the request.
    ///
    /// Someday, we'll also return the result.
    fn send(self);
}

impl<'a, T> Send for T
where
    T: Into<Request<'a>>,
{
    /// Delegates to [`Request::send`](struct.Request.html#method.send).
    fn send(self) {
        let request: Request = self.into();
        request.send()
    }
}

impl<'a, T: Select> From<Toggle<'a, T>> for Request<'a> {
    fn from(toggle: Toggle<'a, T>) -> Self {
        Self {
            client: toggle.parent.client,
            path: format!("/lights/{}/toggle", toggle.parent.selector),
            body: None,
        }
    }
}

impl<'a, T: Select> From<SetState<'a, T>> for Request<'a> {
    fn from(state: SetState<'a, T>) -> Self {
        Self {
            client: state.parent.client,
            path: format!("/lights/{}/state", state.parent.selector),
            body: Some(state.params()),
        }
    }
}

impl<'a, T: Select> From<ChangeState<'a, T>> for Request<'a> {
    fn from(delta: ChangeState<'a, T>) -> Self {
        Self {
            client: delta.parent.client,
            path: format!("/lights/{}/state/delta", delta.parent.selector),
            body: Some(delta.params()),
        }
    }
}

impl<'a> From<Activate<'a>> for Request<'a> {
    fn from(activate: Activate<'a>) -> Self {
        let mut body = HashMap::new();
        if let Some(transition) = activate.transition {
            body.insert("duration", format!("{}", transition.as_secs()));
        }
        // body.insert("ignore".to_string(), activate.ignore_str());
        // body.insert("overrides".to_string(), format!("{}", state));
        Self {
            client: activate.parent.client,
            path: format!("/scenes/scene_id:{}/activate", activate.uuid),
            body: Some(body),
        }
    }
}

/// A scoped request that can be used to get or set light states.
///
/// Created by [`Client::select`](struct.Client.html#method.select).
pub struct Selected<'a, T: Select> {
    client: &'a Client,
    selector: T,
}

/// A scoped request to toggle specific lights which may be further customized.
pub struct Toggle<'a, T: Select> {
    parent: &'a Selected<'a, T>,
}

impl<'a, T: Select> Toggle<'a, T> {
    /// Sets the transition time for the toggle.
    pub fn transition(&self, duration: Duration) -> Request<'a> {
        let mut body = HashMap::new();
        body.insert("duration", format!("{}", duration.as_secs()));
        Request {
            client: self.parent.client,
            path: format!("/lights/{}", self.parent.selector),
            body: Some(body),
        }
    }
}
/// A scoped request to uniformly set the state for all selected bulbs.
pub struct SetState<'a, T: Select> {
    parent: &'a Selected<'a, T>,
    new: State,
}

impl<'a, T: Select> SetState<'a, T> {
    /// Sets the power state of all selected bulbs.
    pub fn power(&'a mut self, on: bool) -> &'a mut SetState<'a, T> {
        self.new.power = Some(on);
        self
    }
    /// Sets the color of all selected bulbs.
    pub fn color(&'a mut self, color: ColorSetting) -> &'a mut SetState<'a, T> {
        self.new.color = Some(color);
        self
    }
    /// Sets the brightness of all selected bulbs (overriding color settings).
    pub fn brightness(&'a mut self, brightness: f32) -> &'a mut SetState<'a, T> {
        self.new.brightness = Some(brightness);
        self
    }
    /// Sets the transition time (duration) for the change.
    pub fn transition(&'a mut self, duration: Duration) -> &'a mut SetState<'a, T> {
        self.new.duration = Some(duration);
        self
    }
    /// Sets the infrared level, if applicable.
    pub fn infrared(&'a mut self, ir: f32) -> &'a mut SetState<'a, T> {
        self.new.infrared = Some(ir);
        self
    }
    /// Delegates to [`Request::send`](struct.Request.html#method.send).
    pub fn send(self) {
        let request: Request = self.into();
        request.send()
    }
    fn params(&self) -> HashMap<&'static str, String> {
        let mut params = HashMap::new();
        if let Some(power) = self.new.power {
            params.insert("power", (if power { "on" } else { "off" }).to_string());
        }
        if let Some(color) = &self.new.color {
            params.insert("color", format!("{}", color));
        }
        if let Some(brightness) = self.new.brightness {
            params.insert("brightness", format!("{:.2}", brightness));
        }
        if let Some(duration) = self.new.duration {
            params.insert("duration", format!("{}", duration.as_secs()));
        }
        if let Some(infrared) = self.new.infrared {
            params.insert("infrared", format!("{:.2}", infrared));
        }
        params
    }
}

/// A scoped request to uniformly change the state for all selected bulbs.
pub struct ChangeState<'a, T: Select> {
    parent: &'a Selected<'a, T>,
    change: StateChange,
}

impl<'a, T: Select> ChangeState<'a, T> {
    /// Sets target power state.
    pub fn power(&'a mut self, on: bool) -> &'a mut Self {
        self.change.power = Some(on);
        self
    }
    /// Sets transition duration.
    pub fn transition(&'a mut self, duration: Duration) -> &'a mut Self {
        self.change.duration = Some(duration);
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
    fn params(&self) -> HashMap<&'static str, String> {
        let mut params = HashMap::new();
        if let Some(power) = self.change.power {
            params.insert("power", (if power { "on" } else { "off" }).to_string());
        }
        if let Some(hue) = self.change.hue {
            params.insert("hue", format!("{:.2}", hue));
        }
        if let Some(saturation) = self.change.saturation {
            params.insert("saturation", format!("{:.2}", saturation));
        }
        if let Some(brightness) = self.change.brightness {
            params.insert("brightness", format!("{:.2}", brightness));
        }
        if let Some(duration) = self.change.duration {
            params.insert("duration", format!("{}", duration.as_secs()));
        }
        if let Some(kelvin) = self.change.kelvin {
            params.insert("kelvin", format!("{}", kelvin));
        }
        if let Some(infrared) = self.change.infrared {
            params.insert("infrared", format!("{:.2}", infrared));
        }
        params
    }
}

impl<'a, T> Selected<'a, T>
where
    T: Select,
{
    /// Creates a request to get information about the selected lights (including their states).
    pub fn list(&'a self) -> Request<'a> {
        Request {
            client: self.client,
            path: format!("/lights/{}", self.selector),
            body: None,
        }
    }
    /// Creates a request to set a uniform state on one or more lights.
    pub fn set_state(&'a self) -> SetState<'a, T> {
        SetState {
            parent: self,
            new: State::default(),
        }
    }
    /// Creates a request to toggle power to the specified light(s), with an optional transition
    /// time.
    ///
    /// ### Notes
    /// All specified lights will have the same power state after this request is processed; if all
    /// are off, all will be turned on, but if any are on, all will be turned off.
    pub fn toggle(&'a self) -> Toggle<'a, T> {
        Toggle { parent: self }
    }
}

/// A waypoint in working with scenes.
///
/// This struct is basically useless; call one of its member methods to do anything interesting.
pub struct Scenes<'a> {
    client: &'a Client,
}

impl<'a> Scenes<'a> {
    /// Creates a terminal request to list all scenes.
    pub fn list(&'a self) -> Request<'a> {
        Request {
            client: self.client,
            path: format!("/scenes"),
            body: None,
        }
    }
    /// Creates a configurable request for activating a specific scene.
    pub fn activate(&'a self, uuid: String) -> Activate<'a> {
        Activate {
            parent: self,
            uuid,
            transition: None,
            ignore: Vec::new(),
            overrides: None,
        }
    }
}

pub struct Activate<'a> {
    parent: &'a Scenes<'a>,
    uuid: String,
    transition: Option<Duration>,
    ignore: Vec<String>,
    overrides: Option<State>,
}

impl<'a> Activate<'a> {
    /// Sets the transition time for the scene activation.
    pub fn transition(&'a mut self, transition: Duration) -> &'a mut Self {
        self.transition = Some(transition);
        self
    }
    /// Adds a property to the list of ignored properties when changing.
    ///
    /// This method takes a string for now; in later versions, it will be strongly-typed.
    pub fn ignore(&'a mut self, s: impl Into<String>) -> &'a mut Self {
        self.ignore.push(s.into());
        self
    }
    /// Sets an overriding state that will take priority over all scene attributes.
    pub fn overwrite(&'a mut self, state: State) -> &'a mut Self {
        self.overrides = Some(state);
        self
    }
}
