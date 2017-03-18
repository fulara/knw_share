use std::fs::File;
use std::io::prelude::*;

enum MyErrors {
    OpenFail,
    Whatever,
}

fn open_file() -> std::io::Result<File> {
    File::open("foo.txt")
}

fn open_file_and_sth() -> std::io::Result<File> {
    let f = open_file()?;

    println!("ok opened file.");

    Ok(f)
}

fn execute_op() -> Result<(), MyErrors> {
    match open_file_and_sth() {
        Ok(f) => {
            println!("ok i got file.. i can do something with it.");
            use_file(&f);
            Ok(()) 
        },
        _ => Err(MyErrors::OpenFail),
    }
}

fn use_file(f : &File) {
}

fn open_file_use() {
    match execute_op() {
        Ok(_) => println!("op succesful."),
        Err(e) => {
            println!("op failed. getting specific err.");
            match e {
                MyErrors::Whatever => {
                    println!("got whatever.");
                }
                MyErrors::OpenFail => {
                    println!("ok open file failed");
                }
            }
        }
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
