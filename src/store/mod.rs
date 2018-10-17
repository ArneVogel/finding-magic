extern crate serde;
extern crate serde_json;

use calculator::Calculator;
use magicsquare::MagicSquare;
use serde::ser::{Serialize, SerializeMap, SerializeSeq, Serializer};
use std::collections::{HashMap, HashSet};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::fs;
// File structure
// 0;1   ;2        ;3        ;4   ;5        ;6
// n;cmax;min_value;negatives;type;functions;powers
// 3;20;0;false;semi;add;1
// 3;10;1;true;magic;addmul;1,2

pub fn save(ms: &MagicSquare) {
    let config_string: String = format!(
        "{};{};{};{};{};{}\n",
        ms.n, ms.cmax, ms.min_value, ms.negatives, ms.magicType ,ms.calc
    );
    let serialazation_string: String =
        serde_json::to_string(&ms.intermediate_results).expect("could turn intermediate results into serde_json");
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(ms.tmp_file.to_owned())
        .unwrap();
    file.write(config_string.as_bytes());
    file.write(serialazation_string.as_bytes());
    fs::rename(ms.tmp_file.to_owned(), ms.config_file.to_owned());
}

pub fn load(config_file: String) -> MagicSquare {
    let file = File::open(config_file.to_owned()).unwrap();
    let mut buffer = BufReader::new(file);
    let mut config_string = String::new();
    buffer
        .read_line(&mut config_string)
        .expect("coudnt get config_string from file");

    let options: Vec<&str> = config_string.split(";").collect();

    let negatives_allowed = options[3].contains("t");

    let mut deserialization_string = String::new();
    buffer
        .read_line(&mut deserialization_string)
        .expect("couldnt get deserialization_string from file");

    let mut intermediate_results: HashMap<String, HashSet<Vec<i64>>> = HashMap::new();

    if deserialization_string.len() > 2 {
        intermediate_results = serde_json::from_str(&deserialization_string).unwrap();
    }
    let tmp = config_file.to_owned();
    let v: Vec<&str> = tmp.split(".").collect();
    let mut prefix: String = v[0].to_owned();
    for i in 1..v.len()-2 {
        prefix.push_str(v[i]);
    }
    let suffix = v[v.len()-1];
    let solution_file: String = prefix.to_owned() + "_solutions." + &suffix.to_owned();
    let tmp_file: String = prefix.to_owned() + "_tmp." + &suffix.to_owned();

    return MagicSquare {
        n: options[0].parse::<i64>().unwrap(),
        cmax: options[1].parse::<i64>().unwrap(),
        calc: Calculator::new(options[5].to_owned(), options[6].to_owned()),
        negatives: negatives_allowed,
        magicType: options[4].to_owned().to_string(),
        min_value: options[2].parse::<i64>().unwrap(),
        config_file: config_file,
        solution_file: solution_file,
        tmp_file: tmp_file,
        intermediate_results: intermediate_results,
    };
}

#[cfg(test)]
mod tests;
