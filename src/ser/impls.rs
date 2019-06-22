use super::*;
use chrono::{DateTime, Utc};
use std::collections::{BTreeMap, BTreeSet, HashMap};

impl<T: TransitSerialize + ?Sized> TransitSerialize for Box<T> {
    fn transit_serialize<S: TransitSerializer>(&self, serializer: &S) -> TransitType<S::Output> {
        (**self).transit_serialize(serializer)
    }

    fn transit_serialize_key<S: TransitSerializer>(
        &self,
        serializer: &S,
    ) -> TransitType<S::Output> {
        (**self).transit_serialize_key(serializer)
    }
}

impl<'a, T: TransitSerialize + ?Sized> TransitSerialize for &'a T {
    fn transit_serialize<S: TransitSerializer>(&self, serializer: &S) -> TransitType<S::Output> {
        (**self).transit_serialize(serializer)
    }

    fn transit_serialize_key<S: TransitSerializer>(
        &self,
        serializer: &S,
    ) -> TransitType<S::Output> {
        (**self).transit_serialize_key(serializer)
    }
}

impl TransitSerialize for bool {
    fn transit_serialize<S: TransitSerializer>(&self, serializer: &S) -> TransitType<S::Output> {
        serializer.serialize_bool(*self)
    }

    fn transit_serialize_key<S: TransitSerializer>(
        &self,
        serializer: &S,
    ) -> TransitType<S::Output> {
        let s = if *self {
            "~?t".to_owned()
        } else {
            "~?f".to_owned()
        };

        serializer.serialize_string(&s)
    }
}

impl<K: TransitSerialize, V: TransitSerialize> TransitSerialize for BTreeMap<K, V> {
    fn transit_serialize<S: TransitSerializer>(&self, serializer: &S) -> TransitType<S::Output> {
        serializer.serialize_map(self.iter())
    }

    fn transit_serialize_key<S: TransitSerializer>(
        &self,
        serializer: &S,
    ) -> TransitType<S::Output> {
        self.transit_serialize(serializer)
    }
}

impl<K: TransitSerialize, V: TransitSerialize> TransitSerialize for HashMap<K, V> {
    fn transit_serialize<S: TransitSerializer>(&self, serializer: &S) -> TransitType<S::Output> {
        serializer.serialize_map(self.iter())
    }

    fn transit_serialize_key<S: TransitSerializer>(
        &self,
        serializer: &S,
    ) -> TransitType<S::Output> {
        self.transit_serialize(serializer)
    }
}

impl<T: TransitSerialize> TransitSerialize for Vec<T> {
    fn transit_serialize<S: TransitSerializer>(&self, serializer: &S) -> TransitType<S::Output> {
        serializer.serialize_array(self.iter())
    }

    fn transit_serialize_key<S: TransitSerializer>(
        &self,
        serializer: &S,
    ) -> TransitType<S::Output> {
        self.transit_serialize(serializer)
    }
}

impl<T: TransitSerialize> TransitSerialize for BTreeSet<T> {
    fn transit_serialize<S: TransitSerializer>(&self, serializer: &S) -> TransitType<S::Output> {
        serializer.serialize_tagged_array("~#set", self.iter())
    }

    fn transit_serialize_key<S: TransitSerializer>(
        &self,
        serializer: &S,
    ) -> TransitType<S::Output> {
        self.transit_serialize(serializer)
    }
}

impl TransitSerialize for i32 {
    fn transit_serialize<S: TransitSerializer>(&self, serializer: &S) -> TransitType<S::Output> {
        serializer.serialize_int((*self).into())
    }

    fn transit_serialize_key<S: TransitSerializer>(
        &self,
        serializer: &S,
    ) -> TransitType<S::Output> {
        serializer.serialize_string(&format!("~i{}", self))
    }
}

impl TransitSerialize for String {
    fn transit_serialize<S: TransitSerializer>(&self, serializer: &S) -> TransitType<S::Output> {
        serializer.serialize_string(self)
    }

    fn transit_serialize_key<S: TransitSerializer>(
        &self,
        serializer: &S,
    ) -> TransitType<S::Output> {
        serializer.serialize_string(self)
    }
}

impl TransitSerialize for &str {
    fn transit_serialize<S: TransitSerializer>(&self, serializer: &S) -> TransitType<S::Output> {
        serializer.serialize_string(self)
    }

    fn transit_serialize_key<S: TransitSerializer>(
        &self,
        serializer: &S,
    ) -> TransitType<S::Output> {
        serializer.serialize_string(self)
    }
}

impl<T: TransitSerialize> TransitSerialize for Option<T> {
    fn transit_serialize<S: TransitSerializer>(&self, serializer: &S) -> TransitType<S::Output> {
        if let Some(x) = self {
            x.transit_serialize(serializer)
        } else {
            serializer.serialize_null()
        }
    }

    fn transit_serialize_key<S: TransitSerializer>(
        &self,
        serializer: &S,
    ) -> TransitType<S::Output> {
        serializer.serialize_string("~_")
    }
}

fn date_tagged<S>(d: DateTime<Utc>, serializer: &S) -> TransitType<S::Output>
where
    S: TransitSerializer,
{
    serializer.serialize_string(&format!("~t{:?}", d))
}

impl TransitSerialize for DateTime<Utc> {
    fn transit_serialize<S: TransitSerializer>(&self, serializer: &S) -> TransitType<S::Output> {
        date_tagged(*self, serializer)
    }

    fn transit_serialize_key<S: TransitSerializer>(
        &self,
        serializer: &S,
    ) -> TransitType<S::Output> {
        date_tagged(*self, serializer)
    }
}
