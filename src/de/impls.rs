use super::*;

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
        let mut map_iter = deserializer.clone().deserialize_map(input)?.0;
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
                if let Some((k, v)) = map_iter.next() {
                    let k_str = deserializer.clone().deserialize_string(k)?;
                    if k_str == "~#cmap" {
                        Ok(())
                    } else {
                        Err(Error::DoNotMatch(format!("{:?} must be ~#cmap", k_str)))
                    }?;
                    let mut vals: Vec<D::Input> =
                        deserializer.clone().deserialize_array(v)?.0.collect();
                    for i in (0..vals.len()).step_by(2) {
                        result.insert(
                            TransitDeserialize::transit_deserialize(
                                deserializer.clone(),
                                mem::replace(&mut vals[i], D::Input::default()),
                            )?,
                            TransitDeserialize::transit_deserialize(
                                deserializer.clone(),
                                mem::replace(&mut vals[i + 1], D::Input::default()),
                            )?,
                        );
                    }
                }
                Ok(result)
            }
        }
    }

    fn transit_deserialize_key<D: TransitDeserializer>(
        _deserializer: D,
        _input: D::Input,
    ) -> TResult<Self> {
        Err(Error::CannotBeKey(
            "BTreeMap<K, V> cannot be deserialized as key",
        ))
    }
}

impl<K, V> TransitDeserialize for HashMap<K, V>
where
    K: TransitDeserialize + std::hash::Hash + Eq,
    V: TransitDeserialize,
{
    const TF_TYPE: TransitType = TransitType::Composite;

    fn transit_deserialize<D: TransitDeserializer>(
        deserializer: D,
        input: D::Input,
    ) -> TResult<Self> {
        let (mut map_iter, cap) = deserializer.clone().deserialize_map(input)?;
        let mut result: Self = if let Some(c) = cap {
            HashMap::with_capacity(c)
        } else {
            HashMap::new()
        };
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
                if let Some((k, v)) = map_iter.next() {
                    let k_str = deserializer.clone().deserialize_string(k)?;
                    if k_str == "~#cmap" {
                        Ok(())
                    } else {
                        Err(Error::DoNotMatch(format!("{:?} must be ~#cmap", k_str)))
                    }?;
                    let mut vals: Vec<D::Input> =
                        deserializer.clone().deserialize_array(v)?.0.collect();
                    for i in (0..vals.len()).step_by(2) {
                        result.insert(
                            TransitDeserialize::transit_deserialize(
                                deserializer.clone(),
                                mem::replace(&mut vals[i], D::Input::default()),
                            )?,
                            TransitDeserialize::transit_deserialize(
                                deserializer.clone(),
                                mem::replace(&mut vals[i + 1], D::Input::default()),
                            )?,
                        );
                    }
                }
                Ok(result)
            }
        }
    }

    fn transit_deserialize_key<D: TransitDeserializer>(
        _deserializer: D,
        _input: D::Input,
    ) -> TResult<Self> {
        Err(Error::CannotBeKey(
            "HashMap<K, V> cannot be deserialized as key",
        ))
    }
}

impl<T: TransitDeserialize> TransitDeserialize for Vec<T> {
    const TF_TYPE: TransitType = TransitType::Composite;

    fn transit_deserialize<D: TransitDeserializer>(
        deserializer: D,
        input: D::Input,
    ) -> TResult<Self> {
        let (array_iter, cap) = deserializer.clone().deserialize_array(input)?;
        let mut v = if let Some(c) = cap {
            Vec::with_capacity(c)
        } else {
            Vec::new()
        };
        for x in array_iter {
            v.push(TransitDeserialize::transit_deserialize(
                deserializer.clone(),
                x,
            )?);
        }
        Ok(v)
    }

    fn transit_deserialize_key<D: TransitDeserializer>(
        _deserializer: D,
        _input: D::Input,
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
        let s = deserializer.deserialize_string(input)?;
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
        let s = deserializer.deserialize_string(input)?;
        RE.captures(&s)
            .ok_or(Error::DoNotMatch(format!("not proper i32 key",)))
            .and_then(|cap| {
                cap.name("int")
                    .ok_or(Error::DoNotMatch(format!("not proper i32 key",)))
                    .and_then(|i| {
                        (i.as_str())
                            .parse::<Self>()
                            .map_err(|_| Error::DoNotMatch(format!("not i32")))
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
