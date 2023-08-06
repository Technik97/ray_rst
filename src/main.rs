use ray_tracer::core::{ray::Ray, vec3::Vec3, colour::colour};

fn main() {
    let width = 360;
    let height = 240;
    let max = 255;

    println!("P3\n{} {}\n{}", width, height, max);

    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);


    for j in (0..height).rev() {
        for i in 0..width {
            let u = i as f32 / width as f32;
            let v = j as f32 / height as f32;

            let ray = Ray::new(origin, lower_left_corner + horizontal * u + vertical * v);
            let clr = colour(&ray);

            // let b: f32 = 0.25;

            let ir = (255.99 * clr.x()) as i32;
            let ig = (255.99 * clr.y()) as i32;
            let ib = (255.99 * clr.z()) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}
