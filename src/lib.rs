use glam::Vec3;

pub const NUM_OCTAVES: usize = 4;

pub fn fbm(x: Vec3, h: f32) -> f32 {
    let g = (-h).exp2();
    let mut f = 1.0;
    let mut a = 1.0;
    let mut t = 0.0;

    for _ in 0..NUM_OCTAVES {
        t += a * noise(f * x);
        f *= 2.0;
        a *= g;
    }

    t
}

fn noise(v: Vec3) -> f32 {
    v.x * v.y * v.z
}
