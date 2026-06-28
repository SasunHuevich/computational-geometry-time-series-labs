use std::f64::consts::PI;

/// Прямое ДПФ: вход – вектор комплексных чисел (как (re, im)).
/// Выход – вектор комплексных амплитуд.
fn dft(signal: &[(f64, f64)]) -> Vec<(f64, f64)> {
    let n = signal.len();
    let mut spectrum = vec![(0.0, 0.0); n];
    for k in 0..n {
        let mut sum_re = 0.0;
        let mut sum_im = 0.0;
        for (t, &(x_re, x_im)) in signal.iter().enumerate() {
            let c = (2.0 * PI * k as f64 * t as f64 / n as f64).cos();
            let s = (2.0 * PI * k as f64 * t as f64 / n as f64).sin();
            sum_re += x_re * c + x_im * s;
            sum_im += x_im * c - x_re * s;
        }
        spectrum[k] = (sum_re, sum_im);
    }
    spectrum
}

/// Обратное ДПФ
fn idft(spectrum: &[(f64, f64)]) -> Vec<(f64, f64)> {
    let n = spectrum.len();
    let mut signal = vec![(0.0, 0.0); n];
    for t in 0..n {
        let mut sum_re = 0.0;
        let mut sum_im = 0.0;
        for k in 0..n {
            let (x_re, x_im) = spectrum[k];
            let c = (2.0 * PI * k as f64 * t as f64 / n as f64).cos();
            let s = (2.0 * PI * k as f64 * t as f64 / n as f64).sin();
            sum_re += x_re * c - x_im * s;
            sum_im += x_im * c + x_re * s;
        }
        signal[t] = (sum_re / n as f64, sum_im / n as f64);
    }
    signal
}

/// Генерирует сигнал как сумму синусоид (вещественный)
fn generate_signal(n: usize, freqs: &[(f64, f64)]) -> Vec<f64> {
    let mut signal = vec![0.0; n];
    for t in 0..n {
        let mut val = 0.0;
        for &(f, amp) in freqs {
            val += amp * (2.0 * PI * f * t as f64).sin();
        }
        signal[t] = val;
    }
    signal
}

/// Вычисляет амплитудный спектр (модуль комплексного числа)
fn amplitude_spectrum(spectrum: &[(f64, f64)]) -> Vec<f64> {
    spectrum.iter().map(|(re, im)| (re * re + im * im).sqrt()).collect()
}

/// Находит индексы пиков (локальных максимумов)
fn find_peaks(amplitudes: &[f64], threshold: f64) -> Vec<usize> {
    let mut peaks = Vec::new();
    for i in 1..amplitudes.len() - 1 {
        if amplitudes[i] > amplitudes[i - 1] && amplitudes[i] > amplitudes[i + 1] && amplitudes[i] > threshold {
            peaks.push(i);
        }
    }
    peaks
}

fn main() {
    let n = 64; // количество отсчётов
    let fs = 100.0; // частота дискретизации (Гц)
    let expected_freqs = vec![(10.0 / fs, 1.0), (25.0 / fs, 0.5)];
    let signal_real = generate_signal(n, &expected_freqs);

    let signal_complex: Vec<(f64, f64)> = signal_real.iter().map(|&x| (x, 0.0)).collect();

    let spectrum = dft(&signal_complex);
    let amplitudes = amplitude_spectrum(&spectrum);

    println!("Сигнал (первые 10): {:?}", &signal_real[..10]);
    println!("Амплитудный спектр (первые 10): {:?}", &amplitudes[..10]);

    // Ищем пики только в положительных частотах (от 1 до n/2)
    let half = n / 2;
    let threshold = 0.1;
    let mut peaks = Vec::new();
    for i in 1..=half {
        if i > 0 && i < n - 1 && amplitudes[i] > amplitudes[i - 1] && amplitudes[i] > amplitudes[i + 1] && amplitudes[i] > threshold {
            peaks.push(i);
        }
    }

    println!("\nНайденные пики (положительные частоты, индексы от 1 до {}): {:?}", half, peaks);
    println!("Частоты и амплитуды (нормированные на n/2):");
    for &idx in &peaks {
        let freq = idx as f64 * fs / n as f64;
        // Нормируем амплитуду: для вещественного сигнала амплитуда = 2*|X_k|/N
        let amplitude = 2.0 * amplitudes[idx] / n as f64;
        println!("  {:6.2} Гц -> амплитуда {:.3}", freq, amplitude);
    }

    println!("\nОжидаемые частоты (заданы в сигнале):");
    for (rel_freq, amp) in &expected_freqs {
        let freq = rel_freq * fs;
        println!("  {:6.2} Гц -> амплитуда {:.3}", freq, amp);
    }

    let recovered_complex = idft(&spectrum);
    let recovered_real: Vec<f64> = recovered_complex.iter().map(|(re, _)| *re).collect();

    let mut mse = 0.0;
    for i in 0..n {
        let diff = signal_real[i] - recovered_real[i];
        mse += diff * diff;
    }
    mse /= n as f64;
    println!("\nСреднеквадратичная ошибка восстановления: {:.6e}", mse);
    println!("Если MSE близка к нулю, преобразование обратимо и частоты определены верно.");
}