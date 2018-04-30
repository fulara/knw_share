use std::fmt::Debug;

trait TraitUsedStatically {
    const SOME_VALUE: i64;

    fn do_it();
    fn do_that(&self);
    fn create() -> Self;

    fn with_default() {}
}

trait TraitUsedDynamically {
    fn do_that(&self);
}

// each invocation for different object genertes code for that function
fn monomorphization<T : TraitUsedStatically>(t : &mut T) where T : Debug {
    let x = T::create();
    t.do_that();

    println!("t {:?}", t);
}

// one method for all types implementing that trait.
// dynamic dispatch.
// fat pointer.
// debug!
fn dynamic_dispatch(t : &mut TraitUsedDynamically) {
    t.do_that();
}

#[derive(Debug)]
struct Foo {

}

impl TraitUsedStatically for Foo {
    const SOME_VALUE: i64 = 3;

    fn do_it() {
        println!("do it.");
    }

    fn do_that(&self) {
        println!("do_that(&self)");
    }

    fn create() -> Self {
        println!("create");

        Foo {}
    }
}

impl TraitUsedDynamically for Foo {
    fn do_that(&self) {
        println!("do that_dynamic(&self)");
    }
}

#[test]
fn static_dispatch_test() {
    let mut f = Foo {};

    monomorphization(&mut f);
}

#[test]
fn dynamic_dispatch_test() {
    let mut f = Foo {};

    dynamic_dispatch(&mut f);
}
