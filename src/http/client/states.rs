use crate::http::{
    client::{Client, Request, Selected},
    state::{ColorSetting, Duration, Power, State, StateChange},
    Select,
};
use reqwest::Method;

/// A scoped request to toggle specific lights which may be further customized.
pub struct Toggle<'a, T: Select> {
    pub(crate) parent: &'a Selected<'a, T>,
}

impl<'a, T: Select> Toggle<'a, T> {
    /// Sets the transition time for the toggle.
    pub fn transition<D: Into<Duration>>(&self, duration: D) -> Request<'_, Duration> {
        Request {
            client: self.parent.client,
            path: format!("/lights/{}/toggle", self.parent.selector),
            body: duration.into(),
            method: Method::POST,
        }
    }
}

impl<'a, T: Select> From<Toggle<'a, T>> for Request<'a, ()> {
    fn from(toggle: Toggle<'a, T>) -> Self {
        Self {
            client: toggle.parent.client,
            path: format!("/lights/{}/toggle", toggle.parent.selector),
            body: (),
            method: Method::POST,
        }
    }
}

/// A scoped request to uniformly set the state for all selected bulbs.
pub struct SetState<'a, T: Select> {
    pub(crate) parent: &'a Selected<'a, T>,
    pub(crate) new: State,
}

impl<'a, T: Select> SetState<'a, T> {
    /// Sets the power state of all selected bulbs.
    pub fn power<P: Into<Power>>(&'a mut self, on: P) -> &'a mut SetState<'a, T> {
        self.new.power = Some(on.into());
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
    pub fn transition<D: Into<Duration>>(&'a mut self, duration: D) -> &'a mut SetState<'a, T> {
        self.new.duration = Some(duration.into());
        self
    }
    /// Sets the infrared level, if applicable.
    pub fn infrared(&'a mut self, ir: f32) -> &'a mut SetState<'a, T> {
        self.new.infrared = Some(ir);
        self
    }
}

impl<'a, T: Select> From<&SetState<'a, T>> for Request<'a, State> {
    fn from(state: &SetState<'a, T>) -> Self {
        Self {
            client: state.parent.client,
            path: format!("/lights/{}/state", state.parent.selector),
            body: state.new.clone(),
            method: Method::PUT,
        }
    }
}

#[derive(Clone, Serialize)]
pub(crate) struct StateExt {
    pub(crate) selector: String,
    #[serde(flatten)]
    pub(crate) state: State,
}

/// A scoped request to uniformly set the state for all selected bulbs.
#[derive(Clone, Serialize)]
pub struct SetStates<'a> {
    #[serde(skip)]
    pub(crate) parent: &'a Client,
    #[serde(rename = "states", skip_serializing_if = "Vec::is_empty")]
    pub(crate) new: Vec<StateExt>,
    #[serde(rename = "defaults", skip_serializing_if = "Option::is_none")]
    pub(crate) default: Option<State>,
}

impl<'a> SetStates<'a> {
    /// Adds the given state to the list.
    pub fn add<T: Select>(&mut self, selector: T, state: State) -> &'_ mut Self {
        self.new.push(StateExt {
            selector: format!("{}", selector),
            state,
        });
        self
    }
    /// Sets the default properties to use if left unspecified.
    pub fn default(&mut self, state: State) -> &'_ mut Self {
        self.default = Some(state);
        self
    }
}

impl<'a, 'b: 'a> From<&'b SetStates<'a>> for Request<'a, &'b SetStates<'a>> {
    fn from(states: &'b SetStates<'a>) -> Self {
        Self {
            client: states.parent,
            path: "/lights/states".to_string(),
            body: states,
            method: Method::PUT,
        }
    }
}

/// A scoped request to uniformly change the state for all selected bulbs.
pub struct ChangeState<'a, T: Select> {
    pub(crate) parent: &'a Selected<'a, T>,
    pub(crate) change: StateChange,
}

impl<'a, T: Select> ChangeState<'a, T> {
    /// Sets target power state.
    pub fn power<P: Into<Power>>(&'a mut self, on: P) -> &'a mut Self {
        self.change.power = Some(on.into());
        self
    }
    /// Sets transition duration.
    pub fn transition<D: Into<Duration>>(&'a mut self, duration: D) -> &'a mut Self {
        self.change.duration = Some(duration.into());
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
}

impl<'a, T: Select> From<&ChangeState<'a, T>> for Request<'a, StateChange> {
    fn from(delta: &ChangeState<'a, T>) -> Self {
        Self {
            client: delta.parent.client,
            path: format!("/lights/{}/state/delta", delta.parent.selector),
            body: delta.change.clone(),
            method: Method::POST,
        }
    }
}
