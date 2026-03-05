use memmap2::MmapOptions;
use rayon::prelude::*;
use std::collections::{BTreeMap, HashMap};
use std::fs::File;

#[derive(Debug, Clone)]
struct CityData {
    min: f64,
    max: f64,
    sum: f64,
    count: u32,
}

impl CityData {
    fn new(temp: f64) -> Self {
        CityData {
            min: temp,
            max: temp,
            sum: temp,
            count: 1,
        }
    }

    fn update(&mut self, temp: f64) {
        self.min = self.min.min(temp);
        self.max = self.max.max(temp);
        self.sum += temp;
        self.count += 1;
    }

    fn merge(&mut self, other: &CityData) {
        self.min = self.min.min(other.min);
        self.max = self.max.max(other.max);
        self.sum += other.sum;
        self.count += other.count;
    }
}

fn parse_line(line: &str) -> Option<(&str, f64)> {
    let sep = line.find(';')?;
    let city = &line[..sep];
    let temp: f64 = line[sep + 1..].trim().parse().ok()?;
    Some((city, temp))
}

fn process_chunk(chunk: &[u8]) -> HashMap<String, CityData> {
    let mut map = HashMap::new();
    // Split chunk into lines and process each
    let text = std::str::from_utf8(chunk).unwrap_or("");
    for line in text.lines() {
        if let Some((city, temp)) = parse_line(line) {
            map.entry(city.to_string())
                .and_modify(|d: &mut CityData| d.update(temp))
                .or_insert_with(|| CityData::new(temp));
        }
    }
    map
}

fn main() {
    let msg = "Failed to open file";
    let mut _file_path = "/home/sgromme/source/1brc/data/measurements.txt";
    //_file_path = "measurements.txt";
    let file = File::open(_file_path).expect(msg);
    let mmap = unsafe { MmapOptions::new().map(&file).expect("Failed to mmap file") };

    let num_threads = rayon::current_num_threads();

    // Build partitions aligned to newlines
    let mut partitions: Vec<(usize, usize)> = Vec::new();
    let chunk_size = mmap.len() / num_threads;
    let mut start = 0;

    for _ in 0..num_threads {
        if start >= mmap.len() {
            break;
        }
        let mut end = (start + chunk_size).min(mmap.len());
        while end < mmap.len() && mmap[end] != b'\n' {
            end += 1;
        }
        if end < mmap.len() {
            end += 1;
        }
        partitions.push((start, end));
        start = end;
    }

    // ✅ Parallel processing: each partition gets its own HashMap, then we merge
    let city_data: HashMap<String, CityData> = partitions
        .par_iter()
        .map(|&(start, end)| process_chunk(&mmap[start..end]))
        .reduce(HashMap::new, |mut a, b| {
            for (city, data) in b {
                a.entry(city).and_modify(|d| d.merge(&data)).or_insert(data);
            }
            a
        });

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
