use crate::http::{
    client::{Client, Request},
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
    pub fn activate<S: ToString>(&'a self, uuid: S) -> Activate<'a> {
        Activate {
            parent: self,
            uuid: uuid.to_string(),
            transition: None,
            ignore: Vec::new(),
            overrides: None,
        }
    }
}

/// A configurable request for activating a specified scene.
#[derive(Serialize)]
pub struct Activate<'a> {
    #[serde(skip)]
    parent: &'a Scenes<'a>,
    #[serde(skip)]
    uuid: String,
    #[serde(rename = "duration", skip_serializing_if = "Option::is_none")]
    transition: Option<Duration>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    ignore: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    overrides: Option<State>,
}

impl<'a> Activate<'a> {
    /// Sets the transition time for the scene activation.
    pub fn transition<D: Into<Duration>>(&'a mut self, transition: D) -> &'a mut Self {
        self.transition = Some(transition.into());
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

impl<'a, 'b: 'a> From<&'b Activate<'a>> for Request<'a, &Activate<'b>> {
    fn from(activate: &'b Activate<'a>) -> Self {
        Request {
            body: &activate,
            client: activate.parent.client,
            method: Method::PUT,
            path: format!("/scenes/scene_id:{}/activate", activate.uuid),
        }
    }
}
