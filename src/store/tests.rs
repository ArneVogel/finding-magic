use super::*;
use store;

#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}

#[test]
fn load_save() {
    let ms: MagicSquare = store::load("./src/store/test.txt".to_string());
    println!("{:?}", ms.tmp_file);
    store::save(&ms);
}
