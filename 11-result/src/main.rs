use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};

// return ()
fn main() -> Result<(), io::Error> {
    open_file_1();
    open_file_2();

    let s = read_string_1();
    if let Ok(string) = s {
        println!("{}", string);
    }

    let s = read_string_2()?;
    println!("{}", s);

    let s = fs::read_to_string("hello.txt")?;
    println!("{}", s);

    Ok(())
}

fn open_file_1() -> File {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Proble creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
    f
}

fn open_file_2() -> File {
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
    f
}

fn read_string_1() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_string_2() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}
