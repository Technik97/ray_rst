use rand::Rng;
use ray_tracer::core::hittable::{HittableList, List};
use ray_tracer::core::sphere::Sphere;
use ray_tracer::core::{vec3::Vec3, colour::colour};
use ray_tracer::materials::dielectric::Dielectric;
use ray_tracer::materials::lambertian::Lambertian;
use ray_tracer::materials::material::Material;
use ray_tracer::materials::metal::Metal;
use ray_tracer::setup::camera::Camera;

fn main() {
    let aspect_ratio = 16.0 / 9.0;

    let width = 480;
    let height = width as f32 / aspect_ratio;
    let max = 255;
    let samples = 100;
    let mut rng = rand::thread_rng();

    let camera = Camera::new();
    let material_ground = Material::Lambertian(Lambertian::new(Vec3::new(0.8, 0.8, 0.0)));
    let material_center = Material::Lambertian(Lambertian::new(Vec3::new(0.1, 0.2, 0.5)));
    let material_left = Material::Dielectric(Dielectric::new(1.5));
    let material_right = Material::Metal(Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.0));

    let mut list: List = Vec::new();
    list.push(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, material_ground)));
    list.push(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, material_center)));
    list.push(Box::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, material_left)));
    list.push(Box::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), -0.4, material_left)));
    list.push(Box::new(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, material_right)));
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
                clr = clr + colour(&ray, &world, 0);
            }
            // let b: f32 = 0.25;
            clr = clr / samples as f32;

            clr = Vec3::new(clr.x().sqrt(), clr.y().sqrt(), clr.z().sqrt());

            let ir = (255.99 * clr.x()) as i32;
            let ig = (255.99 * clr.y()) as i32;
            let ib = (255.99 * clr.z()) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}
