use crate::math::vec::Vec3;
use crate::constants::PI;

pub fn create_sphere(radius : f32) -> (Vec<Vec3>, Vec<Vec3>, Vec<f32>) // Vertex vector, normals vector, tex coords vector.
{
    let sectorcount : i32 = 30;
    let stackcount : i32 = 30;

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
        z = radius * stackangle.sin();
        for iter2 in 0..sectorcount
        {
            sectorangle = iter2 as f32 * sectorstep;

            x = xy * sectorangle.cos();
            y = xy * sectorangle.sin();
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
    let sectorcount : i32 = 30;
    let stackcount : i32 = 30;
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