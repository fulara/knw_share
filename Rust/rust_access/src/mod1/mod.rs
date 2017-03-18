mod mod_inner;

struct Foo {
}

fn private() {
    println!("mod1::private");
}

pub fn public() {
    println!("mod1::public");
}
