use super::*;
use std::collections::{BTreeMap, BTreeSet, HashMap};

impl<T: TransitSerialize + ?Sized> TransitSerialize for Box<T> {
    const TF_TYPE: TransitType = T::TF_TYPE;

    fn transit_serialize<S: TransitSerializer>(&self, serializer: S) -> S::Output {
        (**self).transit_serialize(serializer)
    }

    fn transit_serialize_key<S: TransitSerializer>(&self, serializer: S) -> Option<S::Output> {
        (**self).transit_serialize_key(serializer)
    }
}

impl<'a, T: TransitSerialize + ?Sized> TransitSerialize for &'a T {
    const TF_TYPE: TransitType = T::TF_TYPE;

    fn transit_serialize<S: TransitSerializer>(&self, serializer: S) -> S::Output {
        (**self).transit_serialize(serializer)
    }

    fn transit_serialize_key<S: TransitSerializer>(&self, serializer: S) -> Option<S::Output> {
        (**self).transit_serialize_key(serializer)
    }
}

impl TransitSerialize for bool {
    const TF_TYPE: TransitType = TransitType::Scalar;

    fn transit_serialize<S: TransitSerializer>(&self, serializer: S) -> S::Output {
        serializer.serialize_bool(*self)
    }

    fn transit_serialize_key<S: TransitSerializer>(&self, serializer: S) -> Option<S::Output> {
        let s = if *self {
            "~?t".to_owned()
        } else {
            "~?f".to_owned()
        };

        Some(serializer.serialize_string(&s))
    }
}

impl<K: TransitSerialize, V: TransitSerialize> TransitSerialize for BTreeMap<K, V> {
    const TF_TYPE: TransitType = TransitType::Composite;

    fn transit_serialize<S: TransitSerializer>(&self, serializer: S) -> S::Output {
        let mut ser_map = serializer.serialize_map(Some(self.len()));
        for (k, v) in self.iter() {
            ser_map.serialize_pair((*k).clone(), (*v).clone());
        }
        ser_map.end()
    }

    fn transit_serialize_key<S: TransitSerializer>(&self, _serializer: S) -> Option<S::Output> {
        None
    }
}

impl<K: TransitSerialize, V: TransitSerialize> TransitSerialize for HashMap<K, V> {
    const TF_TYPE: TransitType = TransitType::Composite;

    fn transit_serialize<S: TransitSerializer>(&self, serializer: S) -> S::Output {
        let mut ser_map = serializer.serialize_map(Some(self.len()));
        for (k, v) in self.iter() {
            ser_map.serialize_pair((*k).clone(), (*v).clone());
        }
        ser_map.end()
    }

    fn transit_serialize_key<S: TransitSerializer>(&self, _serializer: S) -> Option<S::Output> {
        None
    }
}

impl<T: TransitSerialize> TransitSerialize for Vec<T> {
    const TF_TYPE: TransitType = TransitType::Composite;

    fn transit_serialize<S: TransitSerializer>(&self, serializer: S) -> S::Output {
        let mut ser_arr = serializer.serialize_array(Some(self.len()));
        for v in self.iter() {
            ser_arr.serialize_item((*v).clone());
        }
        ser_arr.end()
    }

    fn transit_serialize_key<S: TransitSerializer>(&self, _serializer: S) -> Option<S::Output> {
        None
    }
}

impl<T: TransitSerialize> TransitSerialize for BTreeSet<T> {
    const TF_TYPE: TransitType = TransitType::Composite;

    fn transit_serialize<S: TransitSerializer>(&self, serializer: S) -> S::Output {
        let mut ser_arr = serializer
            .clone()
            .serialize_tagged_array("~#set", Some(self.len()));
        for v in self.iter() {
            ser_arr.serialize_item((*v).clone());
        }
        ser_arr.end()
    }

    fn transit_serialize_key<S: TransitSerializer>(&self, _serializer: S) -> Option<S::Output> {
        None
    }
}

impl TransitSerialize for i32 {
    const TF_TYPE: TransitType = TransitType::Scalar;

    fn transit_serialize<S: TransitSerializer>(&self, serializer: S) -> S::Output {
        serializer.serialize_int((*self).into())
    }

    fn transit_serialize_key<S: TransitSerializer>(&self, serializer: S) -> Option<S::Output> {
        Some(serializer.serialize_string(&format!("~i{}", self)))
    }
}

impl TransitSerialize for String {
    const TF_TYPE: TransitType = TransitType::Scalar;

    fn transit_serialize<S: TransitSerializer>(&self, serializer: S) -> S::Output {
        serializer.serialize_string(self)
    }

    fn transit_serialize_key<S: TransitSerializer>(&self, serializer: S) -> Option<S::Output> {
        Some(serializer.serialize_string(self))
    }
}

impl TransitSerialize for &str {
    const TF_TYPE: TransitType = TransitType::Scalar;

    fn transit_serialize<S: TransitSerializer>(&self, serializer: S) -> S::Output {
        serializer.serialize_string(self)
    }

    fn transit_serialize_key<S: TransitSerializer>(&self, serializer: S) -> Option<S::Output> {
        Some(serializer.serialize_string(self))
    }
}

impl<T: TransitSerialize> TransitSerialize for Option<T> {
    const TF_TYPE: TransitType = T::TF_TYPE;

    fn transit_serialize<S: TransitSerializer>(&self, serializer: S) -> S::Output {
        if let Some(x) = self {
            x.transit_serialize(serializer)
        } else {
            serializer.serialize_null()
        }
    }

    fn transit_serialize_key<S: TransitSerializer>(&self, serializer: S) -> Option<S::Output> {
        Some(serializer.serialize_string("~_"))
    }
}
