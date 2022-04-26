use crate::constants;
use crate::math::vec;


pub fn plot_kepler_orbit(sm_axis : f64, eccentricity : f64, mass_1 : f64, tot_mass : f64) -> Vec<vec::Vec3>
{
    if(eccentricity < 0)
    {
        panic!("eccentricity is under 0! wat");
    }
    if (eccentricity == 1)
    {
        return Vec::new() //
    }
    let step_size = 0.5 // step size in deg
    let perigee_y = sm_axis * (1 + eccentricity);
    let perigee_x = (grav_param / mass_1) * perigee_y; 
    let plot_table = Vec::new();
    plot_table.push(vec::Vec3::new(perigee_x, perigee_y, z: f64));
    let curr_step = 0;

    while(curr_step < 360)
    {

    }
    plot_table
}
/*
pub fn plot_kepler_orbit(sm_axis : f64, eccentricity : f64, mass_1 : f64, tot_mass : f64) -> Vec<vec::Vec3>
{
    if(eccentricity < 0)
    {
        panic!("eccentricity is under 0! wat");
    }
    if (eccentricity == 1)
    {
        return Vec::new() //
    }
    let grav_param = (mass_1 * tot_mass) / (mass_1 + tot_mass);
    let step_size = 0.5 // step size in deg
    let perigee_y = sm_axis * (1 + eccentricity);
    let perigee_x = (grav_param / mass_1) * perigee_y; 
    let plot_table = Vec::new();
    plot_table.push(vec::Vec3::new(perigee_x, perigee_y, z: f64));
    let curr_step = 0;

    while(curr_step < 360)
    {

    }
    plot_table
}

pub fn plot_kepler_orbit_vec(sm_axis : f64, eccentricity : vec::Vec3, mass_1 : f64, tot_mass : f64) -> Vec<vec::Vec3>
{
    if(eccentricity < 0)
    {
        panic!("eccentricity is under 0! wat");
    }
    if (eccentricity == 1)
    {
        return Vec::new() //
    }
    let grav_param = (mass_1 * tot_mass) / (mass_1 + tot_mass);
    let step_size = 0.5 // step size in deg
    /*
    let perigee_y = sm_axis * (1 + eccentricity);
    let perigee_x = (grav_param / mass_1) * perigee_y; 
    */

    let plot_table = Vec::new();
    plot_table.push(vec::Vec3::new(perigee_x, perigee_y, z: f64));
    let curr_step = 0;

    while(curr_step < 360)

}
*/