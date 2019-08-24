use std::ops;

#[derive(Copy, Clone, Debug)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Copy, Clone, Debug)]
pub struct Vector4 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

#[derive(Copy, Clone, Debug)]
pub struct Matrix4 {
    pub x: Vector4,
    pub y: Vector4,
    pub z: Vector4,
    pub w: Vector4,
}

impl Matrix4 {
    pub fn x_rot(angle: f64) -> Matrix4 {
        return Matrix4 {
            x: Vector4 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
                w: 0.0,
            },
            y: Vector4 {
                x: 0.0,
                y: (-angle).cos(),
                z: (-angle).sin(),
                w: 0.0,
            },
            z: Vector4 {
                x: 0.0,
                y: -(-angle).sin(),
                z: (-angle).cos(),
                w: 0.0,
            },
            w: Vector4 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: 1.0,
            },
        };
    }
}

impl ops::Mul<Matrix4> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: Matrix4) -> Vector3 {
        return Vector3 {
            x: self.x * rhs.x.x + self.y * rhs.x.y + self.z * rhs.x.z + rhs.x.w,
            y: self.x * rhs.y.x + self.y * rhs.y.y + self.z * rhs.y.z + rhs.y.w,
            z: self.x * rhs.z.x + self.y * rhs.z.y + self.z * rhs.z.z + rhs.z.w,
        };
    }
}

impl Vector3 {
    pub fn magnitude(self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn dot(self, rhs: Vector3) -> f64 {
        return self.x * rhs.x + self.y * rhs.y + self.z * rhs.z;
    }

    pub fn normalize(self) -> Vector3 {
        return self / self.magnitude();
    }
}

impl ops::Neg for Vector3 {
    type Output = Vector3;

    fn neg(self) -> Vector3 {
        Vector3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl ops::Add<Vector3> for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Vector3) -> Vector3 {
        Vector3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::Mul<Vector3> for f64 {
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Vector3 {
        Vector3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl ops::Mul<f64> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: f64) -> Vector3 {
        Vector3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl ops::Div<f64> for Vector3 {
    type Output = Vector3;

    fn div(self, rhs: f64) -> Vector3 {
        Vector3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl ops::Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Vector3) -> Vector3 {
        Vector3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}
