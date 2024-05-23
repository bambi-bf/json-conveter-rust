use serde::{Serialize,Deserialize};
use std::fs::File;
use std::io::{BufReader, Read, Write};

#[derive(Deserialize, Debug)]
struct InputData {
    pairIndex: u64,
    price: f64,
    ts: f64,
}

#[derive(Serialize)]
struct OutputData {
    open: f64,
    high: f64,
    low: f64,
    close: f64
}

fn main() {
    // Open the JSON file
    let file = File::open("input.json").expect("Failed to open file");
    let reader = BufReader::new(file);

    // Deserialize JSON data into a vector of InputData
    let data: Vec<InputData> = serde_json::from_reader(reader).expect("Failed to deserialize JSON");

    // Print the deserialized data
    let mut cnt = 0;
    let mut open: Vec<f64> = Vec::new();
    let mut high: Vec<f64> = Vec::new();
    let mut low: Vec<f64> = Vec::new();
    let mut close: Vec<f64> = Vec::new();
    let mut last_high_index: Option<usize> = None;
    let mut last_low_index: Option<usize> = None;
    let mut last_close_index: Option<usize> = None;

    let mut output: Vec<OutputData> = Vec::new();

    for entry in data {
        while entry.pairIndex != cnt as u64 {
            if cnt == open.len() {
                open.push(0.0);
                high.push(0.0);
                low.push(0.0);
                close.push(0.0);
            }
            cnt += 1;
        }
        if cnt as u64 == entry.pairIndex {
            if cnt as u64 == open.len() as u64 {
                open.push(entry.price);
                high.push(entry.price);
                low.push(entry.price);
                close.push(entry.price);
                last_high_index = Some(high.len() - 1);
                last_low_index = Some(low.len() - 1);
                last_close_index = Some(close.len() - 1);
            }
            else {
                if let Some(idx) = last_high_index {
                    if high[idx] < entry.price {
                        high[idx] = entry.price; // Update the last price if it's less than entry.price
                    }
                }
                if let Some(idx) = last_low_index {
                    if low[idx] > entry.price {
                        low[idx] = entry.price; // Update the last price if it's less than entry.price
                    }
                }
                if let Some(idx) = last_low_index {
                    close[idx] = entry.price; // Update the last price if it's less than entry.price
                }
            }
        }
    }
    for i in 0..open.len() {
        output.push(OutputData {open: open[i],high: high[i],low: low[i],close: close[i]});
    }

    let json_data = serde_json::to_string(&output).expect("Failed to serialize");

    let mut write_file = File::create("output.json").unwrap();
    write_file.write_all(json_data.as_bytes()).expect("Failed to write to file");
}
