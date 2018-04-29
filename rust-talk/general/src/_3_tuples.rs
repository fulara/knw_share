
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
    println!("sometimes you are just too lazy. {:?}", ("str", 1, Boo { i : 10}))
}
