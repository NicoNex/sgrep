use clap::Parser;
use glob::Pattern;
use rayon::prelude::*;
use regex::Regex;
// use std::fs::File;
// use std::io::{self, Read, Write};
use walkdir::{DirEntry, WalkDir};

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
	no_filename: bool,
}

fn main() {
	let opts = Options::parse();
	let re = Regex::new(opts.pattern.as_str()).unwrap();
	let pattern = Pattern::new(opts.glob.as_deref().unwrap_or("*")).expect("Invalid glob pattern");
    let walker = WalkDir::new(String::from(opts.path)).into_iter();

    walker
        .filter_map(Result::ok)
        .filter(|e| !e.path().is_dir())
        .par_bridge()
        .for_each(|e| process_file(e, &re, &opts.replacement, opts.verbose, opts.to_stdout));

	println!("Hello, world!");
}
