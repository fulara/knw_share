# my favorites - Traits

Traits are similar to what C++ will maybe someday get - Concepts.

```rust
trait Worker {
    fn work(&self);
}

fn work<T : Worker>(worker : T) {
    worker.work();
}

struct Someone {}
impl Worker for Someone {
    fn work(&self) {
	    println!("i am hard worker.");
	}
}

fn main() {
    work(Someone{});
}

```

Trait besides method can also have assiociated:
* static methods.
* types
* consts