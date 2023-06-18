fn main() {
    const IMG_HEIGHT: i32 = 256;
    const IMG_WIDTH: i32 = 256;

    println!("P\n{} {} \n255\n", IMG_WIDTH, IMG_HEIGHT);

    let mut j = IMG_HEIGHT - 1;
    while j > 0 {
        // println!("{}", j);
        let mut i = 0;
        while i < IMG_WIDTH {
            let r: f32 = (i / (IMG_WIDTH - 1)) as f32;
            let g: f32 = (j / (IMG_HEIGHT - 1)) as f32;
            let b: f32 = 0.25;

            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;

            println!("{} {} {}", ir , ig , ib);
            i += 1;
        }
        j -= 1;
    }
}
