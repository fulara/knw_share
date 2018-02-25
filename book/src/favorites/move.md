# move semantics
On this talk I will present my favorites features in the language.
move semantics

Rust has supports for move semantics - which in principle work very similarly as the ones in C++. However unlike C++, where moved out of objects can still be valid, in Rust its a compiler error to access moved out of object.

```rust
fn consume(_v : Vec<i32>) {}

fn main() {
	let v = vec![];
	consume(v);
	
	v.len();
}
```

As you can imagine this saves a lot of pain & thinking when looking at the code using moves. In C++ you can only know that some class is not being cloned somewhere if you have disabled the copy constructor. However while working with vector for example it is very easy to make mistake of forgeting & somewhere and clone the vector. 