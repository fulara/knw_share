extern crate rust_dep_on_err_prop_0_01;
extern crate rust_dep_on_err_prop_0_02;

//check this comp err. 
//extern crate rust_dep_on_err_prop_0_02::;

//can use the types.
fn get_001() {
    let r1 = rust_dep_on_err_prop_0_01::get();
    let r2 = rust_dep_on_err_prop_0_01::get();

    r1 + r2;
}

fn get_002() {
    let r1 = rust_dep_on_err_prop_0_02::get();
    let r2 = rust_dep_on_err_prop_0_02::get();

    r1 + r2;
}

fn mixed() {
    let r1_001 = rust_dep_on_err_prop_0_01::get();
    let r2_002 = rust_dep_on_err_prop_0_02::get();

    //uncomment me.
    //r1_001 + r2_002;
}

fn main() {
    println!("Hello, world!");
}
