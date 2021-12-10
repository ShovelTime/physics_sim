extern crate json;
extern crate rend3;
extern crate pollster;
extern crate winit;
extern crate chrono;

pub mod math;
pub mod render;
pub mod p_engine;
pub mod constants;


use chrono::Date;
use chrono::DateTime;
use chrono::Datelike;
use std::path;
use std::fs;
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};



fn main() {
    let mut currpath = std::env::current_dir().unwrap();
    currpath.push("Sol.json");
    let mut phys_engine = p_engine::PEngine::default();

    let file_dat = fs::read_to_string(&currpath).unwrap();
    let parse_res = json::parse(&file_dat).unwrap();

    let time = &parse_res["World"]["date"].to_string();
    let p_name = &parse_res["World"]["name"].to_string();
    let split_time = time.split('-');

    let bodies = &parse_res["World"]["Bodies"];
    let body_iter = bodies.members();

    phys_engine.worldstate = p_engine::PEngineState::Loading;
    phys_engine.world.name = p_name.to_string();
    for members in body_iter
    {
        phys_engine.world.bodylist.push(p_engine::Body
            {
                name : members["name"].to_string(),
                mass : members["mass"].as_f64().unwrap(),
                radius : members["radius"].as_f32().unwrap(),
                position : math::vec::Vec3::new(members["position"][0].as_f64().unwrap(), members["position"][1].as_f64().unwrap(), members["position"][1].as_f64().unwrap()),
                velocity : math::vec::Vec3::new(members["velocity"][0].as_f64().unwrap(), members["velocity"][1].as_f64().unwrap(), members["velocity"][1].as_f64().unwrap())

            });
    }
    phys_engine.worldstate = p_engine::PEngineState::Loaded;

    let (p_thread, r_thread) = init(phys_engine);

    p_thread.join().unwrap();
    r_thread.join().unwrap();
    return;

    
   

    //timestamp = chrono::DateTime::from_str(&time.as_str())
    //let time = parse_res.
    //let mut p_engine = PEngine;      
}
fn init(engine_state : p_engine::PEngine) -> (std::thread::JoinHandle<()>, std::thread::JoinHandle<()>)
{
    let (bodytx, bodyrx) : ( Sender<Vec<p_engine::Body>> , Receiver<Vec<p_engine::Body>> ) = mpsc::channel();
    let phys_thread = std::thread::spawn(move || {
        p_loop(engine_state , bodytx)
        
    });
    let rend_thread = std::thread::spawn(move || {
        //render::init_Render(bodyrx);
        println!("Render thread sorta started lmao");
        #[allow(while_true)]
        while true
        {
            bodyrx.recv().expect("Okay this is ridiculous how did we fail to WAIT for a value");
            std::thread::sleep(std::time::Duration::from_secs(3));
        }
    });
    
    return (phys_thread, rend_thread);
    
}

fn p_loop(mut engine_state : p_engine::PEngine, bodytx : std::sync::mpsc::Sender<Vec<p_engine::Body>>) 
{
    engine_state.worldstate = p_engine::PEngineState::Running;
    while engine_state.worldstate != p_engine::PEngineState::Stopped 
    {
        if engine_state.simticks == 525600
        {
            engine_state.worldstate = p_engine::PEngineState::Stopped;
        }
        engine_state.Process_Physics();
        engine_state.simticks = engine_state.simticks + 1;
        bodytx.send(engine_state.world.get_body_list_cpy().unwrap()).expect("Wait how the fuck did we fail to send");


    }
    println!("ticks ran: {0}", engine_state.simticks);

}
