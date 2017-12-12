use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;
use std::iter;
use std::collections::HashMap;


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

fn get_bar_str(len: usize) -> String {
    iter::repeat("|")
        .take(len)
        .fold("".to_string(), |mut acc, c| {
            acc.push_str(c);
            acc
        })
}

fn main() {
    let days = parse_file();

    let mut daily = String::new();
    let mut cats = HashMap::new();
    let mut discs = HashMap::new();

    for d in days {
        let half_hours = d.logs.iter().fold(0f32, |acc, l| acc + (l.time * 2f32));
        let time_bar = get_bar_str(half_hours as usize);

        daily += &format!("{} [{:.1}h] {}\n", d.date, half_hours / 2f32, time_bar);

        for l in d.logs {
            if !cats.contains_key(&l.cat) {
                cats.insert(l.cat, l.time);
            } else {
                let mut cat = cats[&l.cat];
                cat += l.time;
                cats.insert(l.cat, cat);
            }
            
            if !discs.contains_key(&l.disc) {
                discs.insert(l.disc, l.time);
            } else {
                let mut disc = discs[&l.disc];
                disc += l.time;
                discs.insert(l.disc, disc);
            }
        }
    }

    println!("DAILY\n{}", daily);

    println!("CATEGORIES");
    for (k, v) in cats {
        println!("{:>4} [{:>4}h] {}", k, v, get_bar_str((v * 2f32) as usize));
    }
    
    println!("\nDISCIPLINES");
    for (k, v) in discs {
        println!("{:>3} [{:>4}h] {}", k, v, get_bar_str((v * 2f32) as usize));
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
