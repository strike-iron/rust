use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

use clap::{Arg, ArgMatches, Command};
use regex::Regex;

fn create_args() -> ArgMatches {
    Command::new("grep-lite")
        .version("0.1.0")
        .about("searches for patterns")
        .arg(
            Arg::new("pattern")
                // .long("pattern")
                // .short('p')
                .help("The pattern to search for")
                .num_args(1)
                .required(true),
        )
        .arg(
            Arg::new("file")
                .long("file")
                .short('f')
                .help("File to search")
                .num_args(1)
                .required(false),
        )
        .get_matches()
}

fn get_contents_by_buffer<T>(mut buffer: T) -> Result<String, Box<dyn Error>>
where
    T: BufRead + Sized,
{
    let mut result = String::new();
    loop {
        let len = buffer.read_line(&mut result)?;
        if len == 0 {
            break;
        }
    }
    Ok(result)
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = create_args();

    let pattern = args.get_one::<String>("pattern").unwrap();
    let re = Regex::new(pattern)?;

    let default_input = "examples/grep-lite/grep-lite".to_string();
    let input = args.get_one::<String>("file").unwrap_or(&default_input);
    dbg!(input);

    let f = File::open(input)?;
    let buffer = BufReader::new(f);
    let contents = get_contents_by_buffer(buffer)?;

    let ctx_lines = 2;
    let mut tags: Vec<usize> = vec![];
    let mut ctx: Vec<Vec<(usize, String)>> = vec![];
    for (i, line) in contents.lines().enumerate() {
        let contains_substring = re.find(line);

        if contains_substring.is_some() {
            tags.push(i);
            ctx.push(Vec::with_capacity(ctx_lines * 2 + 1));
        }
    }

    if tags.is_empty() {
        println!("Unmatched!");
        return Ok(());
    }

    for (i, line) in contents.lines().enumerate() {
        for (j, tag) in tags.iter().enumerate() {
            let lower_bound = tag.saturating_sub(ctx_lines);
            let upper_bound = tag + ctx_lines;

            if i >= lower_bound && i <= upper_bound {
                ctx[j].push((i + 1, line.to_string()));
            }
        }
    }

    for local_ctx in &ctx {
        for (i, line) in local_ctx {
            println!("{i} {line:?}");
        }
    }

    Ok(())
}
