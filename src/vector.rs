use std::ops;

#[derive(Copy, Clone, Debug)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector {
    pub fn magnitude(self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn dot(self, rhs: Vector) -> f64 {
        return self.x * rhs.x + self.y * rhs.y + self.z * rhs.z;
    }

    pub fn normalize(self) -> Vector {
        return self / self.magnitude();
    }
}

impl ops::Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Vector {
        Vector {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl ops::Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Vector {
        Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f64) -> Vector {
        Vector {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl ops::Div<f64> for Vector {
    type Output = Vector;

    fn div(self, rhs: f64) -> Vector {
        Vector {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl ops::Sub for Vector {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Vector {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

pub fn collide(origin: Vector, dir: Vector, center: Vector, r: f64) -> Option<Vector> {
    let l = center - origin;
    let tca = l.dot(dir);
    let d2 = l.dot(l) - tca * tca;

    let r2 = r * r;

    if d2 > r2 {
        return None;
    }

    let thc = (r2 - d2).sqrt();

    let mut t0 = tca - thc;
    let mut t1 = tca + thc;

    if t0 > t1 {
        std::mem::swap(&mut t0, &mut t1);
    }

    if t0 < 0.0 {
        t0 = t1;
        if t0 < 0.0 {
            return None;
        }
    }

    return Some(origin + dir * t0);
}
