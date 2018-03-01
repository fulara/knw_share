# move semantics

Rust has supports for move semantics - which in principle work very similarly as the ones in C++. However unlike C++, where moved out of objects can still be valid, in Rust its a compiler error to access moved out of object.

```rust
fn consume(_v : Vec<i32>) {}

fn main() {
	let v = vec![];
	consume(v);
	
	v.len();
}
```