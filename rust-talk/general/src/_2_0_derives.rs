

// derives os type of proc macro and its all about code generation.
// compiler is able to generate implementation of traits for types based on recipes.

#[derive(Debug)]
struct Foo {
    a: i32,
}

#[test]
fn foo_print() {
    println!("{:?}", Foo { a: 3 })
}


//you want your type to be  key in a map

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Foo2 {
    a: i32,
}

#[test]
fn btree_map() {
    use std::collections::BTreeMap;

    let mut map = BTreeMap::new();

    map.insert(Foo2 { a: 4 }, 1);
    map.insert(Foo2 { a: 3 }, 1);

    println!("{:?}", map);
}

#[derive(PartialEq, Eq, Hash, Debug)]
struct Foo4 {
    a: i32,
}

#[test]
fn hash_map() {
    use std::collections::HashMap;

    let mut map = HashMap::new();

    map.insert(Foo4 { a: 4 }, 1);
    map.insert(Foo4 { a: 3 }, 1);

    println!("{:?}", map);
}


#[derive(PartialOrd, Ord, Debug, PartialEq, Eq, Clone, Copy)]
struct Foo3 {
    a: i32,
}
