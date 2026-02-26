use std::collections::{BTreeMap, HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone)]
struct CityData {
    min: f64,
    max: f64,
    sum: f64,
    count: u32,
}

impl CityData {
    fn update(&mut self, temp: f64) {
        self.min = self.min.min(temp);
        self.max = self.max.max(temp);
        self.sum += temp;
        self.count += 1;
    }
    fn update_initial(temp: f64) -> Self {
        CityData {
            min: temp,
            max: temp,
            sum: temp,
            count: 1,
        }
    }
    fn merge(&mut self, other: &CityData) {
        self.min = self.min.min(other.min);
        self.max = self.max.max(other.max);
        self.sum += other.sum;
        self.count += other.count;
    }
}

fn main() {
    let mut city_data = HashMap::<String, CityData>::new();

    let msg = "Failed to open file";
    let mut _file_path = "/home/sgromme/source/1brc/data/measurements.txt";
    _file_path = "measurements.txt";
    let file = File::open(_file_path).expect(msg);

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
        city_data
            .entry(city)
            .and_modify(|d| d.update(temp))
            .or_insert_with(|| CityData::update_initial(temp));
    }

    // Putting the HashMap into a BtreeMap to sort by city name
    let city_data = BTreeMap::from_iter(city_data.into_iter());

    for (city, data) in city_data {
        print!(
            " {}: min: {}, max: {}, avg: {:.2}",
            city,
            data.min,
            data.max,
            data.sum / data.count as f64
        );
    }
}
