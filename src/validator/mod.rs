extern crate combinations;
extern crate permutations;

use combinations::combinations::Combinations;
use magicsquare::MagicSquareType;
use permutations::permutations::Permutations;
use std::collections::{HashMap, HashSet};
use rayon::prelude::*;

pub struct Validator {
    magic_type: MagicSquareType,
    min_req: HashMap<u32, u32>,
    n: u32,
}

impl Validator {
    pub fn new(n: u32, magic_type_string: String) -> Validator {
        let mut magic_type = MagicSquareType::Magic;
        let mut semi_magic = false;
        if magic_type_string.contains("semi") {
            magic_type = MagicSquareType::SemiMagic;
            semi_magic = true;
        }
        Validator {
            n: n,
            magic_type: magic_type,
            min_req: min_req_creator(n, semi_magic),
        }
    }

    //checks all of intermediate results for a valid solution
    pub fn check_for_solution(
        &self,
        intermediate_results: HashMap<String, HashSet<Vec<u32>>>,
        cmax: u32,
    ) -> bool {
        //println!("{:?}", intermediate_results);
        let mut found_atleast_one: bool = false;
        //for set in intermediate_results.iter() {
        intermediate_results.into_par_iter().for_each(|set| {
            // n + n + 2 for n vertically, n horizontally and 2 diagonal
            if set.1.len() < (self.n + self.n + 2) as usize {
                //println!("set skipped because of not enough elements");
                //continue;
                return;
            }

            //cmax has to be in atleast one of the sets, otherwise the set was checked in a
            //previous iteration
            if !set.1.iter().any(|x| x[0] == cmax) {
                //println!("set skipped because of no cmax: {}", cmax);
                //continue;
                return;
            }

            if !min_req_check(self.min_req.to_owned(), create_amounts(set.1.to_owned())) {
                //println!("set skipped because of no not satisfying min req");
                //continue;
                return;
            }

            //println!("not skipping: {:?}", set);

            let mut solutions_vec: Vec<Vec<u32>> = Vec::new();
            for item in set.1.iter() {
                solutions_vec.push(item.to_vec());
            }

            // guess n solution vectors and check if they allow for a valid magic square
            //
            // a b c
            // d e f
            // g h i
            //
            // guess the vectors abc def ghi and check if abg beh cfi gec aei exist in the
            // solutions_vec
            let mut comb =
                combinations::combinations::Combinations::new(solutions_vec, self.n as usize);
            // Iterate over all ways to arange abc def ghi
            while comb.has_next() {
                //TODO check if the combination contains one of the newly added vectors, eg
                //[1,2,3] as added but is not in comb => discard comb because it was checked
                //alreay
                //
                //this should have done it, since the last iteration should have an element
                //with value cmax
                //println!("{:?}", comb.get_combination());
                if !comb.get_combination().iter().any(|x| x[0] == cmax) {
                    comb.next();
                    continue;
                }

                if comb_has_duplicates(comb.get_combination().to_owned()) {
                    comb.next();
                    continue;
                }

                let mut comb_perm = Permutations::new(comb.get_combination());
                while comb_perm.has_next() {
                    // Iterate over all ways to arange a b c, d e f, g h i

                    let mut perm_counter: Vec<u32> = vec![0; self.n as usize];
                    let mut perm_permer: Vec<Permutations<u32>> = Vec::new();
                    for perm in comb_perm.get_permutation().iter() {
                        perm_permer.push(Permutations::new(perm.to_vec()));
                    }
                    loop {
                        //this loop goes over all permutations of the combinations and checks
                        //if its a valid solutions with the avaliable vectors

                        let mut v: Vec<Vec<u32>> = Vec::new();
                        for perm in perm_permer.iter() {
                            v.push(perm.get_permutation().to_owned());
                        }

                        //TODO make sure not mirror solutions are checked
                        //123
                        //456
                        //789
                        //
                        //987
                        //654
                        //321
                        if (&self.check_solution(v.to_owned(), set.1.to_owned())).to_owned() {
                            //found_atleast_one = true;
                        }

                        //TODO: theres some double counting going on I think
                        // the last one seems to get repeated once
                        // => change the perm method
                        //println!("{:?}, {}, {}", v, counter, same_counter);

                        for j in 0..self.n as usize {
                            if perm_permer[j].has_next() {
                                perm_permer[j].next();
                                for k in 0..j {
                                    perm_permer[k].reset();
                                }
                                break;
                            }
                        }
                        if perm_permer.iter().all(|ref mut x| !x.has_next()) {
                            break;
                        }
                    }
                    comb_perm.next();
                }
                comb.next();
            }
            //}
        });
        return found_atleast_one.to_owned();
    }

    //checks a single permutation if its a valid solution
    //for a permutation to be a valid solution the hashset of the vectors resulting in the string
    //result has to contain all the vectors of a
    //permutation, the permutation has to be sorted first
    fn check_solution(
        &self,
        permutation: Vec<Vec<u32>>,
        resulting_vecs: HashSet<Vec<u32>>,
    ) -> bool {
        let mut to_find: Vec<Vec<u32>> = Vec::new();
        let mut tv: Vec<u32> = Vec::new(); //temporary vector
        let mut all_e: Vec<u32> = Vec::new();
        for v in permutation.iter() {
            tv = v.to_owned();
            tv.sort_by(|a, b| b.cmp(a));
            to_find.push(tv);
            for i in v.iter() {
                all_e.push(i.to_owned());
            }
        }

        //all elements should be unique
        all_e.sort_by(|a, b| b.cmp(a));
        for i in 0..all_e.len() - 1 {
            if all_e[i] <= all_e[i + 1] {
                return false;
            }
        }

        //1 2 3
        //4 5 6
        //7 8 9
        for i in 0..permutation.len() {
            let mut v: Vec<u32> = Vec::new();
            for j in 0..self.n {
                v.push(permutation[j as usize][i as usize]);
            }
            v.sort_by(|a, b| b.cmp(a));
            to_find.push(v);
        }

        let mut v: Vec<u32> = Vec::new();
        let mut w: Vec<u32> = Vec::new();
        for i in 0..permutation.len() {
            v.push(permutation[i as usize][i as usize]);
            w.push(permutation[i][(self.n as usize - i) - 1 as usize]);
        }
        v.sort_by(|a, b| b.cmp(a));
        w.sort_by(|a, b| b.cmp(a));
        to_find.push(v);
        to_find.push(w);

        //println!("permutation: {:?}\n to_find: {:?}", permutation, to_find);

        if to_find.iter().all(|x| resulting_vecs.contains(x)) {
            println!("found solution:");
            for v in permutation.iter() {
                println!("{:?}", v);
            }
            println!("to find {:?}", to_find);
            return true;
        }
        return false;
    }
}

// true = satisfies min_req
// false = doesnt satisfie min req
fn min_req_check(mut min_req: HashMap<u32, u32>, amounts: HashMap<u32, u32>) -> bool {
    amounts
        .iter()
        .for_each(|s| match get_key_smallerequal(min_req.to_owned(), *s.1) {
            Some(x) => {
                if *min_req.get(&x).unwrap() == 1 {
                    min_req.remove(&x);
                } else {
                    let counter = min_req.entry(x).or_insert(2);
                    *counter -= 1;
                }
            }
            None => {}
        });
    return min_req.len() == 0;
}

fn comb_has_duplicates(comb: Vec<Vec<u32>>) -> bool {
    let mut set: HashSet<u32> = HashSet::new();
    for vec in comb.iter() {
        for v in vec.iter() {
            if set.contains(v) {
                return true;
            }
            set.insert(v.to_owned());
        }
    }
    return false;
}

fn create_amounts(set: HashSet<Vec<u32>>) -> HashMap<u32, u32> {
    let mut counter: HashMap<u32, u32> = HashMap::new();
    set.iter().for_each(|vec| {
        vec.iter().for_each(|n| {
            let c = counter.entry(*n).or_insert(0);
            *c += 1;
        });
    });
    counter
}

fn create_amounts_from_vec(vec: Vec<Vec<u32>>) -> HashMap<u32, u32> {
    let mut set: HashSet<Vec<u32>> = HashSet::new();
    for v in vec {
        set.insert(v.to_owned());
    }
    return create_amounts(set);
}

fn min_req_creator(n: u32, semi_magic: bool) -> HashMap<u32, u32> {
    let mut v: Vec<Vec<u32>> = Vec::new();
    for i in 0..n {
        v.push(vec![0; n as usize]);
    }

    for i in 0..n {
        for j in 0..n {
            v[i as usize][j as usize] += 1;
            v[j as usize][i as usize] += 1;
        }
        if !semi_magic {
            v[i as usize][i as usize] += 1;
            v[(n - i - 1) as usize][i as usize] += 1;
        }
    }

    let mut map: HashMap<u32, u32> = HashMap::new();
    v.iter().for_each(|p| {
        //println!("{:?}", p);
        p.iter().for_each(|n| {
            let counter = map.entry(*n).or_insert(0);
            *counter += 1;
        });
    });
    //println!("{:?}", map);
    return map;
}

fn get_key_smallerequal(map: HashMap<u32, u32>, n: u32) -> Option<u32> {
    let mut key: u32 = 0;
    let mut value = u32::min_value();
    map.iter().for_each(|m| {
        if *m.0 == n {
            key = *m.0;
            value = u32::max_value();
            return;
        }
        if *m.0 > value && *m.0 < n {
            value = *m.0;
            key = *m.0;
        }
    });

    if value != u32::min_value() {
        return Some(key);
    }
    return None;
}

#[cfg(test)]
mod tests;
