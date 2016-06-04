use scene;

pub fn secant<F>(mut x0: f32,
                 mut x1: f32,
                 precision: f32,
                 max_iterations: usize,
                 f: F)
                 -> Option<f32>
    where F: Fn(f32) -> scene::Point
{
    let mut iteration = 0;

    loop {
        let old_x1 = x1;

        let f_x0 = f(x0);
        let f_x1 = f(x1);

        x1 = x1 - f_x1.value * ((x1 - x0) / (f_x1.value - f_x0.value));
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
    where F: Fn(f32) -> scene::Point
{
    let mut current = min;
    let mut iteration = 0;

    while current < max {
        if f(current).value <= 0.0 {
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
    use scene;

    #[test]
    fn secant_test() {
        assert_eq!(secant(10.0, 30.0, 0.001, 32, |x: f32| {
                       scene::Point {
                           value: x.powi(2) - 612.0,
                           colour: scene::Colour::zero(),
                       }
                   }),
                   Some(24.738634));

        assert_eq!(secant(0.0, 1.0, 0.001, 32, |x: f32| {
                       scene::Point {
                           value: x.powi(2) + 1.0,
                           colour: scene::Colour::zero(),
                       }
                   }),
                   None);
    }

    #[test]
    fn ray_march_test() {
        assert_eq!(ray_march(0.0, 10.0, 0.1, |x| {
                       scene::Point {
                           value: 5.0 - x,
                           colour: scene::Colour::zero(),
                       }
                   }),
                   Some(5.0));

        assert_eq!(ray_march(0.0, 10.0, 0.1, |x| {
                       scene::Point {
                           value: 12.0 - x,
                           colour: scene::Colour::zero(),
                       }
                   }),
                   None);
    }
}
