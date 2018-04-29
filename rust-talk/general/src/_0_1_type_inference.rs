fn foo() {
    //type of vector is inferenced based on the usage.
    let mut v = Vec::new();

    v.push(0.);
}

fn foo2() {
    let force: Vec<i32> = Vec::new();
}

//unlike C++ each function has to stand on its own.
//remember these compilation error that have stacktraces of 30 lvls and go up to <memory>?
fn foo3<T>(t: T) {
    //i cant use methods from T, because I didnt put any trait requirement.
}

#[test]
fn test_it() {}
