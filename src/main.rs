use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;
use std::iter;


#[derive(Debug, Clone)]
struct Day {
    date: String,
    logs: Vec<Log>,
}

impl Day {
    fn new(str: &str) -> Day {
        let mut d: Vec<&str> = str.split("\n").filter(|s| !s.is_empty()).collect();

        Day {
            date: d.remove(0).into(),
            logs: d.iter().map(|s| Log::new(s)).collect(),
        }
    }
}

#[derive(Debug, Clone)]
struct Log {
    time: f32,
    disc: String,
    cat: String,
    name: String,
}

impl Log {
    fn new(str: &str) -> Log {
        let v: Vec<&str> = str.split_whitespace().collect();

        Log {
            time: f32::from_str(v[0]).unwrap(),
            disc: v[1].into(),
            cat: v[2].into(),
            name: v[3].into(),
        }
    }
}

fn main() {
    let days = parse_file();

    for d in days {
        let half_hours = d.logs.iter().fold(0f32, |acc, l| acc + (l.time * 2f32));
        let time_bar = iter::repeat("|")
            .take(half_hours as usize)
            .fold("".to_string(), |mut acc, c| {
                acc.push_str(c);
                acc
            });
        println!("{} [{:.1}h] {}", d.date, half_hours / 2f32, time_bar);
    }
}

fn parse_file() -> Vec<Day> {
    let mut file = File::open("../timetrack/timetrack.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    contents
        .split("- ")
        .filter(|s| !s.is_empty())
        .map(|d| Day::new(d.clone()))
        .collect::<Vec<Day>>()
}
