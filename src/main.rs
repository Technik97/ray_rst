use ray_tracer::core::hittable::Hittable;
use ray_tracer::core::hittable::{HittableList, List};
use ray_tracer::core::sphere::Sphere;
use ray_tracer::core::{ray::Ray, vec3::Vec3, colour::colour};
// use ray_tracer::configuration::get_configuration;


fn main() {
    // let config = get_configuration().expect("Couldn't read configuration");

    let aspect_ratio = 16.0 / 9.0;

    let width = 400;
    let height = width as f32 / aspect_ratio;
    let max = 255;

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let mut list: List = Vec::new();
    list.push(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    list.push(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));
    let world = HittableList::new(list);

    // println!("{} : {}", width, height);
    println!("P3\n{} {}\n{}", width, height, max);

    let origin = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - Vec3::new(0.0, 0.0, focal_length);
    

    for j in (0..height as i32).rev() {
        for i in 0..width {
            let u = i as f32 / width as f32;
            let v = j as f32 / height as f32;

            let ray = Ray::new(origin, lower_left_corner + horizontal * u + vertical * v - origin);
            let clr = colour(&ray, &world);

            // let b: f32 = 0.25;

            let ir = (255.99 * clr.x()) as i32;
            let ig = (255.99 * clr.y()) as i32;
            let ib = (255.99 * clr.z()) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}
