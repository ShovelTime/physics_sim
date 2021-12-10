use crate::math::vec;
use crate::p_engine;

pub trait Phys{
    fn get_distance_vec(&self, other: &vec::Vec3) -> vec::Vec3;
    fn get_distance_sum(&self, other: &vec::Vec3) -> f64;
    fn get_direction(&self, other: &vec::Vec3) -> vec::Vec3;
    fn get_acceleration_vec(&self, tgt : &p_engine::Body) -> vec::Vec3;

}
