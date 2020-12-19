#[macro_use]
extern crate lazy_static;

use std::collections::hash_map::Keys;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use regex::Regex;

struct BagSpecs {
    map: HashMap<String, Vec<(String, u8)>>,
}

impl BagSpecs {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    fn insert(&mut self, s: &str) {
        let (key, val) = Self::parse(s);
        self.map.insert(key, val);
    }

    fn parse(line: &str) -> (String, Vec<(String, u8)>) {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"(?P<bag>.*) bags contain (?P<contained>.*)\.").unwrap();
            static ref CONTAINED_RE: Regex =
                Regex::new(r"(?P<count>\d+) (?P<color>.*) bags?").unwrap();
        }
        let caps = RE.captures(&line).unwrap();
        let bag = caps["bag"].to_owned();
        let contained: Vec<(String, u8)> = if &caps["contained"] != "no other bags" {
            caps["contained"]
                .split(", ")
                .map(|s| {
                    let caps = CONTAINED_RE.captures(s).unwrap();
                    (
                        caps["color"].to_owned(),
                        caps["count"].to_owned().parse::<u8>().unwrap(),
                    )
                })
                .collect()
        } else {
            Vec::new()
        };
        (bag, contained)
    }

    fn bags(&self) -> Keys<String, Vec<(String, u8)>> {
        self.map.keys()
    }

    fn contains(&self, enclosing: &str, contained: &str) -> bool {
        let direct = self.map.get(enclosing).unwrap();
        if direct.iter().filter(|p| p.0 == contained).next().is_some() {
            return true;
        }
        return self
            .map
            .get(enclosing)
            .unwrap()
            .iter()
            .map(|k| self.contains(&k.0, contained))
            .any(|b| b);
    }

    fn count(&self, enclosing: &str) -> u32 {
        return 1 as u32
            + self
                .map
                .get(enclosing)
                .unwrap()
                .iter()
                .map(|(bag, count)| count.to_owned() as u32 * self.count(bag))
                .sum::<u32>();
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let mut bag_specs = BagSpecs::new();
    for line in BufReader::new(File::open(filename)?)
        .lines()
        .map(|r| r.unwrap())
    {
        bag_specs.insert(&line);
    }

    let containers = bag_specs
        .bags()
        .map(|bag| bag_specs.contains(bag, "shiny gold"))
        .filter(|b| b.to_owned())
        .count();

    println!("Can contain shiny gold bag: {}", containers);
    println!("Bag Count: {}", bag_specs.count("shiny gold") - 1);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parser() {
        assert_eq!(
            BagSpecs::parse("light red bags contain 1 bright white bag, 2 muted yellow bags."),
            (
                "light red".to_owned(),
                vec![
                    ("bright white".to_owned(), 1),
                    ("muted yellow".to_owned(), 2)
                ]
            )
        );

        assert_eq!(
            BagSpecs::parse("dotted black bags contain no other bags."),
            ("dotted black".to_owned(), vec![])
        );

        assert_eq!(
            BagSpecs::parse("dark orange bags contain 3 bright white bags, 4 muted yellow bags."),
            (
                "dark orange".to_owned(),
                vec![
                    ("bright white".to_owned(), 3),
                    ("muted yellow".to_owned(), 4)
                ]
            )
        );

        for line in BufReader::new(File::open("input_01").unwrap())
            .lines()
            .map(|r| r.unwrap())
        {
            BagSpecs::parse(&line);
        }
    }
}
