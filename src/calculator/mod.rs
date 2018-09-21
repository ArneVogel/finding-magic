pub extern crate num_integer;

use calculator::num_integer::Integer;
use std::vec::Vec;
use std::fmt;

enum Functions {
    Add,
    Mul,
    Gcd,
    Lcm,
}

pub struct Calculator {
    functions: Vec<Functions>,
    powers: Vec<u32>,
}

impl Calculator {
    pub fn new(mut functions_string: String, mut power_string: String) -> Calculator {
        let mut functions: Vec<Functions> = Vec::new();
        functions_string = functions_string.to_lowercase();
        if functions_string.contains("add") {
            functions.push(Functions::Add);
        }
        if functions_string.contains("mul") {
            functions.push(Functions::Mul);
        }
        if functions_string.contains("gcd") {
            functions.push(Functions::Gcd);
        }
        if functions_string.contains("lcm") {
            functions.push(Functions::Lcm);
        }

        let mut powers: Vec<u32> = Vec::new();
        if power_string.contains("\n") {
            power_string.pop();
        }
        let power_strings: Vec<&str> = power_string.split(",").collect();

        for ps in power_strings.iter() {
            powers.push(ps.parse::<u32>().unwrap());
        }
        return Calculator {
            functions: functions,
            powers: powers,
        };
    }

    pub fn calculate_string(&self, numbers: Vec<u32>) -> String {
        format!("{:?}", self.calculate(numbers))
    }
    pub fn calculate(&self, numbers: Vec<u32>) -> Vec<u32> {
        if numbers.len() <= 1 {
            return numbers;
        }
        let mut results: Vec<u32> = Vec::new();
        for power in self.powers.iter() {
            for func in self.functions.iter() {
                let mut tmp: u32 = 0;
                for i in 0..numbers.len() - 1 {
                    if i == 0 {
                        match func {
                            Functions::Add => {
                                tmp = numbers[i].pow(*power) + numbers[i + 1].pow(*power)
                            }
                            Functions::Mul => {
                                tmp = numbers[i].pow(*power) * numbers[i + 1].pow(*power)
                            }
                            Functions::Gcd => tmp = numbers[i].gcd(&numbers[i + 1].pow(*power)),
                            Functions::Lcm => {
                                tmp = numbers[i].pow(*power).lcm(&numbers[i + 1].pow(*power))
                            }
                        }
                        continue;
                    }
                    match func {
                        Functions::Add => tmp += numbers[i + 1].pow(*power),
                        Functions::Mul => tmp *= numbers[i + 1].pow(*power),
                        Functions::Gcd => tmp = tmp.gcd(&numbers[i + 1].pow(*power)),
                        Functions::Lcm => tmp = tmp.lcm(&numbers[i + 1]).pow(*power),
                    }
                }
                results.push(tmp);
            }
        }
        return results;
    }
}

impl fmt::Display for Calculator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s: String = String::from("");
        for func in self.functions.iter() {
            match func {
                Functions::Add => s.push_str("add"),
                Functions::Mul => s.push_str("mul"),
                Functions::Gcd => s.push_str("gcd"),
                Functions::Lcm => s.push_str("lcm"),
            }
        }
        s.push_str(";");
        for power in self.powers.iter() {
            s.push_str(&power.to_string());
            s.push_str(",");
        }
        s.pop();
        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod tests;
