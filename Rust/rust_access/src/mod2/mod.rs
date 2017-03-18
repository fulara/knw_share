use ::mod1::public;
//use ::mod1::mod_inner::*;
//use ::mod1::private;
fn using_public() {
    public();
}

fn using_private() {
//    private();
}

#[cfg(test)]
mod tests {
    #[test]
    fn testing() {
    }
}
