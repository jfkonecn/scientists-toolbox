use approx::relative_eq;

#[derive(Debug, PartialEq)]
pub enum RootFinderErr {
    ToleranceBelowZero,
    MaxIterationsReached,
}

pub fn secant_method(f: impl Fn(f64) -> f64, x0: f64, tol: f64) -> Result<f64, RootFinderErr> {
    let max_iter = 50;
    if tol < 0.0 {
        return Err(RootFinderErr::ToleranceBelowZero);
    }

    fn solver(
        (x0, y0): (f64, f64),
        (x1, y1): (f64, f64),
        tries: i64,
        f: impl Fn(f64) -> f64,
        max_iter: i64,
        tol: f64,
    ) -> Result<f64, RootFinderErr> {
        // let x2 = x1 - y1 * (x1 - x0) / (y1 - y0);

        let x2 = if y1.abs() > y0.abs() {
            (-y0 / y1 * x1 + x0) / (1f64 - y0 / y1)
        } else {
            (-y1 / y0 * x0 + x1) / (1f64 - y1 / y0)
        };
        let y2 = f(x2);
        if relative_eq!(0.0, y2, epsilon = tol) {
            Ok(x2)
        } else if tries > max_iter {
            Err(RootFinderErr::MaxIterationsReached)
        } else {
            solver((x1, y1), (x2, y2), tries + 1, f, max_iter, tol)
        }
    }

    let eps = 1e-4;
    let x1_shift = x0 * (1f64 + eps);
    let x1 = x1_shift + if x1_shift >= 0f64 { eps } else { -eps };
    let y0 = f(x0);
    let y1 = f(x1);
    solver((x0, y0), (x1, y1), 0, f, max_iter, tol)
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    macro_rules! secant_method_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let f : fn(f64) -> f64 = $value;
                let x = secant_method(f, 3.0, 1e-6).unwrap();
                assert_approx_eq!(0.0, f(x));
            }
        )*
        }
    }

    secant_method_tests! {
        secant_method_01: |x| x * x - 2.0 * x - 1.0,
        secant_method_02: |x| x.exp() - x.sin(),
    }

    #[test]
    fn secant_should_rtn_err_if_negative_torr() {
        assert_eq!(
            RootFinderErr::ToleranceBelowZero,
            secant_method(|x| x, 3.0, -1e-6).unwrap_err()
        );
    }

    #[test]
    fn secant_should_rtn_err_if_cannot_converge() {
        assert_eq!(
            RootFinderErr::MaxIterationsReached,
            secant_method(|_| 1f64, 3.0, 1e-15).unwrap_err()
        );
    }
}
