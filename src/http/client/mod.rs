use std::string::ToString;

use crate::http::{selector::Select, state::Color};
use reqwest::{Client as ReqwestClient, Method};
use serde::Serialize;

mod effects;
mod scenes;
mod states;
use self::effects::*;
use self::scenes::*;
use self::states::*;

/// Trait enabling non-terminal conversion of request builders to requests.
pub trait AsRequest<S: Serialize> {
    /// The HTTP verb to be used.
    fn method() -> reqwest::Method;
    /// A reference to the shared client (so we can reuse it).
    fn client(&self) -> &'_ Client;
    /// The relative path (to the API root) of the appropriate endpoint.
    fn path(&self) -> String;
    /// The request body to be used, as configured by the user.
    fn body(&self) -> &'_ S;
}

/// The result type for all requests made with the client.
pub type ClientResult = ::std::result::Result<reqwest::Response, reqwest::Error>;

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
        SetStates::new(self)
    }
    /// Creates a request to validate the given color.
    pub fn validate(&self, color: &Color) -> Request<'_, ()> {
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
pub struct Request<'a, S> {
    client: &'a Client,
    path: String,
    body: S,
    method: Method,
}

impl<'a, S> Request<'a, S>
where
    S: Serialize,
{
    fn send(self) -> ClientResult {
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
pub trait Send<S> {
    /// Sends the request.
    fn send(&self) -> ClientResult;
}

impl<'a, T, S> Send<S> for T
where
    T: AsRequest<S>,
    S: Serialize,
{
    /// Delegates to [`Request::send`](struct.Request.html#method.send).
    fn send(&self) -> ClientResult {
        let request = Request {
            body: self.body(),
            client: self.client(),
            method: Self::method(),
            path: self.path(),
        };
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
        SetState::new(self)
    }
    /// Creates a request to incrementally change state on one or more lights.
    pub fn change_state(&'a self) -> ChangeState<'a, T> {
        ChangeState::new(self)
    }
    /// Creates a request to begin a "breathe" effect.
    pub fn breathe(&'a self, color: Color) -> Breathe<'a, T> {
        Breathe::new(self, color)
    }
    /// Creates a request to begin a "pulse" effect.
    pub fn pulse(&'a self, color: Color) -> Pulse<'a, T> {
        Pulse::new(self, color)
    }
    /// Begins the processor of specifying a cycle.
    pub fn cycle(&'a self) -> Cycle<'a, T> {
        Cycle::new(self)
    }
    /// Creates a request to toggle power to the specified light(s), with an optional transition
    /// time.
    ///
    /// ### Notes
    /// All specified lights will have the same power state after this request is processed; if all
    /// are off, all will be turned on, but if any are on, all will be turned off.
    pub fn toggle(&'a self) -> Toggle<'a, T> {
        Toggle::new(self)
    }
}
