use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;


#[derive(Debug, Clone)]
struct Day<'a> {
    date: &'a str,
    logs: Vec<Log<'a>>,
}

impl<'a> Day<'a> {
    fn new(str: &str) -> Day {
        let mut d: Vec<&str> = str.split("\n").filter(|s| !s.is_empty()).collect();

        Day {
            date: d.remove(0),
            logs: d.iter().map(|s| Log::new(s)).collect(),
        }
    }
}

#[derive(Debug, Clone)]
struct Log<'a> {
    time: f32,
    disc: &'a str,
    cat: &'a str,
    name: &'a str,
}

impl<'a> Log<'a> {
    fn new(str: &str) -> Log {
        let v: Vec<&str> = str.split_whitespace().collect();

        Log {
            time: f32::from_str(v[0]).unwrap(),
            disc: v[1],
            cat: v[2],
            name: v[3],
        }
    }
}

fn main() {
    parse_file();
}

fn parse_file() {
    let mut file = File::open("../timetrack.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let days: Vec<Day> = contents
        .split("- ")
        .filter(|s| !s.is_empty())
        .map(|d| Day::new(d))
        .collect();

    println!("{:?}", days);
}
