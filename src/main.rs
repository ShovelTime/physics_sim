extern crate json;
extern crate rend3;
extern crate chrono;
use chrono::Date;
use std::path;
use std::fs;

enum PEngineState{
    Unloaded,
    Loading,
    Loaded,
    Paused,
    Running,
    Stopped,

}

struct PEngine
{
    timestamp : chrono::Duration,
    simticks : i64,
    worldstate : PEngineState,
    time_step : f64, //Time increment in seconds per tick
    loadedworld : World,

}

struct World
{
    name : String,
    bodylist : Vec<Body>,
    updatepkg : Vec<Body>,

}

struct Body
{
    name : String,
    radius : f32,
    mass : f64,
    velocity : Vec<f64>,
    position : Vec<f64>,


}

fn main() {
    let mut currpath = std::env::current_dir().unwrap();
    currpath.push("Sol.json");
    let path_str = currpath.as_os_str().to_str().unwrap();
    println!("{}" ,&path_str);
    let file_dat = fs::read_to_string(&currpath).unwrap();
    let parse_res = json::parse(&file_dat).unwrap();
    
}
