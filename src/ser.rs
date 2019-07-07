mod impls;
pub mod json_verbose;
pub mod json;

pub trait TransitSerialize {
    fn transit_serialize<S: TransitSerializer>(&self, serializer: &S) -> S::Output;
    fn transit_serialize_key<KS: TransitKeySerializer>(
        &self,
        serializer: &KS,
    ) -> Option<KS::Output>;
}

/// Trait for creation of final representation
/// because Transit is generic over JSON and MessagePack (both Verbose and not)
pub trait TransitSerializer {
    type Output;
    type MapSerializer: TransitMapSerializer<Output = Self::Output>;
    type ArraySerializer: TransitArraySerializer<Output = Self::Output>;
    type TaggedMapSerializer: TransitTaggedMapSerializer<Output = Self::Output>;
    type TaggedArraySerializer: TransitTaggedArraySerializer<Output = Self::Output>;

    fn serialize_null(&self) -> Self::Output;
    fn serialize_string(&self, v: &str) -> Self::Output;
    fn serialize_bool(&self, v: bool) -> Self::Output;
    fn serialize_int(&self, v: i64) -> Self::Output;
    fn serialize_float(&self, v: f64) -> Self::Output;

    fn serialize_array(&self, len: Option<usize>) -> Self::ArraySerializer;
    fn serialize_map(&self, len: Option<usize>) -> Self::MapSerializer;
    fn serialize_tagged_array(&self, tag: &str, len: Option<usize>) -> Self::TaggedArraySerializer;
    fn serialize_tagged_map(&self, tag: &str, len: Option<usize>) -> Self::TaggedMapSerializer;

    fn serialize_array_iter<'t, T, I>(&self, v: I) -> Self::Output
    where
        T: TransitSerialize + 't,
        I: Iterator<Item = &'t T>;

    fn serialize_map_iter<'t, K, V, I>(&self, v: I) -> Self::Output
    where
        K: TransitSerialize + 't,
        V: TransitSerialize + 't,
        I: Iterator<Item = (&'t K, &'t V)>;

    fn serialize_tagged_array_iter<'t, T, I>(&self, tag: &str, v: I) -> Self::Output
    where
        T: TransitSerialize + 't,
        I: Iterator<Item = &'t T>;

    fn serialize_tagged_map_iter<'i, 't, K, V, I>(&self, tag: &str, v: I) -> Self::Output
    where
        K: TransitSerialize + 't,
        V: TransitSerialize + 't,
        I: Iterator<Item = (&'t K, &'t V)>;
}

pub trait TransitKeySerializer {
    type Output;

    fn serialize_key(&self, v: &str) -> Self::Output;
}

pub trait TransitArraySerializer {
    type Output;

    fn serialize_item<T: TransitSerialize>(&mut self, v: &T);
    fn end(self) -> Self::Output;
}

pub trait TransitMapSerializer {
    type Output;

    fn serialize_pair<K: TransitSerialize, V: TransitSerialize>(&mut self, k: &K, v: &V);
    fn end(self) -> Self::Output;
}

pub trait TransitTaggedArraySerializer {
    type Output;

    fn serialize_item<T: TransitSerialize>(&mut self, v: &T);
    fn end(self) -> Self::Output;
}

pub trait TransitTaggedMapSerializer {
    type Output;

    fn serialize_pair<K: TransitSerialize, V: TransitSerialize>(&mut self, k: &K, v: &V);
    fn end(self) -> Self::Output;
}
