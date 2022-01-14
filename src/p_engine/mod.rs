use crate::math::vec;
use crate::math::phys::Phys;


#[derive(PartialEq, Clone)]
pub enum PEngineState{
    Unloaded,
    Loading,
    Loaded,
    Paused,
    Running,
    Stopped,

}
#[derive(Clone)]
pub struct PEngine
{
    pub bodycount : i64,
    pub timestamp : i64,
    pub simticks : i64, //tracker of how many ticks passed in simulation
    pub worldstate : PEngineState,
    pub time_step : f64, //Time increment in seconds per tick
    pub world : World,

}
impl PEngine
{
    pub fn process_physics(&mut self)
    {
        /*
        let mut newbodylist = Vec::<Body>::new();
        
        for bodies in self.world.get_body_list().iter() {
            let accel1 = self.accel_loop(&bodies.position, &bodies.name);
            let new_pos = bodies.position + bodies.velocity * self.time_step + ((accel1 * self.time_step.powi(2)) / 2.0f64);
            let accel2 = self.accel_loop(&new_pos, &bodies.name);
            let new_vel = bodies.velocity + ((accel1 + accel2) / 2.0f64) * self.time_step;
            let new_body = Body
            {
                bID : bodies.bID,
                name : bodies.name.clone(),
                position : new_pos,
                velocity : new_vel,
                mass : bodies.mass,
                radius : bodies.radius


            };
            newbodylist.push(new_body);
        }
        self.world.bodylist = newbodylist;
        */
        let mut newbodylist = self.world.get_body_list_cpy().unwrap();
        for bodies in newbodylist.iter_mut() {
            //Velocity Verlet Integration
            let accel1 = self.accel_loop(&bodies.position, &bodies.bID);
            let new_pos = bodies.position + bodies.velocity * self.time_step + ((accel1 * self.time_step.powi(2)) / 2.0f64);
            let accel2 = self.accel_loop(&new_pos, &bodies.bID);
            let new_vel = bodies.velocity + ((accel1 + accel2) / 2.0f64) * self.time_step;
            bodies.position = new_pos;
            bodies.velocity = new_vel;
        }
        self.world.bodylist = newbodylist;
        
    }
    fn accel_loop(&self, orig : &vec::Vec3, bid : &i64) -> vec::Vec3
    {
        let mut accel_vel = vec::Vec3::default();
        for tgt in self.world.get_body_list().iter() 
        {
            if &tgt.bID == bid // dont perform calculations on itself.
            {
                continue;
            }
            accel_vel = accel_vel + orig.get_acceleration_vec(&tgt)

            
        }
        accel_vel

    }

}


impl Default for PEngine
{
    fn default() -> Self
    {
        PEngine
        {
            bodycount : 0,
            timestamp : 0,
            simticks : 0,
            worldstate : PEngineState::Unloaded,
            time_step : 60.0f64,
            world : World::default()
        }
    }
}

#[derive(Clone)]
pub struct World
{
    pub name : String,
    pub bodylist : Vec<Body>,

}
impl Default for World
{
    fn default() -> Self
    {
        World
        {
            name : "World".to_string(),
            bodylist : Vec::new(),
        }
    }
}
impl World 
{
    pub fn get_body_list(&self) -> &Vec<Body>
    {
        &self.bodylist
    }
    pub fn get_body_list_cpy(&self) -> Result<Vec<Body>, String>
    {
        return Ok(self.bodylist.to_vec());
    }

}

#[derive(Clone)]
pub struct Body
{
    pub bID : i64,
    pub name : String,
    pub radius : f32,
    pub mass : f64,
    pub velocity : vec::Vec3,
    pub position : vec::Vec3,


}