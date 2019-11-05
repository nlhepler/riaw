#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn ones() -> Vec3 {
        Vec3::new(1.0, 1.0, 1.0)
    }

    pub fn zeros() -> Vec3 {
        Vec3::new(0.0, 0.0, 0.0)
    }

    pub fn as_unit(self) -> Vec3 {
        self / self.len()
    }

    pub fn to_u8(v: f32) -> u8 {
        (256.0 * v).min(255.0) as u8
    }

    pub fn from_u8(v: u8) -> f32 {
        // 0.5 / 256.
        v as f32 / 256.0 + 0.001953125
    }

    pub fn to_rgb(&self) -> [u8; 3] {
        [
            Self::to_u8(self.x),
            Self::to_u8(self.y),
            Self::to_u8(self.z),
        ]
    }

    pub fn from_argb(v: u32) -> Self {
        let x = Self::from_u8(((v >> 16) & 0xff) as u8);
        let y = Self::from_u8(((v >> 8) & 0xff) as u8);
        let z = Self::from_u8((v & 0xff) as u8);
        Vec3 { x, y, z }
    }

    pub fn to_argb(&self) -> u32 {
        ((Self::to_u8(self.x) as u32) << 16)
            | ((Self::to_u8(self.y) as u32) << 8)
            | (Self::to_u8(self.z) as u32)
    }

    pub fn cross(&self, rhs: &Vec3) -> Vec3 {
        Vec3::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }

    pub fn dot(&self, rhs: &Vec3) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn squared_len(&self) -> f32 {
        self.dot(self)
    }

    pub fn len(&self) -> f32 {
        self.squared_len().sqrt()
    }

    pub fn sqrt(&self) -> Vec3 {
        Vec3::new(self.x.sqrt(), self.y.sqrt(), self.z.sqrt())
    }

    pub fn powi(&self, i: i32) -> Vec3 {
        Vec3::new(self.x.powi(i), self.y.powi(i), self.z.powi(i))
    }

    pub fn to_array(&self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }
}

impl std::ops::Index<usize> for Vec3 {
    type Output = f32;
    fn index(&self, i: usize) -> &f32 {
        match i {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("axis out-of-bounds"),
        }
    }
}

impl std::ops::Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

impl std::ops::Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}
impl std::ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}
impl std::ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}
impl std::ops::Div<Vec3> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.x / rhs.x, self.y / rhs.y, self.z / rhs.z)
    }
}

impl std::ops::Add<f32> for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: f32) -> Vec3 {
        Vec3::new(self.x + rhs, self.y + rhs, self.z + rhs)
    }
}
impl std::ops::Sub<f32> for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: f32) -> Vec3 {
        Vec3::new(self.x - rhs, self.y - rhs, self.z - rhs)
    }
}
impl std::ops::Mul<f32> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f32) -> Vec3 {
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}
impl std::ops::Div<f32> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f32) -> Vec3 {
        let k = 1.0 / rhs;
        self * k
    }
}

impl std::ops::Add<Vec3> for f32 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self + rhs.x, self + rhs.y, self + rhs.z)
    }
}
impl std::ops::Sub<Vec3> for f32 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self - rhs.x, self - rhs.y, self - rhs.z)
    }
}
impl std::ops::Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self * rhs.x, self * rhs.y, self * rhs.z)
    }
}
impl std::ops::Div<Vec3> for f32 {
    type Output = Vec3;
    fn div(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self / rhs.x, self / rhs.y, self / rhs.z)
    }
}

impl std::ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl std::ops::DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

#[macro_export]
macro_rules! vec3 {
    [$x:expr, $y:expr, $z:expr] => {
        Vec3 { x: $x as f32, y: $y as f32, z: $z as f32 }
    }
}
