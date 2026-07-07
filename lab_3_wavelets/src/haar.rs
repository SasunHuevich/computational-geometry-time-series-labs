/// Прямое одномерное дискретное вейвлет-преобразование
pub fn haar_dwt(signal: &[f64]) -> Vec<Vec<f64>> {
    let mut coeffs = Vec::new();
    let mut current = signal.to_vec();
    let mut n = current.len();
    while n > 1 {
        let mut approx = Vec::with_capacity(n / 2);
        let mut detail = Vec::with_capacity(n / 2);
        for i in (0..n).step_by(2) {
            let a = current[i];
            let b = current[i + 1];
            approx.push((a + b) / 2.0_f64.sqrt());
            detail.push((a - b) / 2.0_f64.sqrt());
        }
        coeffs.push(detail);
        current = approx;
        n = current.len();
    }
    coeffs.push(current);
    coeffs.reverse();
    coeffs
}

/// Обратное вейвлет-преобразование
pub fn haar_idwt(coeffs: &[Vec<f64>]) -> Vec<f64> {
    let mut current = coeffs[0].clone();
    for level in 1..coeffs.len() {
        let detail = &coeffs[level];
        let mut reconstructed = Vec::with_capacity(current.len() * 2);
        for i in 0..current.len() {
            let a = current[i];
            let d = detail[i];
            reconstructed.push((a + d) / 2.0_f64.sqrt());
            reconstructed.push((a - d) / 2.0_f64.sqrt());
        }
        current = reconstructed;
    }
    current
}