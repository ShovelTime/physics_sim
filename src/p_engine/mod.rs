use crate::math::vec;
use crate::math::phys::Phys;
use crate::constants;


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
        let mut newbodylist = Vec::<Body>::new();
        
        for bodies in self.world.get_body_list().iter() {
            let accel1 = self.accel_loop(&bodies.position, &bodies.bID);
            let new_pos = bodies.position + bodies.velocity * self.time_step + ((accel1 * self.time_step.powi(2)) / 2.0f64);
            let accel2 = self.accel_loop(&new_pos, &bodies.bID);
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
impl vec::Vec3
{
    pub fn get_acceleration_vec(&self , tgt: &Body) -> vec::Vec3
    {
        let first_arg = constants::GRAV_CONST * &tgt.mass;
        //let dist = self.get_distance_vec(&tgt.position);
        let dist = tgt.position - *self;
        let mag = self.get_distance_sum(&tgt.position).powi(3);

        (dist * first_arg) / mag


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
            time_step : 1.0f64,
            world : World::default()
            
        }
    }
}

#[derive(Clone)]
pub struct World
{
    pub name : String,
    pub bodylist : Vec<Body>,
    pub barycenter_mass : f64,

}
impl Default for World
{
    fn default() -> Self
    {
        World
        {
            name : "World".to_string(),
            bodylist : Vec::new(),
            barycenter_mass : 0.0
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

impl Body{
    pub fn stringify(&self) -> String
    {
        format!("\nname : {0} \n position : [x: {1}; y: {2}; z: {3}] \n velocity : [xv: {4}; yv: {5}; zv: {6}]  \n \n \n", self.name, self.position.x, self.position.y, self.position.z, self.velocity.x, self.velocity.y, self.velocity.z).to_string()
    }
    pub fn get_angular_momentum_vec(&self) -> vec::Vec3 // Relative to Barycenter
    {
        let mass_scalar = self.velocity * self.mass;
        self.position.cross(mass_scalar)
    } 
    pub fn get_specific_ang_momentum_vec(&self) -> vec::Vec3 // Rel to barycenter
    {
        self.position.cross(self.velocity)
    }
    pub fn get_eccentricity_vec(&self, tgt_mass : f64) -> vec::Vec3
    {
        self.velocity.cross(self.get_specific_ang_momentum_vec()) / self.get_grav_param(tgt_mass) - self.position.normalize()()
    }
    pub fn get_eccentricity(&self, tgt_mass : f64) -> f64
    {
        let ecc_vec = self.get_eccentricity_vec(tgt_mass);
        ecc_vec.length()

    }
    pub fn get_grav_param(&self, tgt_mass : f64) -> f64
    {
        constants::GRAV_CONST * tgt_mass
    }

    pub fn get_semimajor_axis(&self ,tgt_mass : f64) -> f64
    {
        let grav_param = self.get_grav_param(tgt_mass);
        let dist_tobarycenter = self.position.length();

        let first_param = grav_param + dist_tobarycenter;
        let second_param = 2.0 * grav_param - dist_tobarycenter * self.velocity.length().powi(2);
        first_param / second_param


    }
}