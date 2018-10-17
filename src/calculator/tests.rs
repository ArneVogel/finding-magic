use super::*;

#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}

#[test]
fn calculator_creation() {
    let calc = Calculator::new("addmul".to_string(), "1,2".to_string());
    assert_eq!(calc.powers, vec![1, 2]);
    assert_eq!(calc.functions.len(), 2);
}

#[test]
fn calculator_result() {
    let calc = Calculator::new("addmul".to_string(), "1,2".to_string());
    assert_eq!(
        vec![
            1 + 2 + 3,
            1 * 2 * 3,
            (1_i64.pow(2)) + (2_i64.pow(2)) + (3_i64.pow(2)),
            (1_i64.pow(2)) * (2_i64.pow(2)) * (3_i64.pow(2)),
        ],
        calc.calculate(vec![1, 2, 3])
    );

    let calc2 = Calculator::new("add".to_string(), "1".to_string());
    assert_eq!(vec![1 + 2 + 3 + 4], calc2.calculate(vec![1, 2, 3, 4]));
    assert_eq!(vec![1 + 2], calc2.calculate(vec![1, 2]));
    assert_eq!(vec![1], calc2.calculate(vec![1]));
}

#[test]
fn calculator_result_string() {
    let calc = Calculator::new("add".to_string(), "1".to_string());
    assert_eq!("[6]", calc.calculate_string(vec![1, 2, 3]));
}

#[test]
fn test_to_string() {
    let calc = Calculator::new("add".to_string(), "1".to_string());
    assert_eq!(format!("{}", calc), "add;1");
    let calc2 = Calculator::new("add,mul,gcd".to_string(), "1,2,3".to_string());
    assert_eq!(format!("{}", calc2), "addmulgcd;1,2,3");
}
