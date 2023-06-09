mod assemble;
mod cpu;
mod instruction;
mod optimise;

extern crate csv;
use rand::seq::IteratorRandom;

use core::num;
use std::time::Instant;
use std::collections::HashMap;
use std::fs::{OpenOptions, File};
use std::io::{BufReader, BufWriter};
use csv::ReaderBuilder;
use csv::WriterBuilder;

fn cartesian_product(n: u32, repeat: usize) -> Vec<Vec<u32>> {
    if repeat == 0 {
        vec![Vec::new()]
    } else {
        let mut all_states = Vec::new();
        for i in 0..=n {
            for mut v in cartesian_product(n, repeat - 1) {
                v.push(i);
                all_states.push(v);
            }
        }
        all_states
    }
}

fn get_tested_configs(file_path: &str) -> HashMap<(String, usize, u32), bool> {
    let mut tested_configs = HashMap::new();
    if let Ok(file) = File::open(file_path) {
        let reader = ReaderBuilder::new().from_reader(BufReader::new(file));
        for result in reader.into_records().skip(1) {
            if let Ok(record) = result {
                let state = record.get(0).unwrap().to_string();
                let max_length: usize = record.get(1).unwrap().parse().unwrap();
                let max_val: u32 = record.get(2).unwrap().parse().unwrap();
                tested_configs.insert((state, max_length, max_val), true);
            }
        }
    }
    tested_configs
}

pub fn benchmark_optimal_from_state(file_path: &str, max_mem_cells: usize, max_length: usize, max_val: u32, num_test_states: Option<usize>) {
    assert!(max_length > 2);
    assert!(max_val > 2);
    let all_lengths_vals: Vec<(usize, u32)> = (2..=max_length)
        .flat_map(|x| (2..=max_val).map(move |y| (x, y)))
        .collect();

    let mut tested_configs = get_tested_configs(file_path);

    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(file_path)
        .unwrap();

    let mut writer = WriterBuilder::new().from_writer(BufWriter::new(file));

    if tested_configs.is_empty() {
        writer.write_record(&["State", "Max Length", "Max Val", "Execution Time"]).unwrap();
    }

    for (max_length, max_val) in all_lengths_vals {
        let mut all_states = cartesian_product(max_val, max_mem_cells);
        if num_test_states.is_some() {
            all_states = all_states.iter()
                .choose_multiple(&mut rand::thread_rng(), num_test_states.unwrap()).into_iter()
                .cloned()  // Clone the items to get owned values
                .collect();
        }
        for state in all_states {
            let state_string = format!("{:?}", state);
            if !tested_configs.contains_key(&(state_string.clone(), max_length, max_val)) {
                let start = Instant::now();
                optimise::optimal_from_state(state, max_length, max_val, false);
                let execution_time = start.elapsed();
                writer.write_record(&[state_string.clone(), max_length.to_string(), max_val.to_string(), execution_time.as_secs_f32().to_string()]).unwrap();
                tested_configs.insert((state_string, max_length, max_val), true);
            }
        }
        writer.flush();
    }
}

// Run the benchmarking function
fn main() {
    benchmark_optimal_from_state("benchmark_results.rust.csv", 3, 5, 5, Option::Some(100) );
}
