use crate::math::vec::Vec3;
use crate::constants::PI;
pub fn draw_sphere(radius : f32, position : Vec3, sectorcount : i32, stackcount : i32 ) -> Vec<[f32; 3]>
{
    let (x, y, z , xy) : (f32, f32, f32, f32);
    let (sectorangle, stackangle) : (f32, f32);
    let vertexvec = Vec::new();
    //let (s, t) : (f32, f32);
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
            vertexvec.push([x, y ,z]);

        } 

    }
    vertexvec
} 