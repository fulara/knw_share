fn foo() -> () {
    ()
}

fn foo2() {}

fn foo3() -> i32 {
    3
}

fn foo4() -> i32 {
    return 3;
}

fn foo5(a: i32, b: String, c: [i32; 2]) -> bool {
    true
}

//where in C like programming it would be..
//C++
//bool foo5( int a, String b, std::array<i32,2> c) {
//     return true;
//}
// i remember being tought how to read C functions, lol. read to the left until parenthesis...


fn foo6<T: Clone>(t: T) {
    t.clone();
}

struct Foo {
    a: i32,
    b: i32,
    c: i32,
}

impl Foo {
    //no CONSTRUCTORS! freedom.
    // about constructor and the mess.
    // a) it is often the case that you will see multiple constructors, different arguments.
    //    they do not give any documentation as to what they are doing really.
    // b) constructors are complex. In  almost all of the C++ panels
    //    i've been through there was question about constructors.
    //    you don't ask questions about somethings thats trivial right?
    // c) finally after 20+ years  constructors in C++ will be able to deduce templates.
    // d) multitude of make functions, simply saying that constructors are not good enough.
    // e) having free function means that you can do any setup you want before constructing.

    //note usage of Self keyword - no need to duplicate class name.
    fn new() -> Self {
        let b = 3;
        Self {
            a: 1,
            //shortcut - if name is already in scope then take value from that no need for : X
            b,
            c: 3,
        }
    }

    fn memberwise_clone(&self) -> Self {
        Self { ..*self }
    }

    fn partial_memberwise_clone(&self) -> Self {
        Self { a: 10, ..*self }
    }

    //clear distrinction between mutable methods and mutable.
    //const methods are messy in C++.
    // for starters in C++-
    //    almost no one gets them rights, because they are about bitwise mutability ( references. )
    fn mutating(&mut self) {
        //to access member variables you have to use syntx self. as in:
        println!("{}", self.a);

        //to invoke member method you have to use self as well.
        self.reading();
        //to invoke static method you have to use Self keyword.
        Self::static_method();
        //by the way self, Self are great,

        //in c like languages you usually will see this. on name conflicts.
    }

    fn reading(&self) {
        //in C like language you probabyl all seen code like this.
    }

    //no bullshit with static keyword.
    fn static_method() {}
}

// non invasive trait(interface) implementations
// how many times you've had struct that you wanted to pass
// somewhere and you had to pollute your class with some impl?
// or maybe you had to do silly adapter? bridge?
impl Clone for Foo {
    //Self keyword!
    //self keyword!
    fn clone(&self) -> Self {
        unimplemented!()
    }

    fn clone_from(&mut self, source: &Self) {
        unimplemented!()
    }
}


