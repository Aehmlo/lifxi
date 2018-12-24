use std::num::NonZeroU8;
use std::string::ToString;
use std::time::{Duration, Instant, SystemTime};

use crate::http::{selector::Select, state::Color};
use reqwest::{Client as ReqwestClient, Method};
use serde::Serialize;

#[inline]
pub(crate) fn unity() -> NonZeroU8 {
    NonZeroU8::new(1).expect("1 == 0")
}

mod effects;
mod scenes;
mod states;
pub use self::effects::*;
pub use self::scenes::*;
pub use self::states::*;

/// Contains useful utilities for working with the LIFX HTTP API.
///
/// Use the prelude to maintain the convenience of glob importing without overly polluting the namespace.
///
/// ## Usage
/// ```
/// use lifxi::http::prelude::*;
/// ```
pub mod prelude {
    pub use crate::http::Client;
    pub use crate::http::Color;
    pub use crate::http::ColorParseError;
    pub use crate::http::ColorValidationError;
    pub use crate::http::Combine;
    pub use crate::http::Randomize;
    pub use crate::http::Retry;
    pub use crate::http::Selector;
    pub use crate::http::SelectorParseError;
    pub use crate::http::Send;
    pub use crate::http::State;
    pub use crate::http::StateChange;
}

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
    /// The number of attempts to be made.
    fn attempts(&self) -> NonZeroU8;
}

/// The result type for all requests made with the client.
pub type ClientResult = Result<reqwest::Response, Error>;

/// The crux of the HTTP API. Start here.
///
/// The client is the entry point for the web API interface. First construct a client, then use it
/// to perform whatever tasks necessary.
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
///     .power(true)
///     .retry()
///     .send();
/// # }
/// ```
pub struct Client {
    client: ReqwestClient,
    token: String,
}

impl Client {
    /// Constructs a new `Client` with the given access token.
    ///
    /// ## Examples
    /// ```
    /// use lifxi::http::prelude::*;
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
    /// use lifxi::http::prelude::*;
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
            attempts: unity(),
        }
    }
    /// Entry point for working with scenes.
    ///
    /// See [`Scenes`](struct.Scenes.html).
    pub fn scenes(&self) -> Scenes {
        Scenes { client: self }
    }
}

/// Represents an error encountered when sending a request.
///
/// Errors may come from a variety of sources, but the ones handled most directly by this crate are
/// client errors. If a client error occurs, we map it to a user-friendly error variant; if another
/// error occurs, we just wrap it and return it. This means that errors stemming from your mistakes
/// are easier to diagnose than errors from the middleware stack.
pub enum Error {
    /// The API is enforcing a rate limit. The associated value is the time at which the rate limit
    /// will be lifted, if it was specified.
    RateLimited(Option<Instant>),
    /// The request was malformed and should not be reattempted (HTTP 400 or 422).
    /// If this came from library methods, please
    /// [create an issue](https://github.com/Aehmlo/lifxi/issues/new). If you're using a custom
    /// color somewhere, please first [validate it](struct.Client.html#method.validate). Otherwise,
    /// check for empty strings.
    BadRequest,
    /// The specified access token was invalid (HTTP 401).
    BadAccessToken,
    /// The requested OAuth scope was invalid (HTTP 403).
    BadOAuthScope,
    /// The given selector (or scene UUID) did not match anything associated with this account
    /// (HTTP 404). The URL is returned as well, if possible, to help with troubleshooting.
    NotFound(Option<String>),
    /// The API server encountered an error, but the request was (seemingly) valid (HTTP 5xx).
    Server(Option<reqwest::StatusCode>, reqwest::Error),
    /// An HTTP stack error was encountered.
    Http(reqwest::Error),
    /// A serialization error was encountered.
    Serialization(reqwest::Error),
    /// A bad redirect was encountered.
    Redirect(reqwest::Error),
    /// A miscellaneous client error occurred (HTTP 4xx).
    Client(Option<reqwest::StatusCode>, reqwest::Error),
    /// Some other error occured.
    Other(reqwest::Error),
}

impl Error {
    /// Whether the error is a client error (indicating that the request should not be retried
    /// without modification).
    fn is_client_error(&self) -> bool {
        use self::Error::*;
        match self {
            RateLimited(_)
            | BadRequest
            | BadAccessToken
            | BadOAuthScope
            | NotFound(_)
            | Client(_, _) => true,
            _ => false,
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        use self::Error::*;
        use reqwest::StatusCode;
        if err.is_client_error() {
            match err.status() {
                Some(StatusCode::BAD_REQUEST) | Some(StatusCode::UNPROCESSABLE_ENTITY) => {
                    BadRequest
                }
                Some(StatusCode::UNAUTHORIZED) => BadAccessToken,
                Some(StatusCode::FORBIDDEN) => BadOAuthScope,
                Some(StatusCode::NOT_FOUND) => NotFound(err.url().map(|u| u.as_str().to_string())),
                s => Client(s, err),
            }
        } else if err.is_http() {
            Http(err)
        } else if err.is_serialization() {
            Serialization(err)
        } else if err.is_redirect() {
            Redirect(err)
        } else if err.is_server_error() {
            Server(err.status(), err)
        } else {
            Other(err)
        }
    }
}

/// Represents a terminal request.
///
/// The only thing to be done with this request is [send it](#method.send).
pub struct Request<'a, S> {
    client: &'a Client,
    path: String,
    body: S,
    method: Method,
    attempts: NonZeroU8,
}

impl<'a, S> Request<'a, S>
where
    S: Serialize,
{
    /// Sends the request, returning the result.
    ///
    /// Requests are synchronous, so this method blocks.
    pub fn send(&self) -> ClientResult {
        use reqwest::StatusCode;
        let header = |name: &'static str| reqwest::header::HeaderName::from_static(name);
        let token = self.client.token.as_str();
        let client = &self.client.client;
        let url = &format!("https://api.lifx.com/v1{}", self.path);
        let method = self.method.clone();
        let result = client
            .request(method, url)
            .bearer_auth(token)
            .json(&self.body)
            .send()?;
        let headers = result.headers();
        let reset = headers.get(&header("x-ratelimit-reset")).map(|s| {
            if let Ok(val) = s.to_str() {
                if let Ok(future) = val.parse::<u64>() {
                    let now = (SystemTime::now(), Instant::now());
                    if let Ok(timestamp) = now
                        .0
                        .duration_since(SystemTime::UNIX_EPOCH)
                        .map(|t| t.as_secs())
                    {
                        return now.1 + Duration::from_secs(future - timestamp);
                    }
                }
            }
            Instant::now() + Duration::from_secs(60)
        });
        let mut result = result.error_for_status().map_err(|e| {
            if e.status() == Some(StatusCode::TOO_MANY_REQUESTS) {
                Error::RateLimited(reset)
            } else {
                e.into()
            }
        });
        for _ in 1..self.attempts.get() {
            match result {
                Ok(r) => {
                    return Ok(r);
                }
                Err(e) => {
                    if let Error::RateLimited(Some(t)) = e {
                        // Wait until we're allowed to try again.
                        ::std::thread::sleep(t - Instant::now());
                    } else if e.is_client_error() {
                        return Err(e);
                    }
                    result = self.send();
                }
            }
        }
        result
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
    T: AsRequest<S> + Retry,
    S: Serialize,
{
    /// Delegates to [`Request::send`](struct.Request.html#method.send).
    fn send(&self) -> ClientResult {
        let request = Request {
            body: self.body(),
            client: self.client(),
            method: Self::method(),
            path: self.path(),
            attempts: self.attempts(),
        };
        request.send()
    }
}

/// Enables automatic implementation of [`Retry`](trait.Retry.html).
#[doc(hidden)]
pub trait Attempts {
    /// Updates the number of times to retry the request.
    fn set_attempts(&mut self, attempts: NonZeroU8);
}

impl<'a, S: Serialize> Attempts for Request<'a, S> {
    fn set_attempts(&mut self, attempts: NonZeroU8) {
        self.attempts = attempts;
    }
}

/// Trait enabling retrying of failed requests.
pub trait Retry {
    /// Retries the corresponding request once.
    fn retry(&mut self) -> &'_ mut Self;
    /// Retries the corresponding request the given number of times.
    fn retries(&mut self, n: NonZeroU8) -> &'_ mut Self;
}

impl<T> Retry for T
where
    T: Attempts,
{
    fn retry(&mut self) -> &'_ mut Self {
        self.retries(unity())
    }
    fn retries(&mut self, n: NonZeroU8) -> &'_ mut Self {
        self.set_attempts(n);
        self
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
    /// use lifxi::http::prelude::*;
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
            attempts: unity(),
        }
    }
    /// Creates a request to set a uniform state on one or more lights.
    ///
    /// ## Example
    /// ```
    /// use lifxi::http::prelude::*;
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
    /// use lifxi::http::prelude::*;
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
    /// use lifxi::http::prelude::*;
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
    /// use lifxi::http::prelude::*;
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
    pub fn toggle(&'a self) -> Toggle<'a, T> {
        Toggle::new(self)
    }
}
