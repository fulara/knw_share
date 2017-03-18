use super::*;

fn using_public() {
    println!("i am using public.");
    public();
}

fn using_private() {
    println!("i am using private.");
    private();
}

fn creating_foo() {
    Foo {
    };
}


pub fn mod1_inner() {
    println!("mod1_inner");
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        using_public();
        using_private();
    }
}
