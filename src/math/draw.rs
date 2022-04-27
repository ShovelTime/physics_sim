use crate::math::vec::Vec3;
use crate::constants::PI;

pub fn plot_kepler_orbit(sm_axis : f64, eccentricity : f64, mass_1 : f64, tot_mass : f64) -> Vec<Vec3>
{
    if eccentricity < 0.0
    {
        panic!("eccentricity is under 0! wat");
    }
    if eccentricity as i64 == 1
    {
        return Vec::new() //
    }
    let step_size = 0.5; // step size in deg
    let polar_perigee = sm_axis * (1.0 - eccentricity);
    let mut plot_table = Vec::new();
    plot_table.push(Vec3::new(polar_perigee * 0.0_f64.cos(), polar_perigee * 0.0_f64.sin(), 0.0));
    let mut curr_step = 0.0;

    loop
    {
        curr_step += step_size;
        let step_rad = curr_step * constants::PI / 180.0;
        let polar_coord = (sm_axis * (1.0 - eccentricity.powi(2))) / (1.0 + eccentricity * step_rad.cos());
        plot_table.push(Vec3::new(polar_coord * step_rad.cos(), step_rad.sin(), 0.0));

        if curr_step as i64 >= 360
        {
            break
        }
    } 
    
    plot_table
}

pub fn create_sphere(radius : f32, offset : Vec3) -> (Vec<Vec3>, Vec<Vec3>, Vec<f32>) // Vertex vector, normals vector, tex coords vector.
{
    let sectorcount : i32 = 100;
    let stackcount : i32 = 100;

    let (mut x, mut y, mut z , mut xy) : (f32, f32, f32, f32);
    let (mut nx, mut ny, mut nz) : (f32, f32, f32);
    let length_inv = 1.0 / radius;

    let (mut sectorangle, mut stackangle) : (f32, f32);

    let mut vertexvec = Vec::new();
    let mut normalsvec = Vec::new();
    let mut texvec = Vec::new();

    let (mut s, mut t) : (f32, f32); //only useful for texture coordinates.
    
    let sectorstep : f32 = (2.0 * PI / sectorcount as f64) as f32;
    let stackstep = (PI / stackcount as f64) as f32;

    for iter in 0..stackcount
    {
        stackangle = (PI / 2.0) as f32 - (iter as f32) * stackstep;
        xy = radius * stackangle.cos();
        z = radius * stackangle.sin() + offset.z as f32;
        for iter2 in 0..sectorcount
        {
            sectorangle = iter2 as f32 * sectorstep;

            x = xy * sectorangle.cos() + offset.x as f32;
            y = xy * sectorangle.sin() + offset.y as f32;
            vertexvec.push(Vec3::new(x as f64, y as f64, z as f64));

            nx = x * length_inv;
            ny = y * length_inv;
            nz = z *length_inv;
            normalsvec.push(Vec3::new(nx as f64, ny as f64, nz as f64));


            s = (iter2 / sectorcount) as f32;
            t = (iter / stackcount) as f32;
            texvec.push(s);
            texvec.push(t);


        } 

    }
    (vertexvec, normalsvec, texvec)
} 

pub fn fast_sphere(radius : f32) -> Vec<Vec3> // only computes vertices coordinate.
{
    let sectorcount : i32 = 100;
    let stackcount : i32 = 100;
    let (mut x, mut y, mut z , mut xy) : (f32, f32, f32, f32);
    let (mut sectorangle, mut stackangle) : (f32, f32);
    let mut vertexvec = Vec::new();
    let sectorstep : f32 = (2.0 * PI / sectorcount as f64) as f32;
    let stackstep = (PI / stackcount as f64) as f32;

    for iter in 0..stackcount
    {
        stackangle = (PI / 2.0) as f32 - (iter as f32) * stackstep;
        xy = radius * stackangle.cos();
        z = radius * stackangle.sin();
        for iter2 in 0..sectorcount
        {
            sectorangle = iter2 as f32 * sectorstep;

            x = xy * sectorangle.cos();
            y = xy * sectorangle.sin();
            vertexvec.push(Vec3::new(x as f64, y as f64, z as f64));


        } 

    }
    vertexvec
}

pub fn fast_circle(radius : f64) -> Vec<Vec3>
{
    let twicepi = PI * 2.0;
    let mut vertexvec : Vec<Vec3> =  Vec::new();
    let sectorcount = 100;
    //vertexvec.push(Vec3::new(0.0,0.0,0.0));
    for i in 0..sectorcount
    {
        vertexvec.push(Vec3::new(radius * (i as f64 * twicepi / sectorcount as f64).cos(), radius * (i as f64 * twicepi / sectorcount as f64).sin(), 0.0 ));
    }
    vertexvec
}