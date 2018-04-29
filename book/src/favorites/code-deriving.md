# code deriving.
A small bit about code generation - or more specifically about deriving.


Rust allow user to define recipes, that will be used to generate code.
Can be used for..
* code generation - think C++ metaclasses.
* linting - check clippy.

Why I am talking about this? Well. lets look at most common case.

Imagine having a simple struct.
```rust
struct Mine {
    f : String,
    a : i32,
	d : f64,
}
```

And you'd like to... make this struct printable. you could do something like...
```rust
fn print_mine(m : &Mine) {
	//this of course has multitude of problems, for one - lets say we add something to Mine. and this method is no longer correct.
	println!("Mine {{ {} }}", m.f, m.a, m.d);
}
```

But in Rust you can...

```rust
#[derive(Debug)]
struct Mine {
    f : String,
    a : i32,
	d : f64,
}

fn print_mine(m : &Mine) {
    //:? is a special instruction for println to use Debug.
	println!("{:?}", m);
}

fn main() {
	print_mine(&Mine { f : String::from("some_string"), a : 1, d : 1.0}); 
}
```

using derive allows you to easily assert your classes.

```rust
#[derive(Debug, PartialEq)]
struct MyClass {
	field : i32
}

fn main() {
    let mc1 = MyClass { field : 2 };
	let mc2 = MyClass { field : 3};
	assert_eq!(mc1,mc2);
}
```


Derive is procedural macro which allows you to implement set of functionalities.
It generates code during compilation time - so has no overhead on runtime.
Really what it does is just implement a trait..
You can implement your own derive and its pretty simple - I've done it myself. 

Rust by default provides range of derives.. some that come to mind are:
`PartialEq, Debug, Hash, Clone, Copy, PartialOrd`

Want to make your type as a key in map? just add PartialOrd or Hash to it!

More advanced case.
One of the best looking cases of proc_macros I've seen is [rocket library](https://rocket.rs/guide/getting-started/#hello-world).

My colleague implemented for fun what we think is proper FIX library  using proc_macros.
In that library you specified FIX messages as follows:


```rust
#[derive(FixSerializable)]
#[MsgType("A")]
struct Logon {
	#[id = "9"]
	seq : u64,

	#[id = "56"]
	target : &str;
}

//after defining method as above, following works:
fn test_it() {
    //i've omited some of the code here for example checksum and body len.
	assert_eq!("FIX4.2|35=A|9=1|56=bla", serialize(Logon { seq : 1, target : "bla" }));
}

```

I've added derive that implement my trait for enums. which looked something like..
```rust
trait SmartEnum {
    //returns how many variants there are in that enum.
    const SIZE : usize;
	
	//get an array of all possible variables in enum
	fn values() -> &[SmartEnum];
	fn from_i32() -> Option<SmartEnum>;
}

//usage was:

//now i can use from_i32  values() and size for E enum!
#[derive(SmartEnum)]
enum E {
	A,
	B,
	C,
}

```
