use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn main() {
    let mut data = HashMap::<String, f64>::new();

    let msg = "Failed to open file";
    let file_path = "/home/sgromme/source/1brc/data/measurements.txt";
    let file = File::open(file_path).expect(msg);

    //
    let file = BufReader::new(file);

    for line in file.lines().flatten() {
        let (city, temp) = if let Some((city, temp)) = line.split_once(';') {
            let temp: f64 = temp.parse().unwrap();
            (city.to_string(), temp)
        } else {
            // None continue
            continue;
        };
        let city_data = data.entry(city.to_string()).or_insert(temp);
        // max temp for city, if temp is higher than current max, update it
        *city_data = temp.max(*city_data);
    }
}
