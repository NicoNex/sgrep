use clap::Parser;
use glob::Pattern;
use rayon::prelude::*;
use regex::Regex;
use std::fs::File;
use std::io::{BufReader, Read};
use walkdir::{DirEntry, WalkDir};
use std::path::Path;

#[derive(Parser, Debug)]
#[command(about, long_about = None)]
struct Options {
	pattern: String,
	
	#[arg(default_value_t = String::from("."))]
	path: String,
	
	/// Add a glob the file names must match to be edited.
	#[arg(short, long)]
	glob: Option<String>,

	/// Treat all files as ASCII text.
	#[arg(short = 'a', long = "text")]
	text: bool,
	
	/// Print byte offset with the output lines.
	#[arg(short = 'b', long = "byte-offset")]
	byte_offset: bool,

	/// Remove the file name from the output.
	#[arg(short = 'h', long = "no-filename")]
	no_filename: bool,

	/// Remove the line numbers from the output.
	#[arg(short = 'N', long = "no-line-number")]
	no_line_number: bool,
}

fn process_file(entry: DirEntry, re: &regex::Regex, with_filename: bool, byte_offset: bool) {
	let path = entry.path();

	if let Ok(file) = File::open(&path) {
		let Ok(lines) = BufReader::new(file).lines();

		let mut lineno = 0;

		for line in reader.lines() {
			lineno += 1;

			match line {
				Ok(line_cnt) => {
					if re.is_match(&line_cnt) {
						println!("{}: {}", lineno, line_cnt);
					}
				},
				Err(_) => (),
			}

		}

		if let Err(_) = file.read_to_string(&mut cnt) {
			return;
		}

		for cap in re.find_iter(cnt) {

		}

	}
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

fn main() {
	let opts = Options::parse();
	let re = Regex::new(opts.pattern.as_str()).unwrap();
	let pattern = Pattern::new(opts.glob.as_deref().unwrap_or("*")).expect("Invalid glob pattern");
    let walker = WalkDir::new(String::from(opts.path)).into_iter();

    walker
        .filter_map(Result::ok)
        .filter(|e| pattern.matches(e.path().to_string_lossy().as_ref()))
        .filter(|e| !e.path().is_dir())
        .par_bridge()
        .for_each(|e| process_file(e, &re, !opts.no_filename, opts.byte_offset));

	println!("Hello, world!");
}
