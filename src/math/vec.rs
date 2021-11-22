#[derive(Copy, Clone)]
pub struct Vec3 
{
    x : f64,
    y : f64,
    z : f64,
}
#[allow(dead_code)]
impl Vec3
{
    fn new(x: f64, y: f64, z: f64) -> Vec3
    {
        Vec3 {x:x,y:y,z:z}
    }
    fn fast_div(&self, other : f64) -> Vec3 // faster than the div"/" Operator, but loses some accuracy to rounding errors.
    {
        let div = 1.0f64 / other;
        Vec3::new(self.x * div, self.y * div, self.z * div)
    }
    fn dot(&self, other : Vec3) -> f64
    {
        self.x * other.x + self.y * other.y + self.z * other.z
    } 
    fn cross(&self, other : Vec3) -> Vec3
    {
        Vec3::new(self.y * other.z - self.z * other.y , self.z * other.x - self.x * other.z, self.x * other.y - self.y * other.x)
    }
    fn length(&self) -> f64
    {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
    fn normalize(self) -> Vec3
    {
        let length = self.length();
        self / length
    }
    fn fast_normalize(&self) -> Vec3
    {
        self.fast_div(self.length())
    }
    //fn to_vec2(self) -> Vec2
    
    //fn to_vec4(self, other : Option<f64>) -> Vec4
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