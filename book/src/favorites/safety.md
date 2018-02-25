# safety - references.
On this talk I will present my favorites features in the language.  
Lets talk about this mentioned beforehand safety..  
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
	
	let mut_ref = &mut i;
	let ref_2_i = &i;
}
```
