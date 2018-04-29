
fn foo() -> (i32, i32) {
    (0, 0)
}

#[test]
fn tuple_2_use() {
    let tuple = foo();

    println!("{:?}", tuple);
    println!("{:?} {:?}", tuple.0, tuple.1);
}

#[derive(Debug)]
struct Boo {
    i: i32,
}


#[test]
fn tuple_lazy_printing_lol() {
    println!("sometimes you are just too lazy. {:?}",
             ("str", 1, Boo { i: 10 }))
}


#[derive(PartialEq, Eq,PartialOrd, Ord, Debug)]
enum Color {
    Red,
    Blue,
}


#[test]
fn tuple_lazy_map() {
    use std::collections::BTreeMap;

    let mut map = BTreeMap::new();

    map.insert((Color::Red, 1), 0);
    map.insert((Color::Blue, 1), 0);
    map.insert((Color::Red, 0), 0);

    println!("{:?}", map);
}
