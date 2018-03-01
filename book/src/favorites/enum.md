# enums.
Enum - In Rust they are really an [algebraic data types](https://en.wikipedia.org/wiki/Algebraic_data_type).  

In Rust there is enum!
```rust
enum Enum {
   One,
   Two,
   Three,
}
```

They provide similar functionality to C++' variants, Where you can bind additional data to enum variants.

```rust	
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

fn main() {
	println!("\n1:");
	use_msg_just_c(Msg::A(1));
	println!("\n2:");
	use_msg_just_c(Msg::C(CustomData { something : String::from("super-string")}));
	
	println!("\n3:");
	msg_handler(Msg::C(CustomData { something : String::from("some-string")}));
	println!("\n4:");
	msg_handler(Msg::B(String::from("abc"), 5));
}
```

Additional info...
These can also be used similarly to C/C++ enums
```rust
// we can cast it int.
fn print_as_int(e : Enum) {
     println!("e {}", e as i32);
}

fn switch_on(e : Enum) {
   match e {
       Enum::One => { println!("one"); }
	   Enum::Two => { println!("two"); }
	   Enum::Three => { println!("three"); }
   }
}

//cannot cast from i32 to Enum.
//casting integers to enum types in C++ is one of the pitfals in the language.
// in C++ you can always have enum whose value is outside  of the **allowed** range.
// hence in Rust we simply return option! - and have no other choice.
fn from_int(i : i32) -> Option<Enum> {
    match i {
	    1 => Some(One),
		2 => Some(Two),
		3 => Some(Three), 
		// in all other cases we return None - similar to boost::none 
		_ => None,
	}
}
```
