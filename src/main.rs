#[macro_use]
extern crate failure;
use failure::Error;
use std::io;
use std::process;

#[derive(Debug, Fail)]
enum DomainError {
    #[fail(display = "too long")]
    TooLong,
}

fn uppercase(input: String) -> Result<String, DomainError> {
    match input.len() {
        0...5 => Ok(input.to_uppercase()),
        _ => Err(DomainError::TooLong),
    }
}

fn bail_out(err: Error) -> () {
    eprintln!("{}", err);
    process::exit(1);
}

fn do_business() -> Result<String, Error> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let len = input.len();
    input.truncate(len - 1);
    let new = uppercase(input)?;
    Ok(new)
}

fn main() -> () {
    let x = do_business();
    if let Err(err) = x {
        bail_out(err);
    } else if let Ok(bla) = x {
        println!("New: {}", bla);
    }
}
