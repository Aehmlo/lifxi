use crate::http::{
    client::{AsRequest, Client, Request},
    state::{Duration, State},
};
use reqwest::Method;

/// A waypoint in working with scenes.
///
/// This struct is basically useless; call one of its member methods to do anything interesting.
pub struct Scenes<'a> {
    pub(crate) client: &'a Client,
}

impl<'a> Scenes<'a> {
    /// Creates a terminal request to list all scenes.
    pub fn list(&'a self) -> Request<'a, ()> {
        Request {
            client: self.client,
            path: "/scenes".to_string(),
            body: (),
            method: Method::GET,
        }
    }
    /// Creates a configurable request for activating a specific scene.
    pub fn activate<S: ToString>(&'a self, uuid: &S) -> Activate<'a> {
        Activate::new(self, uuid.to_string())
    }
}

#[derive(Clone, Default, Serialize)]
#[doc(hidden)]
pub struct ActivatePayload {
    #[serde(rename = "duration", skip_serializing_if = "Option::is_none")]
    transition: Option<Duration>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    ignore: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    overrides: Option<State>,
}

/// A configurable request for activating a specified scene.
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
    pub fn transition<D: Into<Duration>>(&'a mut self, transition: D) -> &'a mut Self {
        self.inner.transition = Some(transition.into());
        self
    }
    /// Adds a property to the list of ignored properties when changing.
    ///
    /// This method takes a string for now; in later versions, it will be strongly-typed.
    pub fn ignore(&'a mut self, s: impl Into<String>) -> &'a mut Self {
        self.inner.ignore.push(s.into());
        self
    }
    /// Sets an overriding state that will take priority over all scene attributes.
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
