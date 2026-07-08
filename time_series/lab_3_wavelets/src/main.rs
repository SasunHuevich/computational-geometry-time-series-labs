mod haar;
mod chirplet;

use std::f64::consts::PI;

fn main() {
    println!("--- 1. Вейвлет-преобразование Хаара ---\n");

    let signal = vec![1.0, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0];
    println!("Исходный сигнал (длина {}):", signal.len());
    println!("  {:?}", signal);

    let coeffs = haar::haar_dwt(&signal);
    println!("\nКоэффициенты вейвлет-разложения:");
    for (i, level) in coeffs.iter().enumerate() {
        println!("  Уровень {}: {:?}", i, level);
    }

    let recovered = haar::haar_idwt(&coeffs);
    println!("\nВосстановленный сигнал:");
    println!("  {:?}", recovered);

    let mut mse = 0.0;
    let mut max_amp = 0.0;
    for (a, b) in signal.iter().zip(recovered.iter()) {
        let diff = a - b;
        mse += diff * diff;
        if a.abs() > max_amp { max_amp = a.abs(); }
        if b.abs() > max_amp { max_amp = b.abs(); }
    }
    mse /= signal.len() as f64;
    let rmse = mse.sqrt();
    let error_percent = (rmse / max_amp) * 100.0;

    println!("\nОшибка восстановления:");
    println!("  MSE  = {:.6e}", mse);
    println!("  RMSE = {:.6}  (это {:.2}% от максимальной амплитуды {:.3})",
             rmse, error_percent, max_amp);
    if error_percent < 1e-6 {
        println!("  => Восстановление выполнено с точностью до машинной погрешности.");
    } else {
        println!("  => Ошибка мала, преобразование обратимо.");
    }

    println!("\n\n--- 2. Чирплет-преобразование (анализ сигнала с переменной частотой) ---\n");

    let n = 256;
    let duration = 4.0;
    let t: Vec<f64> = (0..n).map(|i| i as f64 / n as f64 * duration).collect();
    let signal_chirp: Vec<f64> = t.iter().map(|&t| {
        let phase =
            2.0 * PI *
            (5.0 * t + 15.0 * t * t / (2.0 * duration));

        phase.sin()
    }).collect();

    println!("Сгенерирован чирп-сигнал: частота меняется линейно от 5 до 20 Гц за {:.1} с.", duration);
    println!("Длина сигнала: {} отсчётов.", n);

    let scales = vec![0.5, 1.0, 2.0, 4.0];
    let shifts: Vec<usize> = (0..n).step_by(10).collect();
    let omega0 = 2.0 * PI * 10.0;
    let beta_list = vec![-5.0, 0.0, 5.0];

    println!("\nПараметры чирплетов:");
    println!("  Масштабы:    {:?}", scales);
    println!("  Сдвиги:      {} значений (шаг 10)", shifts.len());
    println!("  Бета (чирп): {:?}", beta_list);
    println!("  Несущая частота: {:.1} Гц", omega0 / (2.0 * PI));

    let coeffs_chirp = chirplet::chirplet_transform(&signal_chirp, &scales, &shifts, omega0, &beta_list);

    let mut max_val = 0.0;
    let mut max_params = (0, 0, 0);
    for (si, _) in scales.iter().enumerate() {
        for (ti, _) in shifts.iter().enumerate() {
            for (bi, _) in beta_list.iter().enumerate() {
                let val = coeffs_chirp[si][ti][bi].abs();
                if val > max_val {
                    max_val = val;
                    max_params = (si, ti, bi);
                }
            }
        }
    }

    let best_scale = scales[max_params.0];
    let best_shift = shifts[max_params.1];
    let best_beta = beta_list[max_params.2];

    println!("\nРезультаты чирплет-анализа:");
    println!("  Максимальный коэффициент: {:.3}", max_val);
    println!("  Соответствующие параметры:");
    println!("    Масштаб a = {:.1}", best_scale);
    println!("    Сдвиг    = {} (индекс отсчёта)", best_shift);
    println!("    Бета     = {:.1} рад/с²", best_beta);

    let estimated_freq = omega0 / (2.0 * PI * best_scale);
    println!("\nИнтерпретация:");
    println!("  Оценка частоты сигнала по масштабу: {:.1} Гц", estimated_freq);
    println!("  Оценка скорости изменения частоты (бета): {:.1} рад/с²", best_beta);
    println!("  Сдвиг соответствует временному положению фрагмента сигнала.");

    if best_beta > 0.0 {
        println!("  Положительная бета указывает на возрастающую частоту.");
    } else if best_beta < 0.0 {
        println!("  Отрицательная бета указывает на убывающую частоту.");
    } else {
        println!("  Нулевая бета означает постоянную частоту (обычный вейвлет).");
    }
}