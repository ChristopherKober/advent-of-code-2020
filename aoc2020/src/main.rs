extern crate clap;
use clap::{Arg, App};

fn main() {
    let matches = App::new("Advent Of Code 2020 Helper")
    .version("0.1")
    .author("Christopher Kober")
    .about("github.com/ChristopherKober")
    .arg(Arg::with_name("day")
        .short("d")
        .long("day")
        .value_name("NUMBER")
        .help("Sets the day/challenge to solve")
        .takes_value(true)
        .required(true))
    .arg(Arg::with_name("part")
        .short("p")
        .long("part")
        .value_name("NUMBER")
        .help("Sets the part of the specified day's challenge")
        .takes_value(true)
        .default_value("1"))
    .arg(Arg::with_name("file")
         .short("f")
         .long("file")
         .value_name("FILE")
         .help("Sets the input file to help solving")
         .takes_value(true)
         .multiple(true))
    .get_matches();

    let day = matches.value_of("day").unwrap();
    let part = matches.value_of("part").unwrap();
    let files = matches.values_of("file").unwrap_or_default();

    println!("Calculating answer for day {}, part {}...", day, part);
}
