#![allow(unused)]
extern crate serde;
extern crate eventual;
use eventual::{ Async, Future };
use serde::json;
use serde::json::Value;
use std::fs::File;
use std::io::{ BufReader, Read };
use std::path::Path;

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

    let value: Value = json::from_str(&sbuf).unwrap();
    let coords = Arc::new(value.find("coordinates").unwrap().as_array().unwrap());
    let x = Future::spawn(|| coords.iter().fold(0f64, |mut a,b| { a += read_coord_value(&b, "x"); a }));
    let y = Future::spawn(|| coords.iter().fold(0f64, |mut a,b| { a += read_coord_value(&b, "y"); a }));
    let z = Future::spawn(|| coords.iter().fold(0f64, |mut a,b| { a += read_coord_value(&b, "z"); a }));

    println!("x: {}; y: {}; z: {}",
             x.await().unwrap(),
             y.await().unwrap(),
             z.await().unwrap());
}

#[inline(always)]
fn read_coord_value(v: &Value, axis: &str) -> f64 {
    v.find(axis).and_then(|v| v.as_f64()).unwrap()
}

fn open_file<I: Iterator<Item=String>>(mut args: I) -> Option<File> {
    args.nth(1).and_then(|f| File::open(&Path::new(&f)).ok())
}
