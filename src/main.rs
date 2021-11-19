extern crate json;
extern crate rend3;
extern crate pollster;
extern crate winit;
extern crate chrono;

mod math;
mod render;
mod p_engine;

use chrono::Date;
use std::path;
use std::fs;



fn main() {
    let mut currpath = std::env::current_dir().unwrap();
    currpath.push("Sol.json");
    let file_dat = fs::read_to_string(&currpath).unwrap();
    let parse_res = json::parse(&file_dat).unwrap();
    //let time = parse_res.
    //let mut p_engine = PEngine;
    
}
