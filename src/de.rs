use lazy_static::lazy_static;
use regex::Regex;
use serde_json::{
    map::{IntoIter as JsMapIntoIter, Map as JsMap},
    Value as JsVal,
};
use std::collections::BTreeMap;
use std::convert::TryFrom;
use std::fmt::Debug;

#[derive(Debug)]
pub enum Error {
    DoNotMatch(String),
    ItWontFit(String),
    CannotBeKey(&'static str),
}

type TResult<T> = Result<T, Error>;

#[derive(PartialEq)]
pub enum TransitType {
    Scalar,
    Composite,
}

pub trait TransitDeserialize: Sized {
    const TF_TYPE: TransitType;

    fn transit_deserialize<D: TransitDeserializer>(
        deserializer: D,
        input: D::Input,
    ) -> TResult<Self>;

    fn transit_deserialize_key<D: TransitDeserializer>(
        deserializer: D,
        input: D::Input,
    ) -> TResult<Self>;
}

pub trait TransitDeserializer: Clone + Debug {
    type Input: Debug + Clone;
    type DeserializeArray: IntoIterator<Item = Self::Input>;
    type DeserializeMap: IntoIterator<Item = (Self::Input, Self::Input)>;

    //fn deserialize_null(self, v: Self::Input) -> Self::Output;
    fn deserialize_string(self, v: Self::Input) -> TResult<String>;
    fn deserialize_bool(self, v: Self::Input) -> TResult<bool>;
    fn deserialize_int(self, v: Self::Input) -> TResult<i64>;
    fn deserialize_float(self, v: Self::Input) -> TResult<f64>;
    fn deserialize_array(self, v: Self::Input) -> TResult<Self::DeserializeArray>;
    fn deserialize_map(self, v: Self::Input) -> TResult<Self::DeserializeMap>;
}

struct JsonObjectIntoIter {
    js_iter: JsMapIntoIter,
}

impl Iterator for JsonObjectIntoIter {
    type Item = (JsVal, JsVal);

    fn next(&mut self) -> Option<Self::Item> {
        self.js_iter.next().map(|(k, v)| (JsVal::String(k), v))
    }
}

#[derive(Clone, Debug)]
struct JsonDeserializer;

impl TransitDeserializer for JsonDeserializer {
    type Input = JsVal;
    type DeserializeArray = std::vec::IntoIter<JsVal>;
    type DeserializeMap = JsonObjectIntoIter;

    fn deserialize_string(self, v: Self::Input) -> TResult<String> {
        v.as_str()
            .map(|x| x.to_owned())
            .ok_or(Error::DoNotMatch(format!("{} is not string", v)))
    }

    fn deserialize_bool(self, v: Self::Input) -> TResult<bool> {
        v.as_bool()
            .ok_or(Error::DoNotMatch(format!("{} is not bool", v)))
    }

    fn deserialize_int(self, v: Self::Input) -> TResult<i64> {
        v.as_i64()
            .ok_or(Error::DoNotMatch(format!("{} is not int", v)))
    }

    fn deserialize_float(self, v: Self::Input) -> TResult<f64> {
        v.as_f64()
            .ok_or(Error::DoNotMatch(format!("{} is not float", v)))
    }

    fn deserialize_array(self, v: Self::Input) -> TResult<Self::DeserializeArray> {
        v.as_array()
            .map(|x| x.clone().into_iter())
            .ok_or(Error::DoNotMatch(format!("{} is not an array", v)))
    }

    fn deserialize_map(self, v: Self::Input) -> TResult<Self::DeserializeMap> {
        v.as_object()
            .map(|x| JsonObjectIntoIter {
                js_iter: x.clone().into_iter(),
            })
            .ok_or(Error::DoNotMatch(format!("{} is not a map", v)))
    }
}

impl<K, V> TransitDeserialize for BTreeMap<K, V>
where
    K: TransitDeserialize + Ord,
    V: TransitDeserialize,
{
    const TF_TYPE: TransitType = TransitType::Composite;

    fn transit_deserialize<D: TransitDeserializer>(
        deserializer: D,
        input: D::Input,
    ) -> TResult<Self> {
        let map_iter = deserializer.clone().deserialize_map(input)?;
        let mut result: Self = BTreeMap::new();
        match K::TF_TYPE {
            TransitType::Scalar => {
                for (k, v) in map_iter {
                    result.insert(
                        TransitDeserialize::transit_deserialize_key(deserializer.clone(), k)?,
                        TransitDeserialize::transit_deserialize(deserializer.clone(), v)?,
                    );
                }
                Ok(result)
            }
            TransitType::Composite => {
                if let Some((k, v)) = map_iter.into_iter().next() {
                    let k_str = deserializer.clone().deserialize_string(k)?;
                    if k_str == "~#cmap" {
                        Ok(())
                    } else {
                        Err(Error::DoNotMatch(format!("{:?} must be ~#cmap", k_str)))
                    }?;
                    let vals = deserializer.clone().deserialize_array(v)?;
                    for c in vals.into_iter().collect::<Vec<D::Input>>().chunks(2) {
                        result.insert(
                            TransitDeserialize::transit_deserialize(
                                deserializer.clone(),
                                c[0].clone(),
                            )?,
                            TransitDeserialize::transit_deserialize(
                                deserializer.clone(),
                                c[1].clone(),
                            )?,
                        );
                    }
                }
                Ok(result)
            }
        }
    }

    fn transit_deserialize_key<D: TransitDeserializer>(
        deserializer: D,
        input: D::Input,
    ) -> TResult<Self> {
        Err(Error::CannotBeKey(
            "BTreeMap<K, V> cannot be deserialized as key",
        ))
    }
}

impl<T: TransitDeserialize> TransitDeserialize for Vec<T> {
    const TF_TYPE: TransitType = TransitType::Composite;

    fn transit_deserialize<D: TransitDeserializer>(
        deserializer: D,
        input: D::Input,
    ) -> TResult<Self> {
        let array_iter = deserializer.clone().deserialize_array(input)?;
        let mut v = Vec::new();
        for x in array_iter {
            v.push(TransitDeserialize::transit_deserialize(
                deserializer.clone(),
                x,
            )?);
        }
        Ok(v)
    }

    fn transit_deserialize_key<D: TransitDeserializer>(
        deserializer: D,
        input: D::Input,
    ) -> TResult<Self> {
        Err(Error::CannotBeKey("Vec<T> cannot be deserialized as key"))
    }
}

impl TransitDeserialize for bool {
    const TF_TYPE: TransitType = TransitType::Scalar;

    fn transit_deserialize<D: TransitDeserializer>(
        deserializer: D,
        input: D::Input,
    ) -> TResult<Self> {
        deserializer.deserialize_bool(input)
    }

    fn transit_deserialize_key<D: TransitDeserializer>(
        deserializer: D,
        input: D::Input,
    ) -> TResult<Self> {
        let s = deserializer.deserialize_string(input.clone())?;
        match s.as_ref() {
            "~?t" => Ok(true),
            "~?f" => Ok(false),
            _ => Err(Error::DoNotMatch(format!("{} is wrong bool key", s))),
        }
    }
}

impl TransitDeserialize for i32 {
    const TF_TYPE: TransitType = TransitType::Scalar;

    fn transit_deserialize<D: TransitDeserializer>(
        deserializer: D,
        input: D::Input,
    ) -> TResult<Self> {
        Self::try_from(deserializer.deserialize_int(input)?)
            .map_err(|_| Error::ItWontFit(format!("Cannot fit in i32")))
    }

    fn transit_deserialize_key<D: TransitDeserializer>(
        deserializer: D,
        input: D::Input,
    ) -> TResult<Self> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"~i(?P<int>-?\d+)").unwrap();
        }
        let s = deserializer.deserialize_string(input.clone())?;
        RE.captures(&s)
            .ok_or(Error::DoNotMatch(format!(
                "{:?} is not proper i32 key",
                input
            )))
            .and_then(|cap| {
                cap.name("int")
                    .ok_or(Error::DoNotMatch(format!(
                        "{:?} is not proper i32 key",
                        input
                    )))
                    .and_then(|i| {
                        (i.as_str())
                            .parse::<Self>()
                            .map_err(|_| Error::DoNotMatch(format!("{:?} is not i32", input)))
                    })
            })
    }
}

impl TransitDeserialize for String {
    const TF_TYPE: TransitType = TransitType::Scalar;

    fn transit_deserialize<D: TransitDeserializer>(
        deserializer: D,
        input: D::Input,
    ) -> TResult<Self> {
        deserializer.deserialize_string(input).map(|x| x.to_owned())
    }

    fn transit_deserialize_key<D: TransitDeserializer>(
        deserializer: D,
        input: D::Input,
    ) -> TResult<Self> {
        deserializer.deserialize_string(input).map(|x| x.to_owned())
    }
}

pub fn from_transit_json<T: TransitDeserialize>(v: JsVal) -> TResult<T> {
    TransitDeserialize::transit_deserialize(JsonDeserializer, v)
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json::json;
    use std::collections::BTreeMap;

    #[test]
    fn dumb_array() {
        let v = vec![1, 2, 3];
        let tr: Vec<i32> = from_transit_json(json!([1, 2, 3])).unwrap();
        assert_eq!(v, tr);
    }

    // TODO: Quoting
    // Check that something like 5 cannot be parsed on top level

    #[test]
    fn scalar_map_btree() {
        let mut m = BTreeMap::new();
        m.insert(4, "yolo".to_owned());
        m.insert(-6, "swag".to_owned());

        let tr = from_transit_json(json!({
            "~i4": "yolo",
            "~i-6": "swag"
        }))
        .unwrap();
        assert_eq!(m, tr);
    }

    #[test]
    fn map_composite_keys() {
        let mut key1: BTreeMap<bool, String> = BTreeMap::new();
        key1.insert(true, "test".to_owned());
        key1.insert(false, "tset".to_owned());

        let mut m = BTreeMap::new();
        m.insert(key1, 1337);

        let tr: BTreeMap<BTreeMap<bool, String>, i32> = from_transit_json(json!({
            "~#cmap": [
                {
                    "~?t": "test",
                    "~?f": "tset",
                },
                1337
            ],
        }))
        .unwrap();
        assert_eq!(tr, m);
    }
}
