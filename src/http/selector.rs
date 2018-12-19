use std::fmt;
use std::iter::FromIterator;

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
#[derive(Clone, Eq, PartialEq)]
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

#[doc(hidden)]
/// Represents a set of zones. Used to constrain selectors further.
pub struct Zones {
    list: Vec<u8>,
}

impl From<Vec<u8>> for Zones {
    fn from(list: Vec<u8>) -> Zones {
        Zones { list }
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

impl From<::std::ops::Range<u8>> for Zones {
    fn from(from: ::std::ops::Range<u8>) -> Zones {
        from.collect::<Vec<_>>().into()
    }
}

impl From<::std::ops::RangeInclusive<u8>> for Zones {
    fn from(from: ::std::ops::RangeInclusive<u8>) -> Zones {
        from.collect::<Vec<_>>().into()
    }
}

impl From<::std::ops::RangeFrom<u8>> for Zones {
    fn from(from: ::std::ops::RangeFrom<u8>) -> Zones {
        (from.start..=255).collect::<Vec<_>>().into()
    }
}

impl From<::std::ops::RangeTo<u8>> for Zones {
    fn from(from: ::std::ops::RangeTo<u8>) -> Zones {
        (0..from.end).collect::<Vec<_>>().into()
    }
}

impl From<::std::ops::RangeToInclusive<u8>> for Zones {
    fn from(from: ::std::ops::RangeToInclusive<u8>) -> Zones {
        (0..=from.end).collect::<Vec<_>>().into()
    }
}

impl From<u8> for Zones {
    fn from(from: u8) -> Zones {
        Zones { list: vec![from] }
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
    fn random(self) -> Random<T> {
        Random(self)
    }
}
