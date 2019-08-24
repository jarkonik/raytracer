extern crate image;

mod vector;
use vector::Vector;

const EYE: Vector = Vector {
    x: 0.0,
    y: 0.0,
    z: 10.0,
};

trait Collider: std::fmt::Debug {
    fn collide(&self, origin: Vector, dir: Vector) -> Option<Vector>;
    fn color(&self) -> [u8; 3];
}

#[derive(Copy, Clone, Debug)]
struct Sphere {
    center: Vector,
    radius: f64,
    color: [u8; 3],
}

#[derive(Copy, Clone, Debug)]
struct Plane {
    normal: Vector,
    offset: f64,
    color: [u8; 3],
}

impl Collider for Plane {
    fn collide(&self, origin: Vector, dir: Vector) -> Option<Vector> {
        let denom = self.normal.dot(dir);

        if denom > std::f64::EPSILON {
            let p0l0 = (self.normal * self.offset) - origin;
            let t = p0l0.dot(self.normal) / denom;

            if t >= 0.0 {
                return Some(origin + dir * t);
            } else {
                return None;
            }
        } else {
            return None;
        }
    }

    fn color(&self) -> [u8; 3] {
        return self.color;
    }
}

impl Collider for Sphere {
    fn collide(&self, origin: Vector, dir: Vector) -> Option<Vector> {
        let l = self.center - origin;
        let tca = l.dot(dir);

        if tca < 0.0 {
            return None;
        }

        let d2 = l.dot(l) - tca * tca;

        let r2 = self.radius * self.radius;

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

    fn color(&self) -> [u8; 3] {
        return self.color;
    }
}

const VIEW_PLANE_DIST: f64 = 400.0;
const IMG_WIDTH: u32 = 800;
const IMG_HEIGHT: u32 = 800;
const LIGHT_POINT: Vector = Vector {
    x: 0.0,
    y: 500.0,
    z: 0.0,
};

#[derive(Debug)]
struct Collision {
    distance: f64,
    object: std::rc::Rc<dyn Collider>,
    hit_point: Vector,
}

struct World {
    pub objects: Vec<std::rc::Rc<dyn Collider>>,
}

fn main() {
    let world = World {
        objects: vec![
            std::rc::Rc::new(Plane {
                normal: Vector {
                    x: 0.0,
                    y: 1.0,
                    z: 0.0,
                },
                color: [0, 0, 255],
                offset: 50.0,
            }),
            std::rc::Rc::new(Sphere {
                center: Vector {
                    x: 0.0,
                    y: 0.0,
                    z: -150.0,
                },
                color: [255, 0, 0],
                radius: 50.0,
            }),
            std::rc::Rc::new(Sphere {
                center: Vector {
                    x: 50.0,
                    y: 0.0,
                    z: -60.0,
                },
                color: [0, 255, 0],
                radius: 50.0,
            }),
        ],
    };
    let mut imgbuf = image::ImageBuffer::new(IMG_WIDTH, IMG_HEIGHT);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let ray_dir = Vector {
            x: (x as f64) - (IMG_WIDTH as f64) / 2.0,
            y: (y as f64) - (IMG_HEIGHT as f64) / 2.0,
            z: -VIEW_PLANE_DIST,
        }
        .normalize();

        let collisions: Vec<Collision> = world
            .objects
            .iter()
            .filter_map(|object| {
                let hit_point = object.collide(EYE, ray_dir);
                match hit_point {
                    Some(point) => Some(Collision {
                        distance: (point - EYE).magnitude(),
                        hit_point: point,
                        object: object.clone(),
                    }),
                    None => None,
                }
            })
            .collect();

        if collisions.is_empty() {
            continue;
        }

        let closest_collision = collisions.iter().fold(&collisions[0], |acc, collision| {
            if collision.distance < acc.distance {
                collision
            } else {
                acc
            }
        });

        let in_shadow = world.objects.iter().any(|object| {
            object
                .collide(
                    closest_collision.hit_point,
                    (closest_collision.hit_point - LIGHT_POINT).normalize(),
                )
                .is_some()
        });

        if in_shadow {
            continue;
        }

        let color = closest_collision.object.color();

        *pixel = image::Rgb(color);
    }

    imgbuf.save("test.png").unwrap();
}
