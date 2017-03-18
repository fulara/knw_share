
fn pick_ref<'a>(l : &'a String, r : &'a String, choice : i32) -> &'a String {
    if choice % 2 == 0 {
        l
    } else {
        r
    }
}


fn main() {
    {
        let l1 = String::new();
        let l2 = String::new();
        let c = pick_ref(&l1, &l2, 0);

        let c2 = pick_ref(&l2, &String::new(), 0);
    }

    println!("Hello, world!");
}
