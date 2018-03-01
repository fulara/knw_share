# a litlte bit about syntax.

Variables are declared using 'let' keyword.
Type declaration follows variable name.
```rust
//creates variable named x of type i32
let x : i32 = 0;
```

```rust
//in rust variables are const by default.
#fn main() {
let x : i32 = 0;
x = 1;

println!("x is {}", x);
#}
```

```rust
//mark it as mut to achieve mutability.
#fn main() {
let mut x : i32 = 0;
x = 1;

println!("x is {}", x);
#}
```

Supports type inference. Uses [Handley-Milner](https://en.wikipedia.org/wiki/Hindley%E2%80%93Milner_type_system) type system.
```rust
//creates variablce named x pointing to a static string "abcd"
let x = "abcd"
```

functions are declared as follows:

```rust
fn fn_name(arg : arg_type) -> return_type {
  //body
  return_type {}
}
```
functions/method in rust each has to stand on their own - so if method compiles once, it will always compile - unlike C++ where method can compile depending on arguments passed in.
In rust there is no function/method overload.

Data and Behavior are declared separately.
```rust
//declare data here
struct Foo {
    field : i32,
}

//add behavior to Foo class.
impl Foo {
	fn member_method(&self) {}	
}
```

references
```rust
//references accepts reference
fn foo(i : &i32) { println!("foo invoked {}", i);}
fn foo_mut(i : &mut i32) {println!("foo_mut invoked {}", i);}

fn main() {
    let mut i = 0;
	//need to add & in order to pass method by reference
	foo(&i);
	//need to add &mut  in order to pass method by mutable reference
	foo_mut(&mut i);
}
```


