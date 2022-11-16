/// Solution to [Composition of Logs](https://bartoszmilewski.com/2014/12/23/kleisli-categories/) Challenge

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub struct Optional<T> {
    pub is_valid: bool,
    pub value: T,
}

impl<T: Default> Optional<T> {
    #[allow(dead_code)]
    pub fn invalid() -> Self {
        Self {
            is_valid: false,
            value: Default::default(),
        }
    }

    #[allow(dead_code)]
    pub fn valid(value: T) -> Self {
        Self {
            is_valid: true,
            value,
        }
    }
}

fn compose<T, U, V, F, G>(f: F, g: G) -> impl FnMut(T) -> Optional<V>
where
    V: Default,
    F: Fn(T) -> Optional<U>,
    G: Fn(U) -> Optional<V>,
{
    move |x: T| {
        let a = f(x);

        if a.is_valid {
            g(a.value)
        } else {
            Optional::invalid()
        }
    }
}

#[allow(dead_code)]
fn identity<T: Default>(x: T) -> Optional<T>
{
    Optional::valid(x)
}

fn safe_root(x: f64) -> Optional<f64> {
    if x >= 0f64 {
        Optional::valid(x.sqrt())
    } else {
        Optional::invalid()
    }
}

fn safe_reciprocal(x: f64) -> Optional<f64> {
    if x != 0f64 {
        Optional::valid(1f64 / x)
    } else {
        Optional::invalid()
    }
}

#[allow(dead_code)]
pub fn safe_root_reciprocal(x: f64) -> Optional<f64> {
    compose(safe_reciprocal, safe_root)(x)
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn identity_returns_optional_x() {
        assert_eq!(Optional::valid(1), identity(1));
    }

    #[test]
    #[allow(unused_must_use)]
    fn safe_root_returns_valid_when_x_is_not_negative() {
        let opt = safe_root(4f64);

        assert!(opt.is_valid);
        relative_eq!(4f64.sqrt(), opt.value, epsilon=f64::EPSILON);
    }

    #[test]
    #[allow(unused_must_use)]
    fn safe_root_returns_invalid_when_x_is_negative() {
        let opt = safe_root(-2f64);

        assert!(!opt.is_valid);
        assert_eq!(Optional::invalid(), opt);
    }

    #[test]
    #[allow(unused_must_use)]
    fn safe_reciprocal_returns_valid_when_x_is_not_zero() {
        let opt = safe_reciprocal(4f64);

        assert!(opt.is_valid);
        relative_eq!(1f64 / 4f64, opt.value, epsilon=f64::EPSILON);
    }

    #[test]
    #[allow(unused_must_use)]
    fn safe_reciprocal_returns_invalid_when_x_is_zero() {
        let opt = safe_reciprocal(0f64);

        assert!(!opt.is_valid);
        assert_eq!(Optional::invalid(), opt);
    }

    #[test]
    #[allow(unused_must_use)]
    fn safe_root_reciprocal_returns_square_root_of_reciprocal_of_x_when_x_is_not_negative_and_zero() {
        let opt = safe_root_reciprocal(4f64);

        assert!(opt.is_valid);
        relative_eq!((1f64 / 4f64).sqrt(), opt.value, epsilon=f64::EPSILON);
    }

    #[test]
    #[allow(unused_must_use)]
    fn safe_root_reciprocal_returns_invalid_when_x_is_negative_or_zero() {
        let opt = safe_root_reciprocal(0f64);

        assert!(!opt.is_valid);
        assert_eq!(Optional::invalid(), opt);

        let opt = safe_root_reciprocal(-2f64);

        assert!(!opt.is_valid);
        assert_eq!(Optional::invalid(), opt);
    }
}
