use crate::http::{
    client::{unity, AsRequest, Attempts, Client, Request},
    state::{Duration, State},
};
use reqwest::Method;
use std::num::NonZeroU8;

/// A waypoint in working with scenes.
///
/// This struct is basically useless; call one of its [member methods](#methods) to do anything
/// interesting.
pub struct Scenes<'a> {
    pub(crate) client: &'a Client,
}

impl<'a> Scenes<'a> {
    /// Creates a terminal request to list all scenes.
    ///
    /// ## Example
    /// ```
    /// use lifxi::http::prelude::*;
    /// # fn run() {
    /// let client = Client::new("foo");
    /// let scenes = client
    ///     .scenes()
    ///     .list()
    ///     .send();
    /// # }
    /// ```
    pub fn list(&'a self) -> Request<'a, ()> {
        Request {
            client: self.client,
            path: "/scenes".to_string(),
            body: (),
            method: Method::GET,
            attempts: unity(),
        }
    }
    /// Creates a configurable request for activating a specific scene.
    ///
    /// ## Example
    /// ```
    /// use lifxi::http::prelude::*;
    /// # fn run() {
    /// let client = Client::new("foo");
    /// let result = client
    ///     .scenes()
    ///     .activate("asdf")
    ///     .ignore("brightness")
    ///     .ignore("saturation")
    ///     .transition(::std::time::Duration::new(7, 0))
    ///     .overwrite(State::builder().power(true))
    ///     .send();
    /// # }
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    pub fn activate<S: ToString>(&'a self, uuid: S) -> Activate<'a> {
        Activate::new(self, uuid.to_string())
    }
}

#[derive(Clone, Default, Serialize)]
#[doc(hidden)]
/// The message constructed by the `Activate` request builder.
pub struct ActivatePayload {
    #[serde(rename = "duration", skip_serializing_if = "Option::is_none")]
    transition: Option<Duration>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    ignore: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    overrides: Option<State>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fast: Option<bool>,
}

/// A configurable request for activating a specified scene.
///
/// ## Example
/// ```
/// use lifxi::http::prelude::*;
/// # fn run() {
/// let client = Client::new("foo");
/// let result = client
///     .scenes()
///     .activate("asdf")
///     .ignore("brightness")
///     .ignore("saturation")
///     .transition(::std::time::Duration::new(7, 0))
///     .overwrite(State::builder().power(true))
///     .send();
/// # }
/// ```
pub struct Activate<'a> {
    parent: &'a Scenes<'a>,
    uuid: String,
    inner: ActivatePayload,
    attempts: Option<NonZeroU8>,
}

impl<'a> Activate<'a> {
    pub(crate) fn new(parent: &'a Scenes<'a>, uuid: String) -> Self {
        Self {
            parent,
            uuid,
            inner: ActivatePayload::default(),
            attempts: None,
        }
    }
    /// Sets the transition time for the scene activation.
    ///
    /// ## Example
    /// ```
    /// use lifxi::http::prelude::*;
    /// # fn run() {
    /// let client = Client::new("foo");
    /// let result = client
    ///     .scenes()
    ///     .activate("asdf")
    ///     .transition(::std::time::Duration::new(7, 0))
    ///     .send();
    /// # }
    /// ```
    pub fn transition<D: Into<Duration>>(&mut self, transition: D) -> &'_ mut Self {
        self.inner.transition = Some(transition.into());
        self
    }
    /// Adds a property to the list of ignored properties when changing.
    ///
    /// This method takes a string for now; in later versions, it will be strongly-typed.
    ///
    /// ## Example
    /// ```
    /// use lifxi::http::prelude::*;
    /// # fn run() {
    /// let client = Client::new("foo");
    /// let result = client
    ///     .scenes()
    ///     .activate("asdf")
    ///     .ignore("brightness")
    ///     .ignore("saturation")
    ///     .send();
    /// # }
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    pub fn ignore(&mut self, s: impl ToString) -> &'_ mut Self {
        self.inner.ignore.push(s.to_string());
        self
    }
    /// Sets an overriding state that will take priority over all scene attributes.
    ///
    /// ## Example
    /// ```
    /// use lifxi::http::prelude::*;
    /// # fn run() {
    /// let client = Client::new("foo");
    /// let result = client
    ///     .scenes()
    ///     .activate("asdf")
    ///     .overwrite(State::builder().power(true))
    ///     .send();
    /// # }
    /// ```
    pub fn overwrite(&mut self, state: State) -> &'_ mut Self {
        self.inner.overrides = Some(state);
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
    ///     .scenes()
    ///     .activate("asdf")
    ///     .overwrite(State::builder().power(true))
    ///     .fast(true)
    ///     .send();
    /// # }
    /// ```
    pub fn fast(&mut self, fast: bool) -> &'_ mut Self {
        self.inner.fast = Some(fast);
        self
    }
}

impl<'a> Attempts for Activate<'a> {
    fn set_attempts(&mut self, attempts: NonZeroU8) {
        self.attempts = Some(attempts);
    }
}

impl<'a> AsRequest<ActivatePayload> for Activate<'a> {
    fn method() -> reqwest::Method {
        Method::PUT
    }
    fn client(&self) -> &'_ Client {
        self.parent.client
    }
    fn path(&self) -> String {
        format!("/scenes/scene_id:{}/activate", self.uuid)
    }
    fn body(&self) -> &'_ ActivatePayload {
        &self.inner
    }
    fn attempts(&self) -> NonZeroU8 {
        self.attempts.unwrap_or_else(unity)
    }
}
