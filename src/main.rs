extern crate rustc_serialize;
use rustc_serialize::json;
use std::fs::File;
use std::io::{ BufReader, Read };
use std::path::Path;

#[derive(RustcEncodable, RustcDecodable)]
struct Coordinate {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(RustcEncodable, RustcDecodable)]
struct Wrapper {
    coordinates: Vec<Coordinate>
}

fn main() {
    let mut input = match open_file(&mut std::env::args()) {
        Some(f) => BufReader::new(f),
        None => {
            println!("File not provided.");
            return;
        }
    };

    let mut sbuf = String::new();
    input.read_to_string(&mut sbuf);

    let wrapper: Wrapper = json::decode(&sbuf).unwrap();
    let mut x = 0.0;
    let mut y = 0.0;
    let mut z = 0.0;

    for coord in &wrapper.coordinates {
        x += coord.x;
        y += coord.y;
        z += coord.z;
    }

    let len = wrapper.coordinates.len() as f64;
    println!("{}", x / len);
    println!("{}", y / len);
    println!("{}", z / len);
}

fn open_file<I: Iterator<Item=String>>(mut args: I) -> Option<File> {
    args.nth(1).and_then(|f| File::open(&Path::new(&f)).ok())
}
