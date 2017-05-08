#[derive(Debug, PartialEq, Clone, Copy)]
struct Bar {
    a : i32,
    b : i32,
}
struct Foo {
    a : Bar,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(Bar{a : 5, b : 3}, Bar{ a : 6 , b : 2});
    }
}
