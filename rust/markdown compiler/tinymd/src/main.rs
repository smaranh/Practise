use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

fn get_title() -> String {
    let mut the_title = String::from(env!("CARGO_PKG_NAME"));
    the_title.push_str(" (v");
    the_title.push_str(env!("CARGO_PKG_VERSION"));
    the_title.push_str("), ");
    the_title.push_str(env!("CARGO_PKG_DESCRIPTION"));
    return the_title;
}

fn parse_markdown_file(_file: &str) {
    print_short_banner();
    println!("Starting to parse {}...", _file);
    let filename = Path::new(_file);
    let file = File::open(&filename)
        .expect("Error file cannot be opened");
    let mut _ptag: bool = false; // keep track of paragraph tags
    let mut _htag: bool = false; // keep track of header tags
    let mut tokens: Vec<String> = Vec::new();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line_content: String = line.unwrap().to_string();
        let mut first_char: Vec<char> = line_content.chars().take(1).collect();
        let mut output_line = String::new();
        match first_char.pop() {
            Some('#') => {
                if _ptag {
                    _ptag = false;
                    output_line.push_str("</p>\n");
                }
                if _htag {
                    _htag = false;
                    output_line.push_str("</h1>");
                }
                _htag = true;
                output_line.push_str("\n<h1>");
                output_line.push_str(&line_content[2..]);
            },
            _ => {
                if !_ptag {
                    _ptag = true;
                    output_line.push_str("<p>");
                }
                output_line.push_str(&line_content);
            }
        }
        if _ptag {
            _ptag = false;
            output_line.push_str("</p>\n");
        }
        if _htag {
            _htag = false;
            output_line.push_str("</h1>\n");
        }
        if output_line != "<p></p>\n" {
            tokens.push(output_line);
        }
    }
    let mut output_filename = String::from(&_file[.._file.len()-3]);
    output_filename.push_str(".html");
    let mut outfile = File::create(output_filename.to_string())
        .expect("Output File could not be created");
    for line in &tokens {
        outfile.write_all(line.as_bytes())
            .expect("Line failed to write");
    }
    println!("Parsing of {} is complete", _file);
}

fn print_short_banner() {
    println!("{}", get_title());
}

fn print_long_banner() {
    print_short_banner();
    let mut the_title = String::from("\nWritten by: ");
    the_title.push_str(env!("CARGO_PKG_AUTHORS"));
    the_title.push_str("\nHomepage: ");
    the_title.push_str(env!("CARGO_PKG_HOMEPAGE"));
    the_title.push_str("\nUsage: tinymd <somefile>.md");
    println!("{}", the_title);
}

fn usage() {
    print_long_banner();
}

fn main() {
    // Collect all arguments in a vector
    let args: Vec<String> = std::env::args().collect();

    match args.len() {
        2 => parse_markdown_file(&args[1]),
        _ => {
            println!("You forgot to provide a file");
            usage();
        }
    }
}