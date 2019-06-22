use super::*;
use serde_json::{map::Map as JsMap, Value as JsVal};
//use transit_derive::TransitSerialize;
use itertools::Itertools;

pub fn to_transit_json<T: TransitSerialize>(v: T) -> JsVal {
    v.transit_serialize(&JsonSerializer::top()).unpack()
}

struct JsonSerializer {
    top_level: bool,
}

impl Default for JsonSerializer {
    fn default() -> Self {
        JsonSerializer { top_level: false }
    }
}

impl JsonSerializer {
    fn top() -> Self {
        JsonSerializer { top_level: true }
    }
    fn quote_check(&self, v: JsVal) -> JsVal {
        if self.top_level {
            let mut m = JsMap::with_capacity(1);
            m.insert("~#".to_owned(), v);
            JsVal::Object(m)
        } else {
            v
        }
    }
}

impl TransitSerializer for JsonSerializer {
    type Output = JsVal;

    fn serialize_null(&self) -> TransitType<Self::Output> {
        TransitType::Scalar(self.quote_check(JsVal::Null))
    }

    fn serialize_string(&self, v: &str) -> TransitType<Self::Output> {
        TransitType::Scalar(self.quote_check(v.into()))
    }

    fn serialize_bool(&self, v: bool) -> TransitType<Self::Output> {
        TransitType::Scalar(self.quote_check(v.into()))
    }

    fn serialize_int(&self, v: i64) -> TransitType<Self::Output> {
        TransitType::Scalar(self.quote_check(v.into()))
    }

    fn serialize_float(&self, v: f64) -> TransitType<Self::Output> {
        TransitType::Scalar(self.quote_check(v.into()))
    }

    fn serialize_array<'t, T, I>(&self, v: I) -> TransitType<Self::Output>
    where
        T: TransitSerialize + 't,
        I: Iterator<Item = &'t T>,
    {
        let serializer = Self::default();
        let v_ser = v
            .map(|x| x.transit_serialize(&serializer).unpack())
            .collect();
        TransitType::Composite(JsVal::Array(v_ser))
    }

    fn serialize_map<'t, K, V, I>(&self, v: I) -> TransitType<Self::Output>
    where
        K: TransitSerialize + 't,
        V: TransitSerialize + 't,
        I: Iterator<Item = (&'t K, &'t V)>,
    {
        let serializer = Self::default();
        let mut has_comp_key = false;
        let (ser_k, ser_v): (Vec<JsVal>, Vec<JsVal>) = v
            .map(|(key, value)| {
                (
                    {
                        let k = key.transit_serialize_key(&serializer);
                        match k {
                            TransitType::Composite(_) => has_comp_key = true,
                            _ => (),
                        }
                        k.unpack()
                    },
                    value.transit_serialize(&serializer).unpack(),
                )
            })
            .unzip();

        if has_comp_key {
            let interleaved: Vec<JsVal> = ser_k.into_iter().interleave(ser_v).collect();
            let mut m = JsMap::with_capacity(1);
            m.insert("~#cmap".to_owned(), JsVal::Array(interleaved));
            TransitType::Composite(JsVal::Object(m))
        } else {
            let mut m = JsMap::with_capacity(ser_k.len());
            for (key, value) in ser_k.into_iter().zip(ser_v) {
                m.insert(
                    key.as_str()
                        .expect("Scalar keys are always strings")
                        .to_owned(),
                    value,
                );
            }
            TransitType::Composite(JsVal::Object(m))
        }
    }

    fn serialize_tagged_array<'t, T, I>(&self, tag: &str, v: I) -> TransitType<Self::Output>
    where
        T: TransitSerialize + 't,
        I: Iterator<Item = &'t T>,
    {
        let v_ser = self.serialize_array(v).unpack();
        let mut m = JsMap::with_capacity(1);
        m.insert(tag.to_owned(), v_ser);
        TransitType::Composite(JsVal::Object(m))
    }

    fn serialize_tagged_map<'t, K, V, I>(&self, tag: &str, v: I) -> TransitType<Self::Output>
    where
        K: TransitSerialize + 't,
        V: TransitSerialize + 't,
        I: Iterator<Item = (&'t K, &'t V)>,
    {
        let m_ser = self.serialize_map(v).unpack();
        let mut m = JsMap::with_capacity(1);
        m.insert(tag.to_owned(), m_ser);
        TransitType::Composite(JsVal::Object(m))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json::json;
    use std::collections::{BTreeMap, BTreeSet};

    #[test]
    fn scalar_map_btree() {
        let mut m = BTreeMap::new();
        m.insert(4, "yolo");
        m.insert(-6, "swag");

        let tr = to_transit_json(m);
        assert_eq!(
            json!({
                "~i4": "yolo",
                "~i-6": "swag"
            }),
            tr
        );
    }

    #[test]
    fn map_composite_keys() {
        let mut key1: BTreeMap<bool, &str> = BTreeMap::new();
        key1.insert(true, "test");
        key1.insert(false, "tset");

        let mut m = BTreeMap::new();
        m.insert(key1, 1337);

        let tr = to_transit_json(m);
        assert_eq!(
            json!({
                "~#cmap": [
                    {
                        "~?t": "test",
                        "~?f": "tset",
                    },
                    1337
                ],
            }),
            tr
        );
    }

    #[test]
    fn quoting() {
        let tr = to_transit_json(5i32);
        assert_eq!(
            json!({
                "~#": 5
            }),
            tr
        );
    }

    #[test]
    fn tagged_set() {
        let mut hs = BTreeSet::new();
        hs.insert(0);
        hs.insert(2);
        hs.insert(4);

        let tr = to_transit_json(hs);
        assert_eq!(json!({"~#set": [0, 2, 4]}), tr);
    }

    // #[test]
    // fn custom_derive_stuct_map() {
    //     use chrono::{DateTime, TimeZone, Utc};
    //     #[derive(Clone, TransitSerialize)]
    //     struct User {
    //         name: String,
    //         related: BTreeSet<String>,
    //         registered: DateTime<Utc>,
    //         skills_by_rates: BTreeMap<i32, BTreeSet<String>>,
    //     }

    //     let mut rel = BTreeSet::new();
    //     rel.insert("Billy".to_owned());
    //     rel.insert("Mark".to_owned());
    //     rel.insert("Steve".to_owned());

    //     let mut skills = BTreeMap::new();
    //     let mut hs1 = BTreeSet::new();
    //     hs1.insert("Linux".to_owned());
    //     hs1.insert("Git".to_owned());
    //     skills.insert(3, hs1);
    //     let mut hs2 = BTreeSet::new();
    //     hs2.insert("Performance artist".to_owned());
    //     skills.insert(2, hs2);
    //     let mut hs3 = BTreeSet::new();
    //     hs3.insert("Rust".to_owned());
    //     skills.insert(1, hs3);

    //     let u = User {
    //         name: "Van".to_owned(),
    //         related: rel,
    //         registered: Utc.ymd(1995, 10, 11).and_hms(0, 0, 0),
    //         skills_by_rates: skills,
    //     };
    //     let tr = to_transit_json(u);
    //     assert_eq!(
    //         json!(
    //             {
    //                 "~#user": {
    //                     "name": "Van",
    //                     "related": {"~#set": ["Billy", "Mark", "Steve"]},
    //                     "registered": "~t1995-10-11T00:00:00Z",
    //                     "skills_by_rates": {
    //                         "~i3": {"~#set": ["Git", "Linux"]},
    //                         "~i2": {"~#set": ["Performance artist"]},
    //                         "~i1": {"~#set": ["Rust"]},
    //                     }
    //                 }
    //             }
    //         ),
    //         tr
    //     );
    // }

    // #[test]
    // fn custom_derive_stuct_tuple() {
    //     #[derive(Clone, TransitSerialize)]
    //     struct Point(i32, i32);
    //     let p = Point(13, 37);

    //     let tr = to_transit_json(p);
    //     assert_eq!(json!({"~#point": [13, 37]}), tr);
    // }

    // #[test]
    // fn custom_derive_enum() {
    //     #[derive(Clone, TransitSerialize)]
    //     enum Event {
    //         TemperatureChanged { room_name: String, temperature: i32 },
    //         MotionDetected { room_name: String },
    //         GoneOnline(String),
    //         GoneOffline(String),
    //         LightsStatus { room_names: Vec<String> },
    //     }

    //     let e1 = Event::TemperatureChanged {
    //         room_name: "Kitchen".to_owned(),
    //         temperature: 32,
    //     };
    //     let tr1 = to_transit_json(e1);
    //     assert_eq!(
    //         json!(
    //         {
    //             "~#temperaturechanged": {
    //                 "room_name": "Kitchen",
    //                 "temperature": 32
    //             }
    //         }),
    //         tr1
    //     );

    //     let e2 = Event::GoneOffline("device".to_owned());
    //     let tr2 = to_transit_json(e2);
    //     assert_eq!(
    //         json!(
    //         {
    //             "~#goneoffline": ["device"]
    //         }),
    //         tr2
    //     );

    //     let names = vec!["test1".to_owned(), "test2".to_owned()];
    //     let e3 = Event::LightsStatus { room_names: names };
    //     let tr3 = to_transit_json(e3);
    //     assert_eq!(
    //         json!(
    //         {
    //             "~#lightsstatus": { "room_names": ["test1", "test2"] }
    //         }),
    //         tr3
    //     );
    // }

    #[test]
    fn array() {
        let mut m1 = BTreeMap::new();
        let mut m2 = BTreeMap::new();
        m1.insert("test", true);
        m1.insert("hih", true);
        m2.insert("ok", true);
        m2.insert("not ok", false);
        let v = vec![Box::new(m1), Box::new(m2)];

        let tr = to_transit_json(v);
        assert_eq!(
            json!([
                 {
                     "test": true,
                     "hih": true
                 },
                 {
                     "ok": true,
                     "not ok": false
                 }
            ]),
            tr
        );
    }
}
