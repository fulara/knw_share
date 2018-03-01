# safety - references.
On this talk I will present my favorites features in the language.  
Lets talk about this mentioned beforehand safety..  
## Pointer safety.

Probably my most favorite feature in the language is... No more pointers!
Meaning.. in most of the code you wont see any pointers, just references ( C++ like references).
And whats the main advantage of references? Well, if they are given to you then it means that the pointed to object exists!
Yep, there is no null pointer int he language at all.

Lets look at case in C.

```C++
void foo(std::unique_ptr<int> p) {
    //so in c++ this may be.
	// yey! we were succesful in modifying some pointer value!
	//p could've been null, coredump :(
	*p = 3;
}
```

```rust
//in rust Box is equvalent to C++'s std::unique_ptr
fn foo(p : Box<i32>) {
    //if we were given Box that means that its safe to modify it.
    *p = 3;
}
```

Raw pointer example.

```C++
void foo(int *p) {
     //we can freely derefence the pointer without any checks :(
	 //can crash.
	 //can cause ub.
	 //can succeed ( yey ) .
     *p = 3;
}
```

```rust
fn accept_ref(r : &i32) {}

//so in order to represent optionally available pointer we need to use Option.. ( yes yes - Option instead of Optional )
//similar to: c++'s std::optional<int&>
// option is really just:
// enum Option<T> {
//      Some(T),
//      None,
// }
// will talk more about enums later.

// }
fn foo(p : Option<&i32>) {
     //*p = 3 wont compile :( 
	 
	 //we can print it without checking!
	 //btw: we would get compiler error here if type stored in option would be not printable
	 // :( towards java - if you dont implement toString() then ... you probably wont get the behavior you expected.
	 println!("print: {:?}", p);
	 
	 //in order to access the value we need to..
	 
	 //yep, no parenthesis required.
	 if p.is_some() {
		//specified type for *clarity* 
		//so we used unwrap here. using unwrap is usually considered bad practice, sometimes unavoidable tho.
		//we've checked here is_some() - so we know that we'll be fine.
	    let v : &i32 = p.unwrap();
		accept_ref(v);
	}
	
	//but unwrap is ugly.. the above can be rewritten as.. if let!
	if let Some(p) = p {
		//we did not need to do any unwraping - we are definitely sure now that p is initialized!
	    accept_ref(p);
	}
	
	//note: if let is really syntactic sugar for match:
	//lets look at match equivalent.
	//match is somewhat similar to switch case, however much more powerful. uses pattern matching.
	match p {
		//match Some(_) pattern.
		Some(p) => {
		    accet_ref(p);
		}
		
		// we did not add else to our if let so match all case is empty.
		_ => {
		}
	}
}
```

C++ example:
```rust 
//lets assumee that we are working with c++17 here.
void foo(p : std::option<int>) {
    //we dont need to do any checking.. we can just use it.
	//c'mon - someone checked that option above..
	*p = 3;
	
	//however we'd rather check if its present
	if (p) {
	   //note how we still are using the same method to access p's content :( it still not safe! what if we were given 2 optionals? typo could lead to problems here.
	   *p = 3;
	}
	
    //no other choice to access optional' content
}
```

Similarly to C++ once you've checked for validity of the parameters you can pass either reference or optional further in.
However unlike C++'s optionals, in Rust you are given much more safer ways to access Option contents.
