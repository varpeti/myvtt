use three_d::Vec3;

pub fn len(v: Vec3) -> f32 {
    (v.x.powf(2.) + v.y.powf(2.) + v.z.powf(2.)).sqrt()
}
