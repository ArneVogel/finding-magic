extern crate serde;
extern crate rayon;
extern crate serde_json;
extern crate permutations;
extern crate combinations;
mod calculator;
mod magicsquare;
mod store;
mod validator;
use calculator::*;
use magicsquare::*;
use store::*;
use validator::Validator;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("No config file specified");
        return;
    }
    let mut ms: MagicSquare = load(args[1].to_owned());
    let v: Validator = Validator::new(ms.n.to_owned(), ms.magicType.to_owned());
    loop {
        println!("{}", ms.cmax);
        &v.check_for_solution(ms.intermediate_results.to_owned(), ms.cmax.to_owned());
        save(&ms);
        &ms.iteration();
    }
}
