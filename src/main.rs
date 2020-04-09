mod types;

fn main() {
    let width  = 200;
    let height = 100;

    println!("P3\n{} {}\n255\n", width, height);
    for j in (0..height).rev() {
        for i in 0..width {
            let r = i as f64 / width as f64;
            let g = j as f64 / height as f64;
            let b = 0.2 as f64;
            let ir = (255.999 * r).floor();
            let ig = (255.999 * g).floor();
            let ib = (255.999 * b).floor();
            println!("{} {} {}", ir, ig, ib);
        }
    }
}
