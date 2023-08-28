use rand::Rng;
use ray_tracer::core::hittable::{HittableList, List};
use ray_tracer::core::sphere::Sphere;
use ray_tracer::core::{vec3::Vec3, colour::colour};
use ray_tracer::setup::camera::Camera;
// use ray_tracer::configuration::get_configuration;


fn main() {
    // let config = get_configuration().expect("Couldn't read configuration");

    let aspect_ratio = 16.0 / 9.0;

    let width = 400;
    let height = width as f32 / aspect_ratio;
    let max = 255;
    let samples = 100;
    let mut rng = rand::thread_rng();

    let camera = Camera::new();

    let mut list: List = Vec::new();
    list.push(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    list.push(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));
    let world = HittableList::new(list);

    // println!("{} : {}", width, height);
    println!("P3\n{} {}\n{}", width, height, max);

    for j in (0..height as i32).rev() {
        for i in 0..width {
            let mut clr = Vec3::default();

            for _ in 0..samples {
                let u = (i as f32 + rng.gen::<f32>()) / width as f32;
                let v = (j as f32 + rng.gen::<f32>()) / height as f32;
    
                // let ray = Ray::new(origin, lower_left_corner + horizontal * u + vertical * v - origin);
                let ray = camera.get_ray(u, v);
                clr = clr + colour(&ray, &world);
            }
            // let b: f32 = 0.25;
            clr = clr / samples as f32;

            let ir = (255.99 * clr.x()) as i32;
            let ig = (255.99 * clr.y()) as i32;
            let ib = (255.99 * clr.z()) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}
