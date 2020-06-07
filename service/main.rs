use std::fs::File;
use std::io::prelude::*;
use serialize::{Decodable, Encodable, json};


// fn main() -> std::io::Result<()> {
//     let mut file = File::create("foo.txt")?;
//     file.write_all(b"Hello, world!")?;
//     Ok(())
// }


fn main() -> std::io::Result<()> {
    let mut file = File::open("sns_event.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("{}",contents);
    Ok(())
}