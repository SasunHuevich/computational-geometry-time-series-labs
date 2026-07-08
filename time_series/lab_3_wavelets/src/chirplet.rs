
pub fn chirplet(t: f64, sigma: f64, omega0: f64, beta: f64) -> f64 {
    let envelope = (-t * t / (2.0 * sigma * sigma)).exp();
    let phase = omega0 * t + 0.5 * beta * t * t;
    envelope * phase.cos()
}

pub fn chirplet_transform(
    signal: &[f64],
    scales: &[f64],
    shifts: &[usize],
    omega0: f64,
    beta_list: &[f64],
) -> Vec<Vec<Vec<f64>>> {
    let n = signal.len();
    let mut coeffs = vec![vec![vec![0.0; beta_list.len()]; shifts.len()]; scales.len()];
    for (si, &scale) in scales.iter().enumerate() {
        let a = scale;
        for (ti, &shift) in shifts.iter().enumerate() {
            for (bi, &beta) in beta_list.iter().enumerate() {
                let mut sum = 0.0;
                for t in 0..n {
                    let idx = (t as f64 - shift as f64) / a;
                    if idx.abs() < 5.0 {
                        let psi = chirplet(idx, 1.0, omega0, beta) / a.sqrt();
                        sum += signal[t] * psi;
                    }
                }
                coeffs[si][ti][bi] = sum;
            }
        }
    }
    coeffs
}