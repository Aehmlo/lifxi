use std::fmt;
use std::iter::FromIterator;
use std::str::FromStr;

use serde::{de::Error as DeError, Deserialize, Deserializer, Serialize, Serializer};

/// Selectors are used to identify one or more lights belonging to a particular account.
///
/// All resolutions of selectors are treated as sets, even if they are logically a single device,
/// for consistency with the API (and to help prevent breakage).
///
/// ### Constraining Selectors
///
/// Selectors may be constrained to specific zones with
/// [the `zoned` method on `Selector`](#method.zoned).
///
/// ### Randomization
///
/// A random device can be chosen from the list of devices matching a selector via
/// [the `Randomize` trait](trait.Randomize.html).
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Selector {
    /// All devices on the user's account.
    All,
    /// Specifies a device with the given label.
    Label(String),
    /// Specifies a device with the given ID/serial number.
    Id(String),
    /// Specifies a collection of devices based on the given group ID.
    ///
    /// Groups are discrete objects with their own IDs; use this to specify a group by ID instead
    /// of by label.
    GroupId(String),
    /// Specifies a collection of devices based on a group with the given label.
    Group(String),
    /// Specifies a collection of devices based on a location with the given ID.
    ///
    /// Like groups, locations have their own IDs, so this option exists for locating devices by
    /// location ID instead of label.
    LocationId(String),
    /// Specifies a collection of devices based on a location with the given label.
    Location(String),
    /// Specifies a collection of devices from a scene with the given ID.
    SceneId(String),
}

impl fmt::Display for Selector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Selector::*;
        match self {
            All => write!(f, "all"),
            Label(label) => write!(f, "label:{}", label),
            Id(id) => write!(f, "id:{}", id),
            GroupId(id) => write!(f, "group_id:{}", id),
            Group(label) => write!(f, "group:{}", label),
            LocationId(id) => write!(f, "location_id:{}", id),
            Location(label) => write!(f, "location:{}", label),
            SceneId(id) => write!(f, "scene_id:{}", id),
        }
    }
}

/// Represents a selector deserialization error.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SelectorParseError {
    /// The selector was neither "all" nor prefixed with a label.
    NoLabel,
    /// The selector contained a label but no following value.
    NoValue,
    /// The selector contained an unknown label.
    UnknownLabel,
}

impl fmt::Display for SelectorParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SelectorParseError::NoLabel => "Unrecognized selector.",
                SelectorParseError::NoValue => "No value given for label.",
                SelectorParseError::UnknownLabel => "Unrecognized label.",
            }
        )
    }
}

impl FromStr for Selector {
    type Err = self::SelectorParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use self::Selector::*;
        use self::SelectorParseError::*;
        match s {
            "all" => Ok(All),
            x => {
                let mut parts = x.split(':');
                if let Some(label) = parts.next() {
                    if let Some(value) = parts.next().map(|p| p.trim().to_string()) {
                        match label {
                            "label" => Ok(Label(value)),
                            "id" => Ok(Id(value)),
                            "group_id" => Ok(GroupId(value)),
                            "group" => Ok(Group(value)),
                            "location_id" => Ok(LocationId(value)),
                            "location" => Ok(Location(value)),
                            "scene_id" => Ok(SceneId(value)),
                            _ => Err(UnknownLabel),
                        }
                    } else {
                        Err(NoValue)
                    }
                } else {
                    Err(NoLabel)
                }
            }
        }
    }
}

impl<'de> Deserialize<'de> for Selector {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<(Self), D::Error> {
        let s = String::deserialize(deserializer)?;
        s.parse::<Self>().map_err(DeError::custom)
    }
}

impl Serialize for Selector {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&format!("{}", self))
    }
}

#[doc(hidden)]
/// A selector that has been constrained to specific zones.
pub struct Zoned {
    selector: Selector,
    zoning: Zones,
}

impl fmt::Display for Zoned {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.selector)?;
        for z in &self.zoning.list {
            write!(f, "|{}", z)?;
        }
        Ok(())
    }
}

impl Serialize for Zoned {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&format!("{}", self))
    }
}

#[doc(hidden)]
/// Represents a set of zones. Used to constrain selectors further.
pub struct Zones {
    list: Vec<u8>,
}

impl From<Vec<u8>> for Zones {
    fn from(list: Vec<u8>) -> Self {
        Self { list }
    }
}

impl FromIterator<u8> for Zones {
    fn from_iter<I: IntoIterator<Item = u8>>(iter: I) -> Self {
        let mut list = Vec::new();
        for item in iter {
            list.push(item);
        }
        Self { list }
    }
}

impl<'a> FromIterator<&'a u8> for Zones {
    fn from_iter<I: IntoIterator<Item = &'a u8>>(iter: I) -> Self {
        Self::from_iter(iter.into_iter().cloned())
    }
}

impl From<::std::ops::Range<u8>> for Zones {
    fn from(from: ::std::ops::Range<u8>) -> Self {
        from.collect::<Vec<_>>().into()
    }
}

impl From<::std::ops::RangeInclusive<u8>> for Zones {
    fn from(from: ::std::ops::RangeInclusive<u8>) -> Self {
        from.collect::<Vec<_>>().into()
    }
}

impl From<::std::ops::RangeFrom<u8>> for Zones {
    fn from(from: ::std::ops::RangeFrom<u8>) -> Self {
        (from.start..=255).collect::<Vec<_>>().into()
    }
}

impl From<::std::ops::RangeTo<u8>> for Zones {
    fn from(from: ::std::ops::RangeTo<u8>) -> Self {
        (0..from.end).collect::<Vec<_>>().into()
    }
}

impl From<::std::ops::RangeToInclusive<u8>> for Zones {
    fn from(from: ::std::ops::RangeToInclusive<u8>) -> Self {
        (0..=from.end).collect::<Vec<_>>().into()
    }
}

impl From<u8> for Zones {
    fn from(from: u8) -> Self {
        Self { list: vec![from] }
    }
}

#[doc(hidden)]
/// A selector that randomly chooses a device from the resultant list.
pub struct Random<T: PureSelect>(T);

impl<T: PureSelect> fmt::Display for Random<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:random", self.0)
    }
}

impl<T: PureSelect> Serialize for Random<T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&format!("{}", self))
    }
}

impl Selector {
    /// Constrains the selector to only match the given zone(s).
    /// ### Examples
    /// ```
    /// use lifx::http::Selector;
    /// // Devices in the "Living Room" group in zones 0 or 1 (ignores devices from other zones).
    /// let sel = Selector::Group("Living Room".to_string()).zoned(0..2);
    /// assert_eq!(&format!("{}", sel), "group:Living Room|0|1");
    /// // Zones 254 or 255 only.
    /// let sel = Selector::Group("Living Room".to_string()).zoned(254..);
    /// assert_eq!(&format!("{}", sel), "group:Living Room|254|255");
    /// // Zones 3 or 4 only.
    /// let sel = Selector::Group("Living Room".to_string()).zoned(3..=4);
    /// assert_eq!(&format!("{}", sel), "group:Living Room|3|4");
    /// // Zone 1 only.
    /// let sel = Selector::Group("Living Room".to_string()).zoned(1);
    /// assert_eq!(&format!("{}", sel), "group:Living Room|1");
    /// // Zones 1 or 255 only.
    /// let sel = Selector::Group("Living Room".to_string()).zoned(vec![1, 255]);
    /// assert_eq!(&format!("{}", sel), "group:Living Room|1|255");
    /// ```
    pub fn zoned<T>(self, z: T) -> Zoned
    where
        T: Into<Zones>,
    {
        Zoned {
            selector: self,
            zoning: z.into(),
        }
    }
}

/// Marker trait indicating the potential for use in identifying devices.
pub trait Select: fmt::Display {}
impl Select for Selector {}
impl Select for Zoned {}
impl<T: PureSelect> Select for Random<T> {}

/// Marker trait for non-randomized selectors.
#[doc(hidden)]
pub trait PureSelect: Select {}
impl PureSelect for Selector {}
impl PureSelect for Zoned {}

/// Enables randomization of non-randomized selectors.
pub trait Randomize<T: PureSelect> {
    /// Chooses a random element from the set of matching devices.
    fn random(self) -> Random<T>;
}

impl<T> Randomize<T> for T
where
    T: PureSelect,
{
    /// Creates a selector which will choose a random device from the list.
    ///
    /// ### Examples
    /// ```
    /// use lifx::http::*;
    /// let sel = Selector::Group("Living Room".to_string()).random();
    /// assert_eq!(&format!("{}", sel), "group:Living Room:random");
    /// let sel = Selector::Group("Living Room".to_string()).zoned(1).random();
    /// assert_eq!(&format!("{}", sel), "group:Living Room|1:random");
    /// ```
    fn random(self) -> Random<Self> {
        Random(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn serialize() {
        let selector = Selector::All;
        assert_eq!(&format!("{}", selector), "all");
        let selector = Selector::Label("Living Room".to_string());
        assert_eq!(&format!("{}", selector), "label:Living Room");
        let selector = Selector::Id("abcd".to_string());
        assert_eq!(&format!("{}", selector), "id:abcd");
        let selector = Selector::GroupId("efgh".to_string());
        assert_eq!(&format!("{}", selector), "group_id:efgh");
        let selector = Selector::Group("Lounge".to_string());
        assert_eq!(&format!("{}", selector), "group:Lounge");
        let selector = Selector::LocationId("ijkl".to_string());
        assert_eq!(&format!("{}", selector), "location_id:ijkl");
        let selector = Selector::Location("Summer Home".to_string());
        assert_eq!(&format!("{}", selector), "location:Summer Home");
        let selector = Selector::SceneId("mnop".to_string());
        assert_eq!(&format!("{}", selector), "scene_id:mnop");
        let selector = Selector::All.zoned(17);
        assert_eq!(&format!("{}", selector), "all|17");
        let selector = Selector::All.zoned(255..);
        assert_eq!(&format!("{}", selector), "all|255");
        let selector = Selector::All.zoned(18..19);
        assert_eq!(&format!("{}", selector), "all|18");
        let selector = Selector::All.zoned(3..=5);
        assert_eq!(&format!("{}", selector), "all|3|4|5");
        let selector = Selector::All.zoned(..=2);
        assert_eq!(&format!("{}", selector), "all|0|1|2");
        let selector = Selector::All.random();
        assert_eq!(&format!("{}", selector), "all:random");
        let zones = vec![1, 2];
        let selector = Selector::All.zoned(zones.iter().collect::<Zones>());
        assert_eq!(&format!("{}", selector), "all|1|2");
        let selector = Selector::All.zoned(zones);
        assert_eq!(&format!("{}", selector), "all|1|2");
    }
    #[test]
    fn deserialize() {
        let selector = "all".parse();
        assert_eq!(selector, Ok(Selector::All));
        let selector = "label:Living Room".parse();
        assert_eq!(selector, Ok(Selector::Label("Living Room".to_string())));
        let selector = "id:abcd".parse();
        assert_eq!(selector, Ok(Selector::Id("abcd".to_string())));
        let selector = "group_id:efgh".parse();
        assert_eq!(selector, Ok(Selector::GroupId("efgh".to_string())));
        let selector = "group:Lounge".parse();
        assert_eq!(selector, Ok(Selector::Group("Lounge".to_string())));
        let selector = "location:ijkl".parse();
        assert_eq!(selector, Ok(Selector::Location("ijkl".to_string())));
        let selector = "location:Summer Home".parse();
        assert_eq!(selector, Ok(Selector::Location("Summer Home".to_string())));
        let selector = "scene_id:mnop".parse();
        assert_eq!(selector, Ok(Selector::SceneId("mnop".to_string())));
    }
}
