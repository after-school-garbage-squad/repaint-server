use std::fmt::{self, Display, Formatter};
use std::marker::PhantomData;
use std::str::FromStr;

use derivative::Derivative;
pub use ulid::DecodeError as Error;
use ulid::Ulid;

#[derive(Derivative)]
#[derivative(
    Clone(bound = ""),
    Copy(bound = ""),
    Debug(bound = ""),
    PartialEq(bound = ""),
    Eq(bound = ""),
    Hash(bound = "")
)]
pub struct Id<T> {
    id: Ulid,

    #[derivative(Debug = "ignore")]
    _phantom: PhantomData<fn() -> T>,
}

impl<T> Default for Id<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Id<T> {
    pub fn new() -> Self {
        Self {
            id: Ulid::new(),
            _phantom: PhantomData,
        }
    }
}

impl<T> Display for Id<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.id.fmt(f)
    }
}

impl<T> FromStr for Id<T> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            id: s.parse()?,
            _phantom: PhantomData,
        })
    }
}

impl<T> serde::ser::Serialize for Id<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.id.serialize(serializer)
    }
}

impl<'de, T> serde::de::Deserialize<'de> for Id<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Id {
            id: Ulid::deserialize(deserializer)?,
            _phantom: PhantomData,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_bounds() {
        struct Hoge {}
        fn test<T: Clone + std::fmt::Debug + Eq + Default>() {}
        test::<Id<Hoge>>();
    }

    #[test]
    fn test_serialize() {
        let id: Id<()> = Id {
            id: "01FVSHW3S537KKHBRMSA418ATB".parse().unwrap(),
            _phantom: PhantomData,
        };

        assert_eq!(
            serde_json::to_string(&id).unwrap(),
            "\"01FVSHW3S537KKHBRMSA418ATB\""
        );

        assert_eq!(
            serde_json::from_str::<Id<()>>("\"01FVSHW3S537KKHBRMSA418ATB\"").unwrap(),
            id,
        );
    }
}
