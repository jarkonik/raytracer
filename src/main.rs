extern crate image;

mod vector;
use vector::Vector;

const EYE: Vector = Vector {
    x: 0.0,
    y: 0.0,
    z: 0.0,
};
const SPHERE: Vector = Vector {
    x: 0.0,
    y: 0.0,
    z: -60.0,
};
const SPHERE_RADIUS: f64 = 50.0;
const VIEW_PLANE_DIST: f64 = 100.0;
const IMG_WIDTH: u32 = 800;
const IMG_HEIGHT: u32 = 800;

fn intersects(origin: Vector, dir: Vector, center: Vector, r: f64) -> bool {
    let l = center - origin;
    let tc = l.dot(dir);

    if tc < 0.0 {
        return false;
    }

    let r2 = r * r;

    let d2 = l.dot(l) - tc * tc;

    return d2 <= r2;
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

        let (r, g, b) = if intersects(EYE, ray_dir, SPHERE, SPHERE_RADIUS) {
            (255, 0, 0)
        } else {
            (255, 255, 255)
        };

        *pixel = image::Rgb([r, g, b]);
    }

    imgbuf.save("test.png").unwrap();
}
