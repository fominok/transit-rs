#[macro_use]
extern crate criterion;
use std::collections::BTreeMap;

use criterion::black_box;
use criterion::Criterion;
use transit_rs::{de, ser};

fn serialize_benchmark(c: &mut Criterion) {
    c.bench_function_over_inputs(
        "composite keys serialize",
        |b, size| {
            let mut m = BTreeMap::new();
            for i in 0..*size {
                let mut key1: BTreeMap<bool, String> = BTreeMap::new();
                key1.insert(true, "test".to_owned());
                key1.insert(false, format!("tset {}", i));

                m.insert(key1, i);
            }

            b.iter(|| ser::json::to_transit_json(&m))
        },
        (500..10000).step_by(500),
    );
}

fn deserialize_benchmark(c: &mut Criterion) {
    c.bench_function_over_inputs(
        "composite keys deserialize",
        |b, size| {
            let mut m = BTreeMap::new();
            for i in 0..*size {
                let mut key1: BTreeMap<bool, String> = BTreeMap::new();
                key1.insert(true, "test".to_owned());
                key1.insert(false, format!("tset {}", i));

                m.insert(key1, i);
            }
            let tr = ser::json::to_transit_json(m);

            b.iter(|| de::from_transit_json::<BTreeMap<BTreeMap<bool, String>, i32>>(tr.clone()))
        },
        (500..10000).step_by(500),
    );
}

criterion_group!(benches, deserialize_benchmark);
criterion_main!(benches);
