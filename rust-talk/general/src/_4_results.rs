use std::fs::File;
use std::io::prelude::*;
use std::io;

fn file_operations() -> Result<File, io::Error> {
    let f = File::open("filename")?;

    //do something with file..

    Ok(f)
}

fn some_user_code() -> Result<File, io::Error> {
    let f = file_operations()?;

    //again something with file.

    Ok(f)
}

pub enum OurError {
    FileError(io::Error),
    SomethingElse,
}


fn wrapping() -> Result<i32, OurError> {
    let f = some_user_code()?;

    Ok(3)
}


impl From<io::Error> for OurError {
    fn from(io_error : io::Error) -> OurError {
        OurError::FileError(io_error)
    }
}