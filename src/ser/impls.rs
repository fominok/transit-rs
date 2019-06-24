use super::*;
use chrono::{DateTime, Utc};
use std::collections::{BTreeMap, BTreeSet, HashMap};

impl<T: TransitSerialize + ?Sized> TransitSerialize for Box<T> {
    fn transit_serialize<S: TransitSerializer>(&self, serializer: &S) -> S::Output {
        (**self).transit_serialize(serializer)
    }

    fn transit_serialize_key<KS: TransitKeySerializer>(
        &self,
        serializer: &KS,
    ) -> Option<KS::Output> {
        (**self).transit_serialize_key(serializer)
    }
}

impl<'a, T: TransitSerialize + ?Sized> TransitSerialize for &'a T {
    fn transit_serialize<S: TransitSerializer>(&self, serializer: &S) -> S::Output {
        (**self).transit_serialize(serializer)
    }

    fn transit_serialize_key<KS: TransitKeySerializer>(
        &self,
        serializer: &KS,
    ) -> Option<KS::Output> {
        (**self).transit_serialize_key(serializer)
    }
}

impl TransitSerialize for bool {
    fn transit_serialize<S: TransitSerializer>(&self, serializer: &S) -> S::Output {
        serializer.serialize_bool(*self)
    }

    fn transit_serialize_key<KS: TransitKeySerializer>(
        &self,
        serializer: &KS,
    ) -> Option<KS::Output> {
        let s = if *self {
            "~?t".to_owned()
        } else {
            "~?f".to_owned()
        };

        Some(serializer.serialize_key(&s))
    }
}

impl<K: TransitSerialize, V: TransitSerialize> TransitSerialize for BTreeMap<K, V> {
    fn transit_serialize<S: TransitSerializer>(&self, serializer: &S) -> S::Output {
        serializer.serialize_map_iter(self.iter())
    }

    fn transit_serialize_key<KS: TransitKeySerializer>(
        &self,
        _serializer: &KS,
    ) -> Option<KS::Output> {
        None
    }
}

impl<K: TransitSerialize, V: TransitSerialize> TransitSerialize for HashMap<K, V> {
    fn transit_serialize<S: TransitSerializer>(&self, serializer: &S) -> S::Output {
        serializer.serialize_map_iter(self.iter())
    }

    fn transit_serialize_key<KS: TransitKeySerializer>(
        &self,
        _serializer: &KS,
    ) -> Option<KS::Output> {
        None
    }
}

impl<T: TransitSerialize> TransitSerialize for Vec<T> {
    fn transit_serialize<S: TransitSerializer>(&self, serializer: &S) -> S::Output {
        serializer.serialize_array_iter(self.iter())
    }

    fn transit_serialize_key<KS: TransitKeySerializer>(
        &self,
        _serializer: &KS,
    ) -> Option<KS::Output> {
        None
    }
}

impl<T: TransitSerialize> TransitSerialize for BTreeSet<T> {
    fn transit_serialize<S: TransitSerializer>(&self, serializer: &S) -> S::Output {
        serializer.serialize_tagged_array_iter("~#set", self.iter())
    }

    fn transit_serialize_key<KS: TransitKeySerializer>(
        &self,
        _serializer: &KS,
    ) -> Option<KS::Output> {
        None
    }
}

impl TransitSerialize for i32 {
    fn transit_serialize<S: TransitSerializer>(&self, serializer: &S) -> S::Output {
        serializer.serialize_int((*self).into())
    }

    fn transit_serialize_key<KS: TransitKeySerializer>(
        &self,
        serializer: &KS,
    ) -> Option<KS::Output> {
        Some(serializer.serialize_key(&format!("~i{}", self)))
    }
}

impl TransitSerialize for String {
    fn transit_serialize<S: TransitSerializer>(&self, serializer: &S) -> S::Output {
        serializer.serialize_string(self)
    }

    fn transit_serialize_key<KS: TransitKeySerializer>(
        &self,
        serializer: &KS,
    ) -> Option<KS::Output> {
        Some(serializer.serialize_key(self))
    }
}

impl TransitSerialize for &str {
    fn transit_serialize<S: TransitSerializer>(&self, serializer: &S) -> S::Output {
        serializer.serialize_string(self)
    }

    fn transit_serialize_key<KS: TransitKeySerializer>(
        &self,
        serializer: &KS,
    ) -> Option<KS::Output> {
        Some(serializer.serialize_key(self))
    }
}

impl<T: TransitSerialize> TransitSerialize for Option<T> {
    fn transit_serialize<S: TransitSerializer>(&self, serializer: &S) -> S::Output {
        if let Some(x) = self {
            x.transit_serialize(serializer)
        } else {
            serializer.serialize_null()
        }
    }

    fn transit_serialize_key<KS: TransitKeySerializer>(
        &self,
        serializer: &KS,
    ) -> Option<KS::Output> {
        Some(serializer.serialize_key("~_"))
    }
}

impl TransitSerialize for DateTime<Utc> {
    fn transit_serialize<S: TransitSerializer>(&self, serializer: &S) -> S::Output {
        serializer.serialize_string(&format!("~t{:?}", self))
    }

    fn transit_serialize_key<KS: TransitKeySerializer>(
        &self,
        serializer: &KS,
    ) -> Option<KS::Output> {
        Some(serializer.serialize_key(&format!("~t{:?}", self)))
    }
}
