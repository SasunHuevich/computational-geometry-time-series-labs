use std::f64::consts::PI;

fn sinc(x: f64) -> f64 {
    if x.abs() < 1e-12 {
        1.0
    } else {
        (PI * x).sin() / (PI * x)
    }
}

fn sinc_interp(samples: &[f64], t: f64, fs: f64, window: usize) -> f64 {
    let t_idx = t * fs;
    let mut sum = 0.0;
    // Ограничим диапазон отсчётов, чтобы не выходить за границы
    let start = (t_idx - window as f64).ceil() as isize;
    let end = (t_idx + window as f64).floor() as isize;
    for i in start..=end {
        if i < 0 || i >= samples.len() as isize {
            continue;
        }
        let idx = i as usize;
        let arg = t_idx - i as f64;
        sum += samples[idx] * sinc(arg);
    }
    sum
}

fn linear_interp(samples: &[f64], t: f64, fs: f64) -> f64 {
    let t_idx = t * fs;
    let i0 = t_idx.floor() as usize;
    let i1 = (i0 + 1).min(samples.len() - 1);
    let frac = t_idx - i0 as f64;
    if i0 >= samples.len() {
        return 0.0;
    }
    samples[i0] * (1.0 - frac) + samples[i1] * frac
}

fn generate_bandlimited_signal(t: f64, freqs: &[(f64, f64)]) -> f64 {
    freqs.iter().map(|(f, amp)| amp * (2.0 * PI * f * t).sin()).sum()
}

fn main() {
    println!("=== Теорема Котельникова (интерполяция) ===\n");
    
    let freqs = vec![(5.0, 1.0), (15.0, 0.7), (20.0, 0.3)];
    let f_max = 20.0;
    let fs_high = 50.0;
    let fs_low = 30.0;

    let t_start = 0.0;
    let t_end = 1.0;

    let num_samples_high = (fs_high * (t_end - t_start)) as usize;
    let num_samples_low = (fs_low * (t_end - t_start)) as usize;

    println!("Сигнал содержит частоты: 5 Гц, 15 Гц, 20 Гц -> f_max = {:.1} Гц", f_max);
    println!("Частота Найквиста = {:.1} Гц", 2.0 * f_max);

    let samples_high: Vec<f64> = (0..num_samples_high)
        .map(|i| {
            let t = t_start + i as f64 / fs_high;
            generate_bandlimited_signal(t, &freqs)
        })
        .collect();

    let samples_low: Vec<f64> = (0..num_samples_low)
        .map(|i| {
            let t = t_start + i as f64 / fs_low;
            generate_bandlimited_signal(t, &freqs)
        })
        .collect();

    let window = 8;
    let test_points = 200;

    let mut max_amp = 0.0;
    for i in 0..test_points {
        let t = t_start + (t_end - t_start) * (i as f64) / (test_points as f64);
        let val = generate_bandlimited_signal(t, &freqs);
        if val.abs() > max_amp {
            max_amp = val.abs();
        }
    }

    println!("Максимальная амплитуда сигнала = {:.3}\n", max_amp);

    let mut mse_sinc_low = 0.0;
    let mut mse_linear_low = 0.0;
    for i in 0..test_points {
        let t = t_start + (t_end - t_start) * (i as f64) / (test_points as f64);
        let true_val = generate_bandlimited_signal(t, &freqs);

        let val_sinc = sinc_interp(&samples_low, t, fs_low, window);
        let val_linear = linear_interp(&samples_low, t, fs_low);

        let err_sinc = val_sinc - true_val;
        let err_linear = val_linear - true_val;
        mse_sinc_low += err_sinc * err_sinc;
        mse_linear_low += err_linear * err_linear;
    }
    mse_sinc_low /= test_points as f64;
    mse_linear_low /= test_points as f64;

    println!("--- Дискретизация с частотой {:.1} Гц (НИЖЕ Найквиста) ---", fs_low);
    println!("  MSE sinc-интерполяции:      {:.6e}", mse_sinc_low);
    println!("  RMSE sinc = {:.4}  (это {:.1}% от максимума)", mse_sinc_low.sqrt(), mse_sinc_low.sqrt() / max_amp * 100.0);
    println!("  MSE линейной интерполяции:   {:.6e}", mse_linear_low);
    println!("  RMSE linear = {:.4}  (это {:.1}% от максимума)", mse_linear_low.sqrt(), mse_linear_low.sqrt() / max_amp * 100.0);
    println!("  => Восстановление НЕВОЗМОЖНО из-за наложения спектров.\n");

    let mut mse_sinc_high = 0.0;
    let mut mse_linear_high = 0.0;
    for i in 0..test_points {
        let t = t_start + (t_end - t_start) * (i as f64) / (test_points as f64);
        let true_val = generate_bandlimited_signal(t, &freqs);

        let val_sinc = sinc_interp(&samples_high, t, fs_high, window);
        let val_linear = linear_interp(&samples_high, t, fs_high);

        let err_sinc = val_sinc - true_val;
        let err_linear = val_linear - true_val;
        mse_sinc_high += err_sinc * err_sinc;
        mse_linear_high += err_linear * err_linear;
    }
    mse_sinc_high /= test_points as f64;
    mse_linear_high /= test_points as f64;
 
    println!("--- Дискретизация с частотой {:.1} Гц (ВЫШЕ Найквиста) ---", fs_high);
    println!("  MSE sinc-интерполяции:      {:.6e}", mse_sinc_high);
    println!("  RMSE sinc = {:.4}  (это {:.1}% от максимума)", mse_sinc_high.sqrt(), mse_sinc_high.sqrt() / max_amp * 100.0);
    println!("  MSE линейной интерполяции:   {:.6e}", mse_linear_high);
    println!("  RMSE linear = {:.4}  (это {:.1}% от максимума)", mse_linear_high.sqrt(), mse_linear_high.sqrt() / max_amp * 100.0);
    println!("  => Восстановление ВОЗМОЖНО с высокой точностью.");
}