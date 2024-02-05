fn perlin_noise_curve(t: f64) -> f64 {
    t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
}

// get inversed function of perlin_noise_curve
pub fn inversed_perlin_noise_curve(y: f64) -> f64 {
    let mut low = 0.0f64;
    let mut high = 1.0f64;
    let mut mid = (low + high) / 2.0;
    while (high - low).abs() > f64::EPSILON {
        if perlin_noise_curve(mid) < y {
            low = mid;
        } else {
            high = mid;
        }
        mid = (low + high) / 2.0;
    }
    mid
}
