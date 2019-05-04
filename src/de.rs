mod impls;
pub mod json_verbose;

use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{BTreeMap, HashMap};
use std::convert::TryFrom;
use std::default::Default;
use std::fmt::Debug;
use std::mem;

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

pub trait TransitDeserializer: Debug + Clone {
    type Input: Debug + Default;
    type DeserializeArray: Iterator<Item = Self::Input>;
    type DeserializeMap: Iterator<Item = (Self::Input, Self::Input)>;

    //fn deserialize_null(self, v: Self::Input) -> Self::Output;
    fn deserialize_string(self, v: Self::Input) -> TResult<String>;
    fn deserialize_bool(self, v: Self::Input) -> TResult<bool>;
    fn deserialize_int(self, v: Self::Input) -> TResult<i64>;
    fn deserialize_float(self, v: Self::Input) -> TResult<f64>;
    fn deserialize_array(self, v: Self::Input) -> TResult<(Self::DeserializeArray, Option<usize>)>;
    fn deserialize_map(self, v: Self::Input) -> TResult<(Self::DeserializeMap, Option<usize>)>;
}
