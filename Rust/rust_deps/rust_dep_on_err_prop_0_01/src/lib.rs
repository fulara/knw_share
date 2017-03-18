extern crate err_prop;

use err_prop::F64Err;

pub fn get() -> F64Err {
    F64Err::new_exact(1.0,1.0)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
