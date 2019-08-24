extern crate image;

mod vector;
use vector::{Matrix4, Vector3};

const EYE: Vector3 = Vector3 {
    x: 0.0,
    y: 120.0,
    z: 150.0,
};

trait Collider: std::fmt::Debug {
    fn collide(&self, origin: Vector3, dir: Vector3) -> Option<CollisionData>;
    fn color(&self) -> Vector3;
}

fn reflect(i: Vector3, n: Vector3) -> Vector3 {
    return i - 2.0 * i.dot(n) * n;
}

#[derive(Copy, Clone, Debug)]
struct Sphere {
    center: Vector3,
    radius: f64,
    color: Vector3,
}

#[derive(Copy, Clone, Debug)]
struct Plane {
    normal: Vector3,
    offset: f64,
    color: Vector3,
}

#[derive(Debug)]
struct CollisionData {
    hit_point: Vector3,
    normal: Vector3,
    distance: f64,
}

impl Collider for Plane {
    fn collide(&self, origin: Vector3, dir: Vector3) -> Option<CollisionData> {
        let denom = (-self.normal).dot(dir);

        if denom > std::f64::EPSILON {
            let p0l0 = (self.normal * self.offset) - origin;
            let t = p0l0.dot(-self.normal) / denom;

            if t >= 0.0 {
                let hit_point = origin + dir * t;
                return Some(CollisionData {
                    hit_point: hit_point,
                    distance: (hit_point - origin).magnitude(),
                    normal: self.normal,
                });
            } else {
                return None;
            }
        } else {
            return None;
        }
    }

    fn color(&self) -> Vector3 {
        return self.color;
    }
}

impl Collider for Sphere {
    fn collide(&self, origin: Vector3, dir: Vector3) -> Option<CollisionData> {
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

        let hit_point = origin + dir * t0;

        return Some(CollisionData {
            hit_point: hit_point,
            distance: (hit_point - origin).magnitude(),
            normal: (hit_point - self.center).normalize(),
        });
    }

    fn color(&self) -> Vector3 {
        return self.color;
    }
}

const VIEW_PLANE_DIST: f64 = 800.0;
const IMG_WIDTH: u32 = 800;
const IMG_HEIGHT: u32 = 800;
const LIGHT_POINT: Vector3 = Vector3 {
    x: 150.0,
    y: 120.0,
    z: -20.0,
};

#[derive(Debug)]
struct Collision {
    data: CollisionData,
    object: std::rc::Rc<dyn Collider>,
}

struct World {
    pub objects: Vec<std::rc::Rc<dyn Collider>>,
}

fn main() {
    let world = World {
        objects: vec![
            std::rc::Rc::new(Plane {
                normal: Vector3 {
                    x: 0.0,
                    y: 1.0,
                    z: 0.0,
                },
                color: Vector3 {
                    x: 0.0,
                    y: 0.0,
                    z: 255.0,
                },
                offset: -50.0,
            }),
            std::rc::Rc::new(Sphere {
                center: Vector3 {
                    x: -50.0,
                    y: 0.0,
                    z: -150.0,
                },
                color: Vector3 {
                    x: 255.0,
                    y: 0.0,
                    z: 0.0,
                },
                radius: 50.0,
            }),
            std::rc::Rc::new(Sphere {
                center: Vector3 {
                    x: 50.0,
                    y: 0.0,
                    z: -60.0,
                },
                color: Vector3 {
                    x: 0.0,
                    y: 255.0,
                    z: 0.0,
                },
                radius: 50.0,
            }),
        ],
    };
    let mut imgbuf = image::ImageBuffer::new(IMG_WIDTH, IMG_HEIGHT);

    let rot: f64 = -30.0;

    let eye_rot = Matrix4::x_rot(rot.to_radians());
    println!("{:?}", eye_rot);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let view_dir = Vector3 {
            x: (x as f64) - (IMG_WIDTH as f64) / 2.0,
            y: (IMG_HEIGHT as f64) / 2.0 - (y as f64),
            z: -VIEW_PLANE_DIST,
        }
        .normalize();

        let ray_dir = view_dir * eye_rot;

        let collisions: Vec<Collision> = world
            .objects
            .iter()
            .filter_map(|object| {
                let data = object.collide(EYE, ray_dir);
                match data {
                    Some(data) => Some(Collision {
                        data: data,
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
            if collision.data.distance < acc.data.distance {
                collision
            } else {
                acc
            }
        });

        let light_dir = (LIGHT_POINT - closest_collision.data.hit_point).normalize();

        let in_shadow = world.objects.iter().any(|object| {
            object
                .collide(closest_collision.data.hit_point, light_dir)
                .is_some()
        });

        if in_shadow {
            continue;
        }

        let albedo = closest_collision.object.color();

        let normal = closest_collision.data.normal;

        let attenuation = 1.0 / ((closest_collision.data.hit_point - LIGHT_POINT).magnitude());
        let light_intenisty = attenuation * 100.0;

        let diffuse = light_intenisty * albedo * f64::max(0.0, normal.dot(light_dir));

        let reflection = reflect(light_dir, normal);

        let specular = attenuation * f64::max(0.0, reflection.dot(-ray_dir)).powf(2.0);

        let color = diffuse
            + Vector3 {
                x: 255.0,
                y: 255.0,
                z: 255.0,
            } * specular
                * 0.1;

        *pixel = image::Rgb([color.x as u8, color.y as u8, color.z as u8]);
    }

    imgbuf.save("test.png").unwrap();
}
