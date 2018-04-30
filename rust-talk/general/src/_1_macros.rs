//#[test]
//fn test() {
//    println!("hello {}");
//}

//#[test]
//fn format_named() {
//    println!("{v} {l} {l} {s} {w} {m}", v="VERY", l="long", s="string", w="with"/*, m="multi"*/);
//    // macros thave have fmt param reuse the same logic. so.. format! write! etc.
//}

//#[test]
//fn logger() {
//    use simple_logger;
//
//    simple_logger::init().unwrap();
//
//    trace!("hai");
//}


#[test]
fn intelligent_macro() {
    let empty_vec: Vec<i32> = vec![];
    println!("empty_vec {:?}", empty_vec);

    let vec_with_some_ints: Vec<i32> = vec![1, 2, 3, 4];
    println!("vec_with_some_ints {:?}", vec_with_some_ints);

    let vec_with_duplicates: Vec<i32> = vec![1;4];
    println!("vec_with_duplicates {:?}", vec_with_duplicates);
}


#[test]
fn own_macro_simple() {
    macro_rules! foo {
        () => { println!("foo! {}:{}", file!(), line!())};
    }

    foo!();

    //no offical function! yet ;( https://github.com/rust-lang/rfcs/issues/1743
}


#[test]
fn own_macro_match() {
    macro_rules! foo {
    (x => $e:expr) => (println!("mode X: {}", $e));
    (y => $e:expr) => (println!("mode Y: {}", $e));
    }

    foo!(x => 3);
}

//on first meeting there was talk about creating parser of
// tcp packets specifying schema of the messages in macros.
