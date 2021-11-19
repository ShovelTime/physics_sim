use std::any::Any;
use std::ops::Mul;
use std::ops::Add;
use std::ops::Sub;
use std::ops::Div;

pub struct Vec3<T : Any + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>> 
{
    x : T,
    y : T,
    z : T,
}

impl<T : Any + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>> Vec3<T>
{
    fn new(x: T, y: T, z: T) -> Vec3<T>
    {
        Vec3 {x:x,y:y,z:z}
    }
}

impl<T : Any + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>> std::ops::Index<i32> for Vec3<T>
{
    type Output = T;
    fn index(&self, _ind :i32) -> &Self::Output
    {
        match _ind
        {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => &self.x,
        }

    }


}
impl<T : Any + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>> std::ops::IndexMut<i32> for Vec3<T>
{
    fn index_mut(&mut self,_ind : i32) -> &mut Self::Output
    {
        match _ind
        {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => &mut self.x,
        }

    }
}

impl<T : Any + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>> std::ops::Add for Vec3<T>
{
    type Output = Option<Vec3<T>>;
    fn add(self, other : Self) -> Option<Vec3<T>>
    {
        if self.x.type_id() == other.x.type_id()
        {
            Some(Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z))
        }
        else
        {
            None
        }
    }
}



/*
impl std::ops::Add for Vec3<f32>
{
    type Output = Self;
    fn add(self, other : Self) -> Vec3<f32>
    {
            Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z)

    }

}
impl std::ops::Sub for Vec3<f32>
{
    type Output = Self;
    fn sub(self, other : Self) -> Vec3<f32>
    {
            Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z)

    }

}
impl std::ops::Mul for Vec3<f32>
{
    type Output = Self;
    fn mul(self, other: Self) -> Vec3<f32>
    {
        Vec3::new(self.x * other.x, self.x * other.y, self.z * other.z)
    }
}


impl std::ops::Add for Vec3<f64>
{
    type Output = Self;
    fn add(self, other : Self) -> Vec3<f64>
    {
            Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z)

    }

}

impl std::ops::Sub for Vec3<f64>
{
    type Output = Self;
    fn sub(self, other : Self) -> Vec3<f64>
    {
            Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z)

    }

}
impl std::ops::Mul for Vec3<f64>
{
    type Output = Self;
    fn mul(self, other: Self) -> Vec3<f64>
    {
        Vec3::new(self.x * other.x, self.x * other.y, self.z * other.z)
    }
}


impl std::ops::Add for Vec3<i128>
{
    type Output = Self;
    fn add(self, other : Self) -> Vec3<i128>
    {
            Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z)

    }

}
impl std::ops::Sub for Vec3<i128>
{
    type Output = Self;
    fn sub(self, other : Self) -> Vec3<i128>
    {
            Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z)

    }

}
impl std::ops::Mul for Vec3<i128>
{
    type Output = Self;
    fn mul(self, other: Self) -> Vec3<i128>
    {
        Vec3::new(self.x * other.x, self.x * other.y, self.z * other.z)
    }
}



impl std::ops::Add for Vec3<i64>
{
    type Output = Self;
    fn add(self, other : Self) -> Vec3<i64>
    {
            Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z)

    }

}
impl std::ops::Sub for Vec3<i64>
{
    type Output = Self;
    fn sub(self, other : Self) -> Vec3<i64>
    {
            Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z)

    }

}
impl std::ops::Mul for Vec3<i64>
{
    type Output = Self;
    fn mul(self, other: Self) -> Vec3<i64>
    {
        Vec3::new(self.x * other.x, self.x * other.y, self.z * other.z)
    }
}



impl std::ops::Add for Vec3<i32>
{
    type Output = Self;
    fn add(self, other : Self) -> Vec3<i32>
    {
            Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z)

    }

}
impl std::ops::Sub for Vec3<i32>
{
    type Output = Self;
    fn sub(self, other : Self) -> Vec3<i32>
    {
            Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z)

    }

}
impl std::ops::Mul for Vec3<i32>
{
    type Output = Self;
    fn mul(self, other: Self) -> Vec3<i32>
    {
        Vec3::new(self.x * other.x, self.x * other.y, self.z * other.z)
    }
}
*/