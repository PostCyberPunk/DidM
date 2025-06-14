use serde::de::value;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::default::Default;
use std::fmt;

struct OptionField<T> {
    value: Option<T>,
    default: Option<T>,
}

impl<T: Default> OptionField<T> {
    fn new(default: Option<T>) -> Self {
        OptionField {
            value: None,
            default,
        }
    }
    fn get(&self) -> Option<&T> {
        self.value.as_ref().or(self.default.as_ref())
    }
    fn take(self) -> T {
        match self.value {
            Some(value) => value,
            None => self.default.unwrap_or_default(),
        }
    }
    fn set(&mut self, value: T) {
        self.value = Some(value);
    }
}

impl<T: Default> Default for OptionField<T> {
    fn default() -> Self {
        OptionField::new(Some(T::default()))
    }
}

impl<T: Serialize> Serialize for OptionField<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match &self.value {
            Some(value) => serializer.serialize_some(value),
            None => serializer.serialize_none(),
        }
    }
}
impl<'de, T: Deserialize<'de>> Deserialize<'de> for OptionField<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: Option<T> = Option::deserialize(deserializer)?;
        Ok(OptionField {
            value,
            default: None,
        })
    }
}

impl<T: PartialEq> PartialEq for OptionField<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}
impl<T: PartialEq> Eq for OptionField<T> {}

impl<T: fmt::Debug> fmt::Debug for OptionField<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("OptionField")
            .field("value", &self.value)
            .finish()
    }
}
