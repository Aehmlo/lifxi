use std::string::ToString;

use crate::http::{selector::Select, state::Color};
use reqwest::{Client as ReqwestClient, Method};
use serde::Serialize;

mod effects;
mod scenes;
mod states;
pub use self::effects::*;
pub use self::scenes::*;
pub use self::states::*;

/// Trait enabling conversion of non-terminal request builders to requests.
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
pub type ClientResult = Result<reqwest::Response, reqwest::Error>;

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
    ///
    /// ## Examples
    /// ```
    /// use lifxi::http::*;
    /// let secret = "foo";
    /// let client = Client::new(secret);
    /// let secret = "foo".to_string();
    /// let client = Client::new(secret);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    pub fn new<S: ToString>(token: S) -> Self {
        Self {
            client: ReqwestClient::new(),
            token: token.to_string(),
        }
    }
    /// Specifies the lights upon which to act.
    ///
    /// See [the documentation for `Selected<T>`](struct.Selected.html) to understand why this is
    /// useful.
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
    ///
    /// ## Example
    /// ```
    /// use lifxi::http::*;
    /// # fn run() {
    /// let secret = "foo";
    /// let client = Client::new(secret);
    /// let color = Color::Custom("cyan".to_string());
    /// let is_valid = client
    ///     .validate(&color)
    ///     .send()
    ///     .is_ok();
    /// # }
    /// ```
    pub fn validate(&self, color: &Color) -> Request<'_, ()> {
        Request {
            client: self,
            path: format!("/color?string={}", color),
            body: (),
            method: Method::GET,
        }
    }
    /// Entry point for working with scenes.
    ///
    /// See [`Scenes`](struct.Scenes.html).
    pub fn scenes(&self) -> Scenes {
        Scenes { client: self }
    }
}

/// Represents a terminal request.
///
/// The only thing to be done with this request is [send it](#method.send); no further configuration is possible.
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
    /// Sends the request, returning the result.
    ///
    /// Requests are synchronous, so this method blocks.
    pub fn send(&self) -> ClientResult {
        let token = self.client.token.as_str();
        let client = &self.client.client;
        let url = &format!("https://api.lifx.com/v1{}", self.path);
        let method = self.method.clone();
        client
            .request(method, url)
            .bearer_auth(token)
            .json(&self.body)
            .send()?
            .error_for_status()
    }
}

/// Trait for configurable (non-terminal) requests to be sent conveniently.
pub trait Send<S> {
    /// Sends the request.
    ///
    /// This method delegates to `Request::send`, so take a look  at
    /// [that documentation](struct.Request.html#method.send) for more information.
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
    ///
    /// ## Example
    /// ```
    /// use lifxi::http::*;
    /// # fn run() {
    /// let client = Client::new("foo");
    /// let lights = client
    ///     .select(Selector::All)
    ///     .list()
    ///     .send();
    /// # }
    /// ```
    pub fn list(&'a self) -> Request<'a, ()> {
        Request {
            client: self.client,
            path: format!("/lights/{}", self.selector),
            body: (),
            method: Method::GET,
        }
    }
    /// Creates a request to set a uniform state on one or more lights.
    ///
    /// ## Example
    /// ```
    /// use lifxi::http::*;
    /// # fn run() {
    /// let client = Client::new("foo");
    /// let lights = client
    ///     .select(Selector::All)
    ///     .set_state()
    ///     .color(Color::Red)
    ///     .power(true)
    ///     .brightness(0.1)
    ///     .transition(::std::time::Duration::new(7, 0))
    ///     .infrared(0.8)
    ///     .send();
    /// # }
    /// ```
    pub fn set_state(&'a self) -> SetState<'a, T> {
        SetState::new(self)
    }
    /// Creates a request to incrementally change state on one or more lights.
    ///
    /// ## Example
    /// ```
    /// use lifxi::http::*;
    /// # fn run() {
    /// let client = Client::new("foo");
    /// let lights = client
    ///     .select(Selector::All)
    ///     .change_state()
    ///     .power(true)
    ///     .brightness(0.4)
    ///     .saturation(-0.1)
    ///     .brightness(0.1)
    ///     .kelvin(-100)
    ///     .transition(::std::time::Duration::new(7, 0))
    ///     .infrared(0.1)
    ///     .send();
    /// # }
    /// ```
    pub fn change_state(&'a self) -> ChangeState<'a, T> {
        ChangeState::new(self)
    }
    /// Creates a request to begin a "breathe" effect.
    ///
    /// ## Example
    /// ```
    /// use lifxi::http::*;
    /// # fn run() {
    /// let client = Client::new("foo");
    /// let lights = client
    ///     .select(Selector::All)
    ///     .breathe(Color::Orange)
    ///     .from(Color::Purple)
    ///     .power(true)
    ///     .cycles(100)
    ///     .period(::std::time::Duration::new(20, 0))
    ///     .peak(0.8)
    ///     .persist(true)
    ///     .send();
    /// # }
    /// ```
    pub fn breathe(&'a self, color: Color) -> Breathe<'a, T> {
        Breathe::new(self, color)
    }
    /// Creates a request to begin a "pulse" effect.
    ///
    /// ## Example
    /// ```
    /// use lifxi::http::*;
    /// # fn run() {
    /// let client = Client::new("foo");
    /// let lights = client
    ///     .select(Selector::All)
    ///     .pulse(Color::Orange)
    ///     .from(Color::Purple)
    ///     .power(true)
    ///     .cycles(100)
    ///     .period(::std::time::Duration::new(20, 0))
    ///     .persist(true)
    ///     .send();
    /// # }
    /// ```
    pub fn pulse(&'a self, color: Color) -> Pulse<'a, T> {
        Pulse::new(self, color)
    }
    /// Begins the process of specifying a cycle.
    ///
    /// Cycles provide a convenient method of moving through a set of changes without client-side
    /// logic; the API keeps track of the state of the bulb and will move to the next appropriate
    /// state upon repeated requests.
    ///
    /// ## Example
    /// ```
    /// use lifxi::http::*;
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
    pub fn cycle(&'a self) -> Cycle<'a, T> {
        Cycle::new(self)
    }
    /// Creates a request to toggle power to the selected light(s), with an optional transition
    /// time (see [`Toggle::transition`](struct.Toggle.html#method.transition) for details).
    ///
    /// ## Notes
    /// All selected lights will have the same power state after this request is processed; if all
    /// are off, all will be turned on, but if any are on, all will be turned off.
    ///
    /// ## Example
    /// ```
    /// use lifxi::http::*;
    /// # fn run() {
    /// let client = Client::new("foo");
    /// let result = client
    ///     .select(Selector::All)
    ///     .toggle()
    ///     .transition(::std::time::Duration::new(2, 0))
    ///     .send();
    /// # }
    /// ```
    pub fn toggle(&'a self) -> Toggle<'a, T> {
        Toggle::new(self)
    }
}
