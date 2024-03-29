
use crate::constants::PI;

#[derive(Copy, Clone)]
pub struct Vec3
{
    pub x : f64,
    pub y : f64,
    pub z : f64,
}
//#[allow(dead_code)]
impl Vec3
{
    pub fn new(x: f64, y: f64, z: f64) -> Vec3
    {
        Vec3 {x:x,y:y,z:z}
    }
    pub fn abs(inp : &Vec3) -> Vec3
    {
        Vec3::new( inp.x.abs(), inp.y.abs(), inp.z.abs() )
    } 
    pub fn abs_self(&self) -> Vec3
    {
        Vec3::new( self.x.abs(), self.y.abs(), self.z.abs() )

    }
    pub fn from_vec(vec : Vec<f64>, offset : usize) -> Vec3
    {
        Vec3::new(vec[offset + 0], vec[offset + 1], vec[offset + 2])
    }
    pub fn fast_div(&self, other : f64) -> Vec3 // faster than the normal division "/" Operator, but loses some accuracy to rounding errors.
    {
        let div = 1.0f64 / other;
        Vec3::new(self.x * div, self.y * div, self.z * div)
    }
    pub fn dot(&self, other : Vec3) -> f64
    {
        self.x * other.x + self.y * other.y + self.z * other.z
    } 
    pub fn cross(&self, other : Vec3) -> Vec3
    {
        Vec3::new(self.y * other.z - self.z * other.y , self.z * other.x - self.x * other.z, self.x * other.y - self.y * other.x)
    }
    pub fn length(&self) -> f64
    {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
    pub fn normalize(&self) -> Vec3
    {
        let length = self.length();
        *self / length
    }
    pub fn fast_normalize(&self) -> Vec3
    {
        self.fast_div(self.length())
    }
    pub fn get_distance_vec(&self, other: &Vec3) -> Vec3
    {
        Vec3::abs(&(*self - *other))
    }
    pub fn get_distance_sum(&self ,other: &Vec3) -> f64
    {
        let stage1 = self.get_distance_vec(other);
        (stage1.x.powi(2) + stage1.y.powi(2) + stage1.z.powi(2)).sqrt()

    }
    pub fn rotate_2d(&self ,theta : f64) -> Vec3
    {
        Vec3::new(self.x * theta.cos() - self.y * theta.sin(), self.y * theta.cos() + self.x * theta.sin() , self.z)

    }

    pub fn get_angle(&self, tgt : &Vec3) -> f64
    {
        let u_vec = (*self - *tgt).normalize();
        let ang = u_vec.y.atan2(u_vec.x);
        if ang < 0.0
        {
            2.0 * PI - ang.abs()
        }
        else
        {
            ang
        }
        
    }

    pub fn get_direction(&self, other: &Vec3) -> Vec3
    {
        self.get_distance_vec(other).normalize()
    }

    //fn to_vec2(self) -> Vec2
    
    //fn to_vec4(self, other : Option<f64>) -> Vec4
}

impl Default for Vec3
{
    fn default() -> Vec3
    {
        Vec3::new( 0.0f64, 0.0f64, 0.0f64)
    }
}

impl std::ops::Index<i32> for Vec3
{
    type Output = f64;
    fn index(&self, _ind :i32) -> &Self::Output
    {
        match _ind
        {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of bounds!")
        }

    }


}
impl std::ops::IndexMut<i32> for Vec3
{
    fn index_mut(&mut self,_ind : i32) -> &mut Self::Output
    {
        match _ind
        {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Index out of bounds!")
        }

    }
}

impl std::ops::Add<Vec3> for Vec3
{
    type Output = Vec3;
    fn add(self, other : Self) -> Self::Output
    {
        Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}
impl std::ops::Add<f64> for Vec3
{
    type Output = Vec3;
    fn add(self, other : f64) -> Self::Output
    {
        Vec3::new(self.x + other, self.y + other, self.z + other)
    }
}

impl std::ops::Sub<Vec3> for Vec3
{
    type Output = Vec3;
    fn sub(self, other: Self) -> Self::Output
    {
        Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}
impl std::ops::Sub<f64> for Vec3
{
    type Output = Vec3;
    fn sub(self, other: f64) -> Self::Output
    {
        Vec3::new(self.x - other, self.y - other, self.z - other)
    }
}

impl std::ops::Mul<Vec3> for Vec3
{
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Self::Output
    {
        Vec3::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }
}
impl std::ops::Mul<f64> for Vec3
{
    type Output = Vec3;
    fn mul(self, other: f64) -> Self::Output
    {
        Vec3::new(self.x * other, self.y * other, self.z * other)
    }
}

impl std::ops::Div<Vec3> for Vec3
{
    type Output = Vec3;
    fn div(self, other: Vec3) -> Self::Output
    {
        Vec3::new(self.x / other.x, self.y / other.y, self.z / other.z)
        
    }

}
impl std::ops::Div<f64> for Vec3
{
    type Output = Vec3;
    fn div(self, other: f64) -> Self::Output
    {
        Vec3::new(self.x / other, self.y / other, self.z / other)
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