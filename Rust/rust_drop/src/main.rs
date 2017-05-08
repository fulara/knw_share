#[repr(C)]
struct Foo<'a> {
    r: &'a mut i32,
}

impl<'a> Foo<'a> {
    fn new(jakis_int: &'a mut i32) -> Foo<'a> {
        Foo {
            r: jakis_int
        }
    }

    fn metoda(&self) {}
}

impl<'a> Drop for Foo<'a> {
    fn drop(&mut self) {
        *self.r = 0;
    }
}

fn main() {
    let mut x = 5;

    println!("x {}", x);
    {
        let foo = Foo::new(&mut x);
        foo.metoda;
    }
    println!("x {}", x);
}
