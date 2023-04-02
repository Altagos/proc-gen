use proc_gen::fbm;
use glam::Vec3;

fn main() {
    let x = fbm(Vec3::new(rand::random(), 0.5, 2.0), 0.5);
    println!("{x}");
}
