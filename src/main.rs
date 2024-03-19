use clap::{App, Arg};
use indicatif::{ProgressBar, ProgressStyle};
use serde_json::{Value, json};
use std::fs::File;
use std::io::{self, Read, Write};

fn main() -> io::Result<()> {
    let matches = App::new("JSON Chunker")
        .version("1.0")
        .about("Splits large JSON files into chunks")
        .arg(Arg::with_name("input")
             .short("i")
             .long("input")
             .value_name("FILE")
             .help("Sets the input file to use")
             .takes_value(true)
             .required(true))
        .arg(Arg::with_name("output")
             .short("o")
             .long("output")
             .value_name("DIRECTORY")
             .help("Sets the output directory for chunked files")
             .takes_value(true)
             .required(true))
        .arg(Arg::with_name("chunks")
             .short("c")
             .long("chunks")
             .value_name("NUMBER")
             .help("Number of chunks")
             .takes_value(true)
             .required(true))
        .get_matches();

    let input_path = matches.value_of("input").unwrap();
    let output_dir = matches.value_of("output").unwrap();
    let chunks: usize = matches.value_of("chunks").unwrap().parse().expect("Chunks must be a number");

    println!("Input file: {}", input_path);
    println!("Output directory: {}", output_dir);
    println!("Number of chunks: {}", chunks);

    let mut file = File::open(input_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("File read into memory.");

    let v: Value = serde_json::from_str(&contents).expect("Failed to parse JSON.");
    if let Some(data_array) = v["data"].as_array() {
        println!("JSON 'data' array length: {}", data_array.len());

        if data_array.is_empty() {
            println!("The 'data' array is empty. No chunks to create.");
            return Ok(());
        }

        let chunk_size = if data_array.len() / chunks == 0 { 1 } else { data_array.len() / chunks };
        println!("Chunk size (number of elements per chunk): {}", chunk_size);

        let progress_bar = ProgressBar::new(chunks as u64);
        progress_bar.set_style(ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} ({eta})")
            .progress_chars("#>-"));

        for i in 0..chunks {
            let end = if i == chunks - 1 || (i + 1) * chunk_size > data_array.len() {
                data_array.len()
            } else {
                (i + 1) * chunk_size
            };
            let chunk = &data_array[i * chunk_size..end];
            let chunked_json = json!({ "data": chunk });
            let chunk_str = serde_json::to_string(&chunked_json)?;

            let output_file_path = format!("{}/chunk_{}.json", output_dir, i);
            let mut output_file = File::create(&output_file_path)?;
            output_file.write_all(chunk_str.as_bytes())?;

            println!("Chunk {} written to: {}", i, output_file_path);
            progress_bar.inc(1); // Update progress bar
        }

        progress_bar.finish_with_message("Chunking complete.");
    } else {
        println!("The JSON does not contain a 'data' array. Please check the JSON structure.");
    }

    Ok(())
}

