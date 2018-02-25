# my favorites
On this talk I will present my favorites features in the language.
Macros!

## Rust' Macro

Rust macros are somewhat similar to C's Macros, however unlike C++ where macros are implemented using simple text substition, rust uses [hygienic macro system](https://en.wikipedia.org/wiki/Hygienic_macro) Each macro expansion happens in a distinct ‘syntax context, however these are implementation details and you are free to learn about these.

Personally I think macros are a great thing to have - unfortunatelly in C++ they never got any love from the commity, and are generally considered as a bad thing. 

in Rust macro invocations are followed by `!`. the most common example of macro would be `println`

```rust
fn main() {
    println!("hello");
}
```
In some of the further examples i will be omitting the `fn main() {`, `}` part. as in:

```rust
#fn main() {
    println!("hello");
#}
```

In some ways Rust's macro fill the role of templates in C++ ( Rust does have generics but more about that later) . Lets look more at that println. 
Since println! is a macro - it is processed during compilation.

* Macros can cause compilation failure:
```rust
#fn main() {
    // oops! i forgot to add arguments
    println!("hello {}");
#}
```
```rust
#fn main() {
    // oops! added argument but forgot to use it!
    println!("hello", 1);
#}
```

```rust
struct Foo {}

fn main() {
    // oops! I wanted to print something that is not printable!
    println!("hello {}", Foo {});
}
```

I think one of the most important macro usage in C++ is for logging. So you can extract current filename, fileline, current method(uhh..) Rust Macro system allows you to do these as well.

As you can imagine from above examples, Rust Macros can implement some logic and behave differently depending on different arguments. lets look at vec! example.
```rust 
fn main() {
	let empty_vec : Vec<i32> = vec![];
	let vec_with_some_ints : Vec<i32> = vec![1,2,3,4];
	// insert that 1 - four times!
	let vec_with_duplicates : Vec<i32> = vec![1;4];
	
	println!("empty vec {:?}", empty_vec);
	println!("vec_with_some_ints {:?}", vec_with_some_ints);
	println!("{} {:?}",stringify!(vec_with_duplicates), vec_with_duplicates);
}
```