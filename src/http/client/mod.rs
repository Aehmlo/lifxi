use std::string::ToString;

use crate::http::{
    selector::Select,
    state::{ColorSetting, State, StateChange},
};
use reqwest::{Client as ReqwestClient, Method};
use serde::Serialize;

mod effects;
mod scenes;
mod states;
use self::effects::*;
use self::scenes::*;
use self::states::*;

/// The result type for all requests made with the client.
pub type Result = ::std::result::Result<reqwest::Response, reqwest::Error>;

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
    pub fn new<S: ToString>(token: &S) -> Self {
        Self {
            client: ReqwestClient::new(),
            token: token.to_string(),
        }
    }
    /// Specifies the lights upon which to act.
    pub fn select<T: Select>(&self, selector: T) -> Selected<T> {
        Selected {
            client: self,
            selector,
        }
    }
    /// Creates a request to set multiple states (on multiple lights).
    ///
    /// For a simpler API when working with a single state on one or multiple lights, see
    /// [`Selected::set_state`](struct.Selected.html#method.set_state).
    pub fn set_states(&self) -> SetStates<'_> {
        SetStates {
            parent: self,
            default: None,
            new: Vec::new(),
        }
    }
    /// Creates a request to validate the given color.
    pub fn validate(&self, color: &ColorSetting) -> Request<'_, ()> {
        Request {
            client: self,
            path: format!("/color?string={}", color),
            body: (),
            method: Method::GET,
        }
    }
    /// Entry point for working with scenes.
    pub fn scenes(&self) -> Scenes {
        Scenes { client: self }
    }
}

/// Represents a terminal request.
///
/// The only thing to be done with this request is send it; no further configuration is possible.
pub struct Request<'a, U> {
    client: &'a Client,
    path: String,
    body: U,
    method: Method,
}

impl<'a, U> Request<'a, U>
where
    U: Serialize,
{
    fn send(self) -> Result {
        let token = self.client.token.as_str();
        let client = &self.client.client;
        let url = &format!("https://api.lifx.com/v1{}", self.path);
        let method = self.method;
        client
            .request(method, url)
            .bearer_auth(token)
            .json(&self.body)
            .send()
    }
}

/// Trait for configurable (non-terminal) requests to be sent conveniently.
pub trait Send<U> {
    /// Sends the request.
    fn send(self) -> Result;
}

impl<'a, T, U> Send<U> for T
where
    T: Into<Request<'a, U>>,
    U: Serialize,
{
    /// Delegates to [`Request::send`](struct.Request.html#method.send).
    fn send(self) -> Result {
        let request: Request<U> = self.into();
        request.send()
    }
}

/// A scoped request that can be used to get or set light states.
///
/// Created by [`Client::select`](struct.Client.html#method.select).
pub struct Selected<'a, T: Select> {
    client: &'a Client,
    selector: T,
}

impl<'a, T> Selected<'a, T>
where
    T: Select,
{
    /// Creates a request to get information about the selected lights (including their states).
    pub fn list(&'a self) -> Request<'a, ()> {
        Request {
            client: self.client,
            path: format!("/lights/{}", self.selector),
            body: (),
            method: Method::GET,
        }
    }
    /// Creates a request to set a uniform state on one or more lights.
    pub fn set_state(&'a self) -> SetState<'a, T> {
        SetState {
            parent: self,
            new: State::default(),
        }
    }
    /// Creates a request to incrementally change state on one or more lights.
    pub fn change_state(&'a self) -> ChangeState<'a, T> {
        ChangeState {
            parent: self,
            change: StateChange::default(),
        }
    }
    /// Creates a request to begin a "breathe" effect.
    pub fn breathe(&'a self, color: ColorSetting) -> Breathe<'a, T> {
        Breathe {
            parent: self,
            color,
            cycles: None,
            from: None,
            peak: None,
            period: None,
            persist: None,
            power_on: None,
            selector: &self.selector,
        }
    }
    /// Creates a request to begin a "pulse" effect.
    pub fn pulse(&'a self, color: ColorSetting) -> Pulse<'a, T> {
        Pulse {
            parent: self,
            color,
            cycles: None,
            from: None,
            period: None,
            persist: None,
            power_on: None,
            selector: &self.selector,
        }
    }
    /// Begins the processor of specifying a cycle.
    pub fn cycle(&'a self) -> Cycle<'a, T> {
        Cycle {
            parent: self,
            selector: &self.selector,
            direction: "forward",
            states: Vec::new(),
            default: None,
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
