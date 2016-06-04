fn secant<F>(mut x0: f32, mut x1: f32, precision: f32, max_iterations: usize, f: F) -> Option<f32>
    where F: Fn(f32) -> f32
{
    let mut iteration = 0;
    loop {
        let old_x1 = x1;
        x1 = x1 - f(x1) * ((x1 - x0) / (f(x1) - f(x0)));
        x0 = old_x1;

        iteration += 1;

        if x1 - x0 > precision {
            break;
        }

        if iteration > max_iterations {
            return None;
        }
    }

    Some(x1)
}

#[cfg(test)]
mod tests {
    use super::secant;

    #[test]
    fn secant_test() {
        assert_eq!(secant(10.0, 30.0, 0.01, 32, |x: f32| x.powi(2) - 612.0),
                   Some(24.545454));
    }
}
