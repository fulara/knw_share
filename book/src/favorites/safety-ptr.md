# safety - references.
On this talk I will present my favorites features in the language.  
Lets talk about this mentioned beforehand safety..  
## Pointer safety.

In Rust you dont have to deal with Pointers.

Lets compare C++ and C cases.
```C++
void foo(std::unique_ptr<int> p) {
    //can crash, can ub, or can succeed
	*p = 3;
}
```

```rust
//in rust Box is equvalent to C++'s std::unique_ptr
fn foo(p : Box<i32>) {
    //its safe!
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

// }
fn foo(p : Option<&i32>) {
     //*p = 3 wont compile!
	 
	 if p.is_some() {
	    //unwrap is bad practice
        let v : &i32 = p.unwrap();
		accept_ref(v);
	}
	
	//but unwrap is ugly.. the above can be rewritten as.. if let!
	if let Some(p) = p {
		//we did not need to do any unwraping - we are definitely sure now that p is initialized!
	    accept_ref(p);
	}
	
	//match - expanded if let.
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
```C++ 
//btw, working with C++ references:
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
