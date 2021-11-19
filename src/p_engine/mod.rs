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
    pub timestamp : chrono::Duration,
    simticks : i64, //tracker of how many ticks passed in simulation
    pub worldstate : PEngineState,
    pub time_step : f64, //Time increment in seconds per tick
    pub loadedworld : World,

}



pub struct World
{
    pub name : String,
    pub bodylist : Vec<Body>,
    updatepkg : Vec<Body>,

}

impl World 
{
    fn get_body_list(&self) -> &Vec<Body>
    {
        return &self.bodylist;
    }

}

pub struct Body
{
   pub  name : String,
   pub radius : f32,
   pub mass : f64,
   pub velocity : Vec<f64>,
   pub position : Vec<f64>,


}