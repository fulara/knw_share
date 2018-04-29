pub trait SmartEnum: Clone {
    const LENGTH: usize;

    fn values() -> &'static [Self];

    fn as_usize(&self) -> usize;
}

#[derive(SmartEnum, Clone, Debug)]
enum SomeEnum {
    A,
    B,
    C,
}

#[test]
fn smart_enum() {
    println!("values: {:?}", SomeEnum::values());
    println!("as_usize: {:?}", SomeEnum::B.as_usize());
}
