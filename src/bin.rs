extern crate transit_rs;

use std::collections::{BTreeMap, HashMap};
use transit_rs::ser;

fn main() {
    for size in (1000..15001).step_by(1000) {
        let mut v = Vec::with_capacity(size);
        for i in 0..size {
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

        let yolo = ser::json::to_transit_json(&v).to_string().bytes().count() / 1024;
        let swag = ser::json_verbose::to_transit_json(&v)
            .to_string()
            .bytes()
            .count() / 1024;
        println!("{}: cache: {}, nocache: {}", size, yolo, swag);
    }
}
