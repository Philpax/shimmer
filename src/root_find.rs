pub fn secant<F>(mut x0: f32,
                 mut x1: f32,
                 precision: f32,
                 max_iterations: usize,
                 f: F)
                 -> Option<f32>
    where F: Fn(f32) -> f32
{
    let mut iteration = 0;

    loop {
        let old_x1 = x1;
        x1 = x1 - f(x1) * ((x1 - x0) / (f(x1) - f(x0)));
        x0 = old_x1;

        iteration += 1;

        if (x1 - x0).abs() <= precision {
            break;
        }

        if iteration > max_iterations || x1.is_infinite() {
            return None;
        }
    }

    Some(x1)
}

pub fn ray_march<F>(min: f32, max: f32, step: f32, f: F) -> Option<f32>
    where F: Fn(f32) -> f32
{
    let mut current = min;
    let mut iteration = 0;

    while current < max {
        if f(current) <= 0.0 {
            return Some(current);
        }

        iteration += 1;
        current = min + step * iteration as f32;
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn secant_test() {
        assert_eq!(secant(10.0, 30.0, 0.001, 32, |x: f32| x.powi(2) - 612.0),
                   Some(24.738634));
        assert_eq!(secant(0.0, 1.0, 0.001, 32, |x: f32| x.powi(2) + 1.0), None);
    }

    #[test]
    fn ray_march_test() {
        assert_eq!(ray_march(0.0, 10.0, 0.1, |x| 5.0 - x), Some(5.0));
        assert_eq!(ray_march(0.0, 10.0, 0.1, |x| 12.0 - x), None);
    }
}
