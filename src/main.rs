extern crate serde;
extern crate serde_json;
extern crate rend3;
extern crate pollster;
extern crate winit;
extern crate chrono;

pub mod math;
pub mod render;
pub mod p_engine;
pub mod constants;
pub mod ch_com;

use std::io;
use chrono::{NaiveDateTime};
use std::fs;
use serde_json::{Value};
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::mpsc::{SyncSender, Receiver, Sender};

#[derive(PartialEq)]
pub enum Inloopcmd
{
    GetSimTicks,
    GetBodyList,
    GetBodyInfo(i64),
    Pause,
    Resume,
    Stop,
    None,
}

#[derive(PartialEq)]
pub enum GlobalState
{
    Loading,
    Unloaded,
    Running,
    Loaded,
    Paused,
    Stopped,
}


//static prog_state : &'static std::sync::Arc<GlobalState> = &std::sync::Arc::new(GlobalState::Unloaded);
fn main() {
    
    #[derive(Serialize, Deserialize)]
    struct JInfo
    {
        name : String,
        date : String
    }

    let mut currpath = std::env::current_dir().unwrap();
    currpath.push("Sol.json");
    println!("{0}", currpath.to_str().unwrap());
    let mut phys_engine = p_engine::PEngine::default();

    let file_dat = fs::File::open(&currpath).unwrap();
    let mut parse_res : Value = serde_json::from_reader(file_dat).expect("JSON failed yo");
    let p_info : JInfo = serde_json::from_value(parse_res["Info"].take()).unwrap();

    
    
    
    let p_date = NaiveDateTime::parse_from_str(p_info.date.as_str(), "%Y-%m-%d %H:%M:%S").unwrap();


    //let bodies = ;
    //let body_list : BodyList = serde_json::from_value(bodies.take()).expect("You fucked up son");
    //**prog_state = GlobalState::Loading;
    phys_engine.worldstate = p_engine::PEngineState::Loading;
    phys_engine.world.name = p_info.name;//p_name.to_string();
    phys_engine.timestamp = p_date.timestamp();

    #[derive(Debug, Deserialize, Serialize)]
    struct JBody
    {
        name: String,
        radius: f32,
        mass: f64,
        location: [f64 ; 3],
        velocity: [f64 ; 3]
    }
    let jbody_list : Vec<JBody> = serde_json::from_value(parse_res["Bodies"].take()).unwrap();

    for members in jbody_list
    {
        let position_arr : [f64 ; 3] = members.location;
        let velocity : [f64 ; 3] = members.velocity;
        phys_engine.world.bodylist.push(p_engine::Body
            {
                bID : phys_engine.bodycount,
                name : members.name,
                mass : members.mass,
                radius : members.radius,
                position : math::vec::Vec3::new(position_arr[0], position_arr[1], position_arr[2]),
                velocity : math::vec::Vec3::new(velocity[0], velocity[1], velocity[2])

            });
            phys_engine.bodycount += 1;
    }
    
    phys_engine.worldstate = p_engine::PEngineState::Loaded;
    println!("{0}", phys_engine.bodycount);
    let (p_thread, r_thread, i_thread) = init(phys_engine);

    p_thread.join().unwrap();
    r_thread.join().unwrap();
    println!("Engine is shutting down, Press Any Keys to continue.");
    i_thread.join().unwrap();
    return;

    
   

    //timestamp = chrono::DateTime::from_str(&time.as_str())
    //let time = parse_res.
    //let mut p_engine = PEngine;      
}
fn init(engine_state : p_engine::PEngine) -> (std::thread::JoinHandle<()>, std::thread::JoinHandle<()>, std::thread::JoinHandle<()>)
{
    let (bodytx, bodyrx) : ( SyncSender<p_engine::PEngine> , Receiver<p_engine::PEngine> ) = mpsc::sync_channel(1);
    let (intx, inrx) : (SyncSender<Inloopcmd>, Receiver<Inloopcmd>) = mpsc::sync_channel(1);
    let (outx, ourx) : (Sender<String>, Receiver<String>) = mpsc::channel();
    let phys_thread = std::thread::spawn(move || {
        println!("Physics thread started");
        p_loop(engine_state , bodytx, inrx, outx)
        
    });
    let rend_thread = std::thread::spawn(move || {
        render::init_Render(bodyrx);
        println!("Render thread started");
    });
    let input_thread = std::thread::spawn(move || {
        inloop(intx,ourx)

    });

   //**prog_state = GlobalState::Running;
    
    return (phys_thread, rend_thread, input_thread);
    
}

fn p_loop(mut engine_state : p_engine::PEngine, bodytx : std::sync::mpsc::SyncSender<p_engine::PEngine>, inrx : std::sync::mpsc::Receiver<Inloopcmd>, outx : std::sync::mpsc::Sender<String>) 
{
    engine_state.worldstate = p_engine::PEngineState::Running;
    while engine_state.worldstate != p_engine::PEngineState::Stopped
    {

        let cmd;

        if engine_state.worldstate == p_engine::PEngineState::Paused
        {
            cmd = inrx.recv().unwrap_or_else(|_| // pause the thread until a response is sent for us to resume
                {
                    Inloopcmd::None
                });
        }
        else 
        {
            cmd = inrx.try_recv().unwrap_or_else(|_| 
                {
                    Inloopcmd::None
                });
        }

        match cmd {
            Inloopcmd::None => (),
            Inloopcmd::Pause => {engine_state.worldstate = p_engine::PEngineState::Paused; outx.send(format!("Simulation Paused")).unwrap()}
            Inloopcmd::Resume => {engine_state.worldstate = p_engine::PEngineState::Running; outx.send(format!("Simulation Resumed")).unwrap()}
            Inloopcmd::Stop => {
                engine_state.worldstate = p_engine::PEngineState::Stopped;
                outx.send("STOP".to_string()).unwrap()

            }
            Inloopcmd::GetBodyInfo(id) => {
                if id > engine_state.bodycount || id < 0
                {
                    outx.send("ID is too large or too small".to_string()).unwrap()
                }
                else
                {
                    outx.send(engine_state.world.get_body_list()[id as usize].stringify()).unwrap()
                }
            }
            Inloopcmd::GetBodyList => {
                let mut out = String::new();
                for body in engine_state.world.get_body_list()
                {
                    out += format!("[{0}: {1}] \n", &body.bID, &body.name).as_str();
                }
                outx.send(out).unwrap()

            }
            Inloopcmd::GetSimTicks => outx.send(format!("{}", &engine_state.simticks)).unwrap()
        }
        if engine_state.worldstate == p_engine::PEngineState::Paused || engine_state.worldstate == p_engine::PEngineState::Stopped
        {
            continue;
        }
        if engine_state.simticks >= 31540000
        {
            engine_state.worldstate = p_engine::PEngineState::Stopped;
        }
        engine_state.process_physics();
        engine_state.simticks = engine_state.simticks + 1;
        match bodytx.try_send(engine_state.clone())
        {
            Ok(success) => success,
            Err(error) => match error {
                std::sync::mpsc::TrySendError::Full(_) => continue, //this does not break the code, therefore it can continue
                std::sync::mpsc::TrySendError::Disconnected(_) => panic!("Reciever to render loop unexpectedly closed!")

            }


        };


    }
    println!("ticks ran: {0}", engine_state.simticks);
    for bodies in engine_state.world.get_body_list()
    {
        println!("{}", bodies.stringify());
    }

}
fn inloop(cmdsend : std::sync::mpsc::SyncSender<Inloopcmd>, cmdres : std::sync::mpsc::Receiver<String>)
{
    let stop_sig = false;
    let _input = std::thread::spawn(move || {
        while !&stop_sig
        {
            
            let mut response = String::new();
            let stdin = io::stdin();
            stdin.read_line(&mut response).unwrap();
            match &response[0..response.len() - 2]
            {
                "help" => println!("Available Commands:\n getsimticks: Return the current physics iteration.\n getbodylist: Return list of all bodies currently being simulated.\n getbodyinfo: returns information about a body determined by its ID on the list.\n pause: pause the simulation.\n resume: resumes the simulation.\n stop: interrupts and stops the program."),
                "getsimticks" => cmdsend.send(Inloopcmd::GetSimTicks).unwrap(),
                "getbodylist" => cmdsend.send(Inloopcmd::GetBodyList).unwrap(),
                "getbodyinfo" => {
                    let mut bid = String::new();
                    println!("Enter ID:");
                    stdin.read_line(&mut bid).unwrap();
                    let integer = bid[0..bid.len() - 2].parse::<i64>().unwrap_or_else(|_|
                        {
                            println!("failed to parse to number, defaulting to 0");
                            std::thread::sleep(std::time::Duration::from_millis(500));
                            0

                        });
                    cmdsend.send(Inloopcmd::GetBodyInfo(integer)).unwrap()
                    

                }
                "pause" => cmdsend.send(Inloopcmd::Pause).unwrap(),
                "resume" => cmdsend.send(Inloopcmd::Resume).unwrap(),
                "stop" => {cmdsend.send(Inloopcmd::Stop).unwrap(); return}
                &_ => println!("Unrecognized Command")
            }
        }
    });
    while !stop_sig
    {
        let res = match cmdres.recv()
        {
            Ok(result) => result,
            Err(_error) => return

        };

        println!("{}", res);



    }

}
