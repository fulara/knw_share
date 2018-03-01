# lifetimes - just mention

in rust there is something like lifetimes.

```rust 
fn longer<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {

	let short = String::from("a");
	
	let result = longer(&short, "bbbb");
	println!("result {}", result);
}
```


```rust 
#fn longer<'a>(x: &'a str, y: &'a str) -> &'a str {
#    if x.len() > y.len() {
#        x
#    } else {
#        y
#    }
#}

fn main() {
   
    let result;
    { 
	    let aaaa = String::from("aaaa");   
	    result = longer(&aaaa, "bbbbb");
	}
	println!("result {}", result);
}
```