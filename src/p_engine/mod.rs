pub enum PEngineState{
    Unloaded,
    Loading,
    Loaded,
    Paused,
    Running,
    Stopped,

}

pub struct PEngine
{
    timestamp : chrono::Duration,
    simticks : i64, //tracker of how many ticks passed in simulation
    worldstate : PEngineState,
    time_step : f64, //Time increment in seconds per tick
    loadedworld : World,

}

pub struct World
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