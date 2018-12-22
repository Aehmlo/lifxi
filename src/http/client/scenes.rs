use crate::http::{
    client::{AsRequest, Client, Request},
    state::{Duration, State},
};
use reqwest::Method;

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
    /// use lifxi::http::*;
    /// # fn run() {
    /// let client = Client::new("foo");
    /// let scenes = client
    ///     .scenes()
    ///     .list()
    ///     .send();
    /// # }
    pub fn list(&'a self) -> Request<'a, ()> {
        Request {
            client: self.client,
            path: "/scenes".to_string(),
            body: (),
            method: Method::GET,
        }
    }
    /// Creates a configurable request for activating a specific scene.
    ///
    /// ## Example
    /// ```
    /// use lifxi::http::*;
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
}

/// A configurable request for activating a specified scene.
///
/// ## Example
/// ```
/// use lifxi::http::*;
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
pub struct Activate<'a> {
    parent: &'a Scenes<'a>,
    uuid: String,
    inner: ActivatePayload,
}

impl<'a> Activate<'a> {
    pub(crate) fn new(parent: &'a Scenes<'a>, uuid: String) -> Self {
        Self {
            parent,
            uuid,
            inner: ActivatePayload::default(),
        }
    }
    /// Sets the transition time for the scene activation.
    ///
    /// ## Example
    /// ```
    /// use lifxi::http::*;
    /// # fn run() {
    /// let client = Client::new("foo");
    /// let result = client
    ///     .scenes()
    ///     .activate("asdf")
    ///     .transition(::std::time::Duration::new(7, 0))
    ///     .send();
    /// # }
    pub fn transition<D: Into<Duration>>(&'a mut self, transition: D) -> &'a mut Self {
        self.inner.transition = Some(transition.into());
        self
    }
    /// Adds a property to the list of ignored properties when changing.
    ///
    /// This method takes a string for now; in later versions, it will be strongly-typed.
    ///
    /// ## Example
    /// ```
    /// use lifxi::http::*;
    /// # fn run() {
    /// let client = Client::new("foo");
    /// let result = client
    ///     .scenes()
    ///     .activate("asdf")
    ///     .ignore("brightness")
    ///     .ignore("saturation")
    ///     .send();
    /// # }
    #[allow(clippy::needless_pass_by_value)]
    pub fn ignore(&'a mut self, s: impl ToString) -> &'a mut Self {
        self.inner.ignore.push(s.to_string());
        self
    }
    /// Sets an overriding state that will take priority over all scene attributes.
    ///
    /// ## Example
    /// ```
    /// use lifxi::http::*;
    /// # fn run() {
    /// let client = Client::new("foo");
    /// let result = client
    ///     .scenes()
    ///     .activate("asdf")
    ///     .overwrite(State::builder().power(true))
    ///     .send();
    /// # }
    pub fn overwrite(&'a mut self, state: State) -> &'a mut Self {
        self.inner.overrides = Some(state);
        self
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
}
