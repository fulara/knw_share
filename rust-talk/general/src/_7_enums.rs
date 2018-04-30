struct CustomData {
    something : String,
}

enum Msg {
    //so A carries int!
    A(i32),
    //B carries
    B(String, i32),
    //C carries user defined struct.
    C(CustomData),
    //D is just D..
    D,
}

//However there is language support allowing you to match on these.
fn msg_handler(m : Msg) {
    match m {
        Msg::A(i) => println!("got A {}", i),
        Msg::B(txt, i) => println!("got B txt: {}, i {}", txt, i),
        Msg::C(data) => println!("got C {}", data.something),
        Msg::D => println!("got D."),
        //note: because did not add _ we would get compiler error if we added another variant to enum.
    }
}

fn use_msg_just_c(m : Msg) {
    //so lets unpack Msg however we only care about C!
    match m {
        //pattern matching helps us - unpacks data into variable.
        Msg::C(custom_data) => {
            println!("yupi, got C with {}", custom_data.something);
        }

        //for anything else..
        _ => {
            println!("meh. got something i dont care about.");
        }
    }
}

#[test]
fn enums() {
    println!("\n1:");
    use_msg_just_c(Msg::A(1));
    println!("\n2:");
    use_msg_just_c(Msg::C(CustomData { something : String::from("super-string")}));

    println!("\n3:");
    msg_handler(Msg::C(CustomData { something : String::from("some-string")}));
    println!("\n4:");
    msg_handler(Msg::B(String::from("abc"), 5));
}