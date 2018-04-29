trait SomeTrait {
    const SOME_VALUE: i64;

    fn do_it();
    fn do_that(&self);
    fn create() -> Self;

    fn with_default() {}
}

// each invocation for different object genertes code for that function
fn monomorphization<T : SomeTrait>(t : &mut T) where T : Debug {
    let x = T::create();
    t.do_that();

    println!("t {:?}", t);
}

// one method for all types implementing that trait.
// dynamic dispatch.
// fat pointer.
// debug!
fn dynamic_dispatch(t : &mut SomeTrait) {
    let x = T::create();
    t.do_that();
}

struct Foo {

}

impl SomeTrait for Foo {
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
