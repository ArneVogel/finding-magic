use super::*;

use std::collections::HashMap;
use validator::*;
#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}

#[test]
fn check_for_solutions_test() {
    let v: Validator = Validator::new(3, "magic".to_string());
    let mut map: HashMap<String, HashSet<Vec<u32>>> = HashMap::new();
    let mut set: HashSet<Vec<u32>> = HashSet::new();
    set.insert(vec![7,6,2]);
    set.insert(vec![9,5,1]);
    set.insert(vec![8,4,3]);
    set.insert(vec![9,4,2]);
    set.insert(vec![7,5,3]);
    set.insert(vec![8,6,1]);
    set.insert(vec![8,5,2]);
    set.insert(vec![6,5,4]);
    map.insert("shouldnt matter".to_string(), set.to_owned());
    assert_eq!(v.check_for_solution(map.to_owned(), 3), false); 
    assert_eq!(v.check_for_solution(map.to_owned(), 9), true); 
}


#[test]
fn test_create_amounts() {
    let mut set: HashSet<Vec<u32>> = HashSet::new();
    set.insert(vec![1, 2, 3]);
    let mut map: HashMap<u32, u32> = HashMap::new();
    map.insert(1, 1);
    map.insert(2, 1);
    map.insert(3, 1);

    let mut set1: HashSet<Vec<u32>> = HashSet::new();
    let mut map1: HashMap<u32, u32> = HashMap::new();

    set1.insert(vec![1, 2, 3]);
    set1.insert(vec![1, 2, 4]);
    set1.insert(vec![1, 3, 5]);
    map1.insert(1, 3);
    map1.insert(2, 2);
    map1.insert(3, 2);
    map1.insert(4, 1);
    map1.insert(5, 1);
    assert_eq!(create_amounts(set), map);
    assert_eq!(create_amounts(set1), map1);
}
#[test]
fn test_get_key_smallerequal() {
    let mut map: HashMap<u32, u32> = HashMap::new();
    map.insert(2, 6);
    map.insert(5, 6);
    map.insert(20, 6);
    assert_eq!(get_key_smallerequal(map.to_owned(), 3).unwrap(), 2);
    assert_eq!(get_key_smallerequal(map.to_owned(), 19).unwrap(), 5);
    assert_eq!(get_key_smallerequal(map.to_owned(), 200).unwrap(), 20);
    assert_eq!(get_key_smallerequal(map.to_owned(), 20).unwrap(), 20);
    assert_eq!(get_key_smallerequal(map.to_owned(), 0), None);
}

#[test]
fn test_min_req_check() {
    let mut req: HashMap<u32, u32> = HashMap::new();
    let mut amounts: HashMap<u32, u32> = HashMap::new();

    req = min_req_creator(3, false);

    assert_eq!(min_req_check(req.to_owned(), amounts.to_owned()), false);
    amounts.insert(1, 3);
    amounts.insert(2, 2);
    assert_eq!(min_req_check(req.to_owned(), amounts.to_owned()), false);
    amounts.insert(3, 3);
    amounts.insert(4, 2);
    amounts.insert(5, 4);
    amounts.insert(6, 2);
    amounts.insert(7, 3);
    assert_eq!(min_req_check(req.to_owned(), amounts.to_owned()), false);
    amounts.insert(8, 2);
    assert_eq!(min_req_check(req.to_owned(), amounts.to_owned()), false);
    amounts.insert(9, 3);
    assert_eq!(min_req_check(req.to_owned(), amounts.to_owned()), true);
    amounts.insert(10, 3);
    amounts.insert(11, 20);
    assert_eq!(min_req_check(req.to_owned(), amounts.to_owned()), true);
}

#[test]
fn test_min_req_creator() {
    let mut semi3: HashMap<u32, u32> = HashMap::new();
    let mut magic3: HashMap<u32, u32> = HashMap::new();
    let mut semi4: HashMap<u32, u32> = HashMap::new();
    let mut magic4: HashMap<u32, u32> = HashMap::new();
    let mut semi5: HashMap<u32, u32> = HashMap::new();
    let mut magic5: HashMap<u32, u32> = HashMap::new();

    // 2 2 2
    // 2 2 2
    // 2 2 2
    semi3.insert(2, 9);

    // 3 2 3
    // 2 4 2
    // 3 2 3
    magic3.insert(3, 4);
    magic3.insert(4, 1);
    magic3.insert(2, 4);
    assert_eq!(magic3, min_req_creator(3, false));
    assert_eq!(semi3, min_req_creator(3, true));

    // 2 2 2 2
    // 2 2 2 2
    // 2 2 2 2
    // 2 2 2 2
    //
    // 3 2 2 3
    // 2 3 3 2
    // 2 3 3 2
    // 3 2 2 3
    semi4.insert(2, 16);
    magic4.insert(3, 8);
    magic4.insert(2, 8);
    assert_eq!(magic4, min_req_creator(4, false));
    assert_eq!(semi4, min_req_creator(4, true));

    // 2 2 2 2 2
    // 2 2 2 2 2
    // 2 2 2 2 2
    // 2 2 2 2 2
    // 2 2 2 2 2
    //
    // 3 2 2 2 3
    // 2 3 2 3 2
    // 2 2 4 2 2
    // 2 3 2 3 2
    // 3 2 2 2 3
    semi5.insert(2, 25);
    magic5.insert(4, 1);
    magic5.insert(3, 8);
    magic5.insert(2, 16);
    assert_eq!(magic5, min_req_creator(5, false));
    assert_eq!(semi5, min_req_creator(5, true));
}

#[test]
fn test_comb_has_duplicates() {
    let mut v: Vec<Vec<u32>> = Vec::new();
    v.push(vec![1,2,3]);
    v.push(vec![4,5,6]);
    assert_eq!(comb_has_duplicates(v.to_owned()), false);
    v.push(vec![2]);
    assert_eq!(comb_has_duplicates(v.to_owned()), true);
}
