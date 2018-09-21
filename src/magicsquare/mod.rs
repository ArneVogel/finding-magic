use calculator::Calculator;
use std::collections::{HashMap, HashSet};
use std::vec::Vec;

pub enum MagicSquareType {
    Magic,
    SemiMagic,
}

// maybe implement MagicSquare and make a new so nothing has to be public
pub struct MagicSquare {
    pub n: u32,    // size of the square nxn
    pub cmax: u32, //the last completed max number that was completed
    pub min_value: u32,
    pub magicType: String,
    pub config_file: String,   //location of the config file
    pub solution_file: String, //location of the solutions file
    pub tmp_file: String,      //file used to write config into before deleting the old file
    pub calc: Calculator,      //generates the results
    pub negatives: bool,       //if negative values are allowed in the magic square
    pub intermediate_results: HashMap<String, HashSet<Vec<u32>>>, //results of calculation vectors mapped to a set of numbers that are able to create the calculation vector, String because serde doesnt work with Vec as keys and I cant implement Display for Vec
}

impl MagicSquare {
    pub fn iteration(&mut self) {
        if self.negatives {
            &self.iternation_negatives();
            return;
        }
        let mut numbers_vec: Vec<u32> = Vec::new();
        numbers_vec.push(self.cmax.to_owned() + 1);
        for i in (0..self.n - 1).rev() {
            numbers_vec.push(i);
        }

        let mut change: bool = false;
        loop {
            change = false;
            let mut all_unique: bool = true;
            for i in 0..self.n - 1 {
                // numbers_vec should be strictly monotonisly decreasing
                if numbers_vec[i as usize] <= numbers_vec[(i + 1) as usize] {
                    all_unique = false;
                    break;
                }
            }
            //insert the result into intermediate result
            if all_unique {
                let res: String = format!("{:?}", self.calc.calculate(numbers_vec.to_owned()));
                if self.intermediate_results.contains_key(&res) {
                    let mut set = self.intermediate_results.get(&res).unwrap().to_owned();
                    set.insert(numbers_vec.to_owned());
                    self.intermediate_results.insert(res, set);
                } else {
                    let mut n: HashSet<Vec<u32>> = HashSet::new();
                    n.insert(numbers_vec.to_owned());
                    self.intermediate_results.insert(res, n);
                }
            }

            //generate the next permutation
            for i in (1..numbers_vec.len()).rev() {
                if numbers_vec[i - 1] != 0 && numbers_vec[i] < numbers_vec[i - 1] - 1 {
                    numbers_vec[i] += 1;
                    for j in (i + 1..numbers_vec.len()) {
                        numbers_vec[j] = self.min_value;
                    }
                    change = true;
                    break;
                }
            }
            if !change {
                break;
            };
        }
        //println!("{:?}", self.intermediate_results);
        self.cmax += 1;
    }
    pub fn iternation_negatives(&mut self) {
        println!("not implemented");
    }
}
