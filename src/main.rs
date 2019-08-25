#![feature(clamp)]
extern crate image;

mod vector;
use vector::{Matrix4, Vector3};

const EYE: Vector3 = Vector3 {
    x: 0.0,
    y: 180.0,
    z: 110.0,
};

const LIGHT_POINT: Vector3 = Vector3 {
    x: -50.0,
    y: 50.0,
    z: 90.0,
};

trait Collider: std::fmt::Debug {
    fn collide(&self, origin: Vector3, dir: Vector3) -> Option<CollisionData>;
    fn color(&self) -> Vector3;
    fn reflectivity(&self) -> f64;
}

fn reflect(i: Vector3, n: Vector3) -> Vector3 {
    return 2.0 * i.dot(n) * n - i;
}

#[derive(Copy, Clone, Debug)]
struct Sphere {
    center: Vector3,
    radius: f64,
    color: Vector3,
    reflectivity: f64,
}

#[derive(Copy, Clone, Debug)]
struct Plane {
    normal: Vector3,
    offset: f64,
    color: Vector3,
    reflectivity: f64,
}

#[derive(Debug)]
struct CollisionData {
    hit_point: Vector3,
    normal: Vector3,
    distance: f64,
}

impl Collider for Plane {
    fn reflectivity(&self) -> f64 {
        return self.reflectivity;
    }

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
    fn reflectivity(&self) -> f64 {
        return self.reflectivity;
    }

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

const IMG_SIZE: u32 = 800;

#[derive(Debug)]
struct Collision {
    data: CollisionData,
    object: std::rc::Rc<dyn Collider>,
}

struct World {
    pub objects: Vec<std::rc::Rc<dyn Collider>>,
}

fn get_color(origin: Vector3, world: &World, ray_dir: Vector3, depth: u8) -> Vector3 {
    if depth > 1 {
        return Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
    }

    let collisions: Vec<Collision> = world
        .objects
        .iter()
        .filter_map(|object| {
            let data = object.collide(origin, ray_dir);
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
        return Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
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

    let albedo = closest_collision.object.color();

    let normal = closest_collision.data.normal;

    let light_intenisty = 1.0;
    let diffuse = (light_intenisty * f64::max(0.0, normal.dot(light_dir))) * albedo;

    let reflection = reflect(light_dir, normal);

    let specular = light_intenisty * f64::max(0.0, reflection.dot(-ray_dir)).powf(10.0);

    let color = if in_shadow {
        Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    } else {
        diffuse
            + Vector3 {
                x: 255.0,
                y: 255.0,
                z: 255.0,
            } * specular
                * 0.5
    };

    let color = color * (1.0 - closest_collision.object.reflectivity())
        + closest_collision.object.reflectivity()
            * get_color(
                closest_collision.data.hit_point,
                world,
                reflection,
                depth + 1,
            );

    let clamped = Vector3 {
        x: f64::clamp(color.x, 0.0, 255.0),
        y: f64::clamp(color.y, 0.0, 255.0),
        z: f64::clamp(color.z, 0.0, 255.0),
    };
    return clamped;
}

fn main() {
    let world = World {
        objects: vec![
            std::rc::Rc::new(Plane {
                reflectivity: 0.1,
                normal: Vector3 {
                    x: 0.0,
                    y: 0.0,
                    z: 1.0,
                },
                color: Vector3 {
                    x: 255.0,
                    y: 255.0,
                    z: 255.0,
                },
                offset: -300.0,
            }),
            std::rc::Rc::new(Plane {
                reflectivity: 0.1,
                normal: Vector3 {
                    x: 0.0,
                    y: 1.0,
                    z: 0.0,
                },
                color: Vector3 {
                    x: 255.0,
                    y: 255.0,
                    z: 255.0,
                },
                offset: -50.0,
            }),
            std::rc::Rc::new(Sphere {
                reflectivity: 0.3,
                center: Vector3 {
                    x: -20.0,
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
                reflectivity: 0.0,
                center: Vector3 {
                    x: 50.0,
                    y: 0.0,
                    z: -40.0,
                },
                color: Vector3 {
                    x: 0.0,
                    y: 255.0,
                    z: 0.0,
                },
                radius: 50.0,
            }),
            std::rc::Rc::new(Sphere {
                reflectivity: 0.3,
                center: Vector3 {
                    x: -75.0,
                    y: 0.0,
                    z: -50.0,
                },
                color: Vector3 {
                    x: 0.0,
                    y: 0.0,
                    z: 255.0,
                },
                radius: 50.0,
            }),
        ],
    };
    let mut imgbuf = image::ImageBuffer::new(IMG_SIZE, IMG_SIZE);

    let rot: f64 = -45.0;

    let eye_rot = Matrix4::x_rot(rot.to_radians());
    println!("{:?}", eye_rot);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let view_dir = Vector3 {
            x: (x as f64) - (IMG_SIZE as f64) / 2.0,
            y: (IMG_SIZE as f64) / 2.0 - (y as f64),
            z: -(IMG_SIZE as f64),
        }
        .normalize();

        let ray_dir = view_dir * eye_rot;

        let color = get_color(EYE, &world, ray_dir, 0);

        *pixel = image::Rgb([color.x as u8, color.y as u8, color.z as u8]);
    }

    imgbuf.save("test.png").unwrap();
}
