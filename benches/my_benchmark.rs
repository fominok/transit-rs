#[macro_use]
extern crate criterion;
use std::collections::{BTreeMap, HashMap};

use criterion::black_box;
use criterion::{Criterion, ParameterizedBenchmark};
use transit_rs::ser;

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

// fn deserialize_benchmark(c: &mut Criterion) {
//     c.bench_function_over_inputs(
//         "composite keys deserialize",
//         |b, size| {
//             let mut m = HashMap::new();
//             for i in 0..*size {
//                 let mut key1: BTreeMap<bool, String> = BTreeMap::new();
//                 key1.insert(true, "test".to_owned());
//                 key1.insert(false, format!("tset {}", i));
//
//                 m.insert(key1, i);
//             }
//             let tr = ser::json_verbose::to_transit_json(m);
//
//             b.iter(|| {
//                 de::json_verbose::from_transit_json::<HashMap<BTreeMap<bool, String>, i32>>(
//                     tr.clone(),
//                 )
//             })
//         },
//         //(500..10000).step_by(500),
//         //(500..501).step_by(500),
//         (9500..9501).step_by(500),
//     );
// }

fn caching_benchmark(c: &mut Criterion) {
    c.bench(
        "array caching",
        ParameterizedBenchmark::new(
            "with caching",
            |b, size| {
                let mut v = Vec::with_capacity(*size);
                for i in 0..*size {
                    let mut m = HashMap::with_capacity(44);
                    m.insert("extremelyveryveryveryveryverylongkey1", "useful string");
                    m.insert("extremelyveryveryveryveryverylongkey2", "useful string");
                    m.insert("extremelyveryveryveryveryverylongkey3", "useful string");
                    m.insert("extremelyveryveryveryveryverylongkey4", "useful string");
                    m.insert("extremelyveryveryveryveryverylongkey5", "useful string");
                    m.insert("extremelyveryveryveryveryverylongkey6", "useful string");
                    m.insert("extremelyveryveryveryveryverylongkey7", "useful string");
                    m.insert("extremelyveryveryveryveryverylongkey8", "useful string");
                    m.insert("extremelyveryveryveryveryverylongkey9", "useful string");
                    m.insert("extremelyveryveryveryveryverylongkey10", "useful string");
                    m.insert("extremelyveryveryveryveryverylongkey11", "useful string");
                    m.insert("extremelyveryveryveryveryverylongkeyz1", "useful string");
                    m.insert("extremelyveryveryveryveryverylongkeyz2", "useful string");
                    m.insert("extremelyveryveryveryveryverylongkeyz3", "useful string");
                    m.insert("extremelyveryveryveryveryverylongkeyz4", "useful string");
                    m.insert("extremelyveryveryveryveryverylongkeyz5", "useful string");
                    m.insert("extremelyveryveryveryveryverylongkeyz6", "useful string");
                    m.insert("extremelyveryveryveryveryverylongkeyz7", "useful string");
                    m.insert("extremelyveryveryveryveryverylongkeyz8", "useful string");
                    m.insert("extremelyveryveryveryveryverylongkeyz9", "useful string");
                    m.insert("extremelyveryveryveryveryverylongkeyz10", "useful string");
                    m.insert("extremelyveryveryveryveryverylongkeyz11", "useful string");
                    m.insert("rextremelyveryveryveryveryverylongkey1", "useful string");
                    m.insert("rextremelyveryveryveryveryverylongkey2", "useful string");
                    m.insert("rextremelyveryveryveryveryverylongkey3", "useful string");
                    m.insert("rextremelyveryveryveryveryverylongkey4", "useful string");
                    m.insert("rextremelyveryveryveryveryverylongkey5", "useful string");
                    m.insert("rextremelyveryveryveryveryverylongkey6", "useful string");
                    m.insert("rextremelyveryveryveryveryverylongkey7", "useful string");
                    m.insert("rextremelyveryveryveryveryverylongkey8", "useful string");
                    m.insert("rextremelyveryveryveryveryverylongkey9", "useful string");
                    m.insert("rextremelyveryveryveryveryverylongkey10", "useful string");
                    m.insert("rextremelyveryveryveryveryverylongkey11", "useful string");
                    m.insert("rextremelyveryveryveryveryverylongkeyz1", "useful string");
                    m.insert("rextremelyveryveryveryveryverylongkeyz2", "useful string");
                    m.insert("rextremelyveryveryveryveryverylongkeyz3", "useful string");
                    m.insert("rextremelyveryveryveryveryverylongkeyz4", "useful string");
                    m.insert("rextremelyveryveryveryveryverylongkeyz5", "useful string");
                    m.insert("rextremelyveryveryveryveryverylongkeyz6", "useful string");
                    m.insert("rextremelyveryveryveryveryverylongkeyz7", "useful string");
                    m.insert("rextremelyveryveryveryveryverylongkeyz8", "useful string");
                    m.insert("rextremelyveryveryveryveryverylongkeyz9", "useful string");
                    m.insert("rextremelyveryveryveryveryverylongkeyz10", "useful string");
                    m.insert("rextremelyveryveryveryveryverylongkeyz11", "useful string");
                    v.push(m);
                }

                b.iter(|| ser::json::to_transit_json(&v).to_string())
            },
            (1000..15001).step_by(1000),
            //(500..501).step_by(500),
        )
        .with_function("without caching", |b, size| {
            let mut v = Vec::with_capacity(*size);
            for i in 0..*size {
                let mut m = HashMap::with_capacity(44);
                m.insert("extremelyveryveryveryveryverylongkey1", "useful string");
                m.insert("extremelyveryveryveryveryverylongkey2", "useful string");
                m.insert("extremelyveryveryveryveryverylongkey3", "useful string");
                m.insert("extremelyveryveryveryveryverylongkey4", "useful string");
                m.insert("extremelyveryveryveryveryverylongkey5", "useful string");
                m.insert("extremelyveryveryveryveryverylongkey6", "useful string");
                m.insert("extremelyveryveryveryveryverylongkey7", "useful string");
                m.insert("extremelyveryveryveryveryverylongkey8", "useful string");
                m.insert("extremelyveryveryveryveryverylongkey9", "useful string");
                m.insert("extremelyveryveryveryveryverylongkey10", "useful string");
                m.insert("extremelyveryveryveryveryverylongkey11", "useful string");
                m.insert("extremelyveryveryveryveryverylongkeyz1", "useful string");
                m.insert("extremelyveryveryveryveryverylongkeyz2", "useful string");
                m.insert("extremelyveryveryveryveryverylongkeyz3", "useful string");
                m.insert("extremelyveryveryveryveryverylongkeyz4", "useful string");
                m.insert("extremelyveryveryveryveryverylongkeyz5", "useful string");
                m.insert("extremelyveryveryveryveryverylongkeyz6", "useful string");
                m.insert("extremelyveryveryveryveryverylongkeyz7", "useful string");
                m.insert("extremelyveryveryveryveryverylongkeyz8", "useful string");
                m.insert("extremelyveryveryveryveryverylongkeyz9", "useful string");
                m.insert("extremelyveryveryveryveryverylongkeyz10", "useful string");
                m.insert("extremelyveryveryveryveryverylongkeyz11", "useful string");
                m.insert("rextremelyveryveryveryveryverylongkey1", "useful string");
                m.insert("rextremelyveryveryveryveryverylongkey2", "useful string");
                m.insert("rextremelyveryveryveryveryverylongkey3", "useful string");
                m.insert("rextremelyveryveryveryveryverylongkey4", "useful string");
                m.insert("rextremelyveryveryveryveryverylongkey5", "useful string");
                m.insert("rextremelyveryveryveryveryverylongkey6", "useful string");
                m.insert("rextremelyveryveryveryveryverylongkey7", "useful string");
                m.insert("rextremelyveryveryveryveryverylongkey8", "useful string");
                m.insert("rextremelyveryveryveryveryverylongkey9", "useful string");
                m.insert("rextremelyveryveryveryveryverylongkey10", "useful string");
                m.insert("rextremelyveryveryveryveryverylongkey11", "useful string");
                m.insert("rextremelyveryveryveryveryverylongkeyz1", "useful string");
                m.insert("rextremelyveryveryveryveryverylongkeyz2", "useful string");
                m.insert("rextremelyveryveryveryveryverylongkeyz3", "useful string");
                m.insert("rextremelyveryveryveryveryverylongkeyz4", "useful string");
                m.insert("rextremelyveryveryveryveryverylongkeyz5", "useful string");
                m.insert("rextremelyveryveryveryveryverylongkeyz6", "useful string");
                m.insert("rextremelyveryveryveryveryverylongkeyz7", "useful string");
                m.insert("rextremelyveryveryveryveryverylongkeyz8", "useful string");
                m.insert("rextremelyveryveryveryveryverylongkeyz9", "useful string");
                m.insert("rextremelyveryveryveryveryverylongkeyz10", "useful string");
                m.insert("rextremelyveryveryveryveryverylongkeyz11", "useful string");
                v.push(m);
            }

            b.iter(|| ser::json_verbose::to_transit_json(&v).to_string())
        }),
    );
}

criterion_group!(benches, caching_benchmark);
criterion_main!(benches);
