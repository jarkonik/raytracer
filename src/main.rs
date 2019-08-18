extern crate image;

mod vector;
use vector::{collide, Vector};

const EYE: Vector = Vector {
    x: 0.0,
    y: 0.0,
    z: 10.0,
};

#[derive(Copy, Clone, Debug)]
struct Sphere {
    center: Vector,
    radius: f64,
    color: [u8; 3],
}

const VIEW_PLANE_DIST: f64 = 400.0;
const IMG_WIDTH: u32 = 800;
const IMG_HEIGHT: u32 = 800;
const LIGHT_POINT: Vector = Vector {
    x: 0.0,
    y: 0.0,
    z: 0.0,
};

const SPHERES: [Sphere; 2] = [
    Sphere {
        center: Vector {
            x: 0.0,
            y: 0.0,
            z: -150.0,
        },
        radius: 50.0,
        color: [255, 0, 0],
    },
    Sphere {
        center: Vector {
            x: 50.0,
            y: 0.0,
            z: -60.0,
        },
        radius: 50.0,
        color: [0, 255, 0],
    },
];

#[derive(Copy, Clone, Debug)]
struct Collision {
    distance: f64,
    sphere: Sphere,
    hit_point: Vector,
}

fn main() {
    let mut imgbuf = image::ImageBuffer::new(IMG_WIDTH, IMG_HEIGHT);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let ray_dir = Vector {
            x: (x as f64) - (IMG_WIDTH as f64) / 2.0,
            y: (y as f64) - (IMG_HEIGHT as f64) / 2.0,
            z: -VIEW_PLANE_DIST,
        }
        .normalize();

        let collisions: Vec<Collision> = SPHERES
            .iter()
            .filter_map(|&sphere| -> Option<Collision> {
                let hit_point = collide(EYE, ray_dir, sphere.center, sphere.radius);
                match hit_point {
                    Some(point) => Some(Collision {
                        distance: (point - EYE).magnitude(),
                        hit_point: point,
                        sphere: sphere,
                    }),
                    None => None,
                }
            })
            .collect();

        if collisions.is_empty() {
            continue;
        }

        let closest_collision = collisions.iter().fold(collisions[0], |acc, &collision| {
            if collision.distance < acc.distance {
                collision
            } else {
                acc
            }
        });

        let in_shadow = SPHERES.iter().any(|sphere| {
            collide(
                closest_collision.hit_point,
                (LIGHT_POINT - closest_collision.hit_point).normalize(),
                sphere.center,
                sphere.radius,
            )
            .is_some()
        });

        if in_shadow {
            continue;
        }

        let color = closest_collision.sphere.color;

        *pixel = image::Rgb(color);
    }

    imgbuf.save("test.png").unwrap();
}
