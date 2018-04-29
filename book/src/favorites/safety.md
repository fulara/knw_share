# safety - references.
On this talk I will present my favorites features in the language.  
Lets talk about this mentioned beforehand safety..  
## Reference safety.
Was somewhat mentioned in the previous part of the talk in move semantics.  
However the main point about Rust' safety is how language deals with references.

* Rust enforces rule: 'either multiple non-mut references or single mut reference.
What does it mean? well looking at example code:

```rust
//multiple references.
fn main() {
	let i = 0;
	
	let ref_1_i = &i;
	let ref_2_i = &i;
}
```

```rust
//However...
##![allow(unused_variables)]
fn main() {
	let mut i = 0;
	
	// borrowing mutably.
	let mut_ref = &mut i;
	// should cause compilation failure! cannot borrow if already mut borrowed.
	let ref_2_i = &i;
}
```


Why such restriction on the user?
Well imagine caes like.. 
```rust

fn main() {
	// we need to mark vet as mutable if we want to modify it contents.
    let mut vec = vec![1,2,3];
	
	//simply add 4 to the end.
	vec.push(4);
	
	//we have reference to 2nd element in the vector.
	let ref_2 = &mut vec[2];
	we are free to change it!

	*ref_2 = 33;
	
	//however we cannot modify the vec now!
	// you can imagine that this would potentially invalidate all references to these elements, this protects us from this.
	//vec.push(5);
}
```

So, in Rust its safe to store references to objects, engineer no longer have to worry about whether that reference is still valid.
Of course this requires rethinking of algorithms - in C++ you are free to do what you want, here not really.