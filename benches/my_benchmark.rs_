#[macro_use]
extern crate criterion;
use std::collections::{BTreeMap, HashMap};

use criterion::black_box;
use criterion::{Criterion, ParameterizedBenchmark};
use transit_rs::{de, ser};

fn serialize_benchmark(c: &mut Criterion) {
    c.bench(
        "composite keys serialize",
        ParameterizedBenchmark::new(
            "Verbose",
            |b, size| {
                let mut m = BTreeMap::new();
                for i in 0..*size {
                    let mut key1: BTreeMap<bool, String> = BTreeMap::new();
                    key1.insert(true, "test".to_owned());
                    key1.insert(false, format!("tset {}", i));

                    m.insert(key1, i);
                }

                b.iter(|| ser::json_verbose::to_transit_json(&m))
            },
            (500..10000).step_by(500),
            //(500..501).step_by(500),
        )
        .with_function("Non verbose", |b, size| {
            let mut m = BTreeMap::new();
            for i in 0..*size {
                let mut key1: BTreeMap<bool, String> = BTreeMap::new();
                key1.insert(true, "test".to_owned());
                key1.insert(false, format!("tset {}", i));

                m.insert(key1, i);
            }

            b.iter(|| ser::json::to_transit_json(&m))
        }),
    );
}

fn deserialize_benchmark(c: &mut Criterion) {
    c.bench_function_over_inputs(
        "composite keys deserialize",
        |b, size| {
            let mut m = HashMap::new();
            for i in 0..*size {
                let mut key1: BTreeMap<bool, String> = BTreeMap::new();
                key1.insert(true, "test".to_owned());
                key1.insert(false, format!("tset {}", i));

                m.insert(key1, i);
            }
            let tr = ser::json_verbose::to_transit_json(m);

            b.iter(|| {
                de::json_verbose::from_transit_json::<HashMap<BTreeMap<bool, String>, i32>>(
                    tr.clone(),
                )
            })
        },
        //(500..10000).step_by(500),
        //(500..501).step_by(500),
        (9500..9501).step_by(500),
    );
}

criterion_group!(benches, serialize_benchmark);
criterion_main!(benches);
