#[macro_use]
#[cfg(test)]
extern crate approx;

mod optional;

use std::{collections::HashMap, hash::Hash};

pub fn id<T>(x: T) -> T {
    x
}

pub fn compose<T, U, V, F, G>(g: G, f: F) -> impl Fn(T) -> V
where
    F: Fn(T) -> U,
    G: Fn(U) -> V,
{
    move |x: T| g(f(x))
}

pub fn memoize<T, U, F>(f: F) -> impl FnMut(T) -> U
where
    T: Eq + Hash + Copy,
    U: Copy,
    F: Fn(T) -> U,
{
    let mut memo = HashMap::<T, U>::new();

    move |x: T| {
        if let Some(v) = memo.get(&x) {
            return *v;
        } else {
            let v = f(x);
            memo.insert(x, v);
            v
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn id_returns_back_its_argument() {
        assert_eq!(id(1), 1);
        assert_eq!(id('s'), 's');
        assert_eq!(id(vec![0]), vec![0]);
    }

    #[test]
    fn compose_takes_two_functions_and_returns_one_function() {
        let f = |x: i32| format!("{}", x);
        let g = |x: String| x.chars().nth(0);
        assert_eq!(compose(g, f)(123), Some('1'));
    }

    #[test]
    fn compose_respects_identify() {
        let f = |x: i32| x * 4;

        assert_eq!(compose(id, f)(1), 4);
        assert_eq!(compose(f, id)(2), 8);
    }

    #[test]
    fn momoize_returns_a_function_behaves_like_original() {
        let f = |x: i32| x * 2;
        let mut memo = memoize(f);

        assert_eq!(f(2), memo(2));
        assert_eq!(f(4), memo(4));
        // ensure memo returns the same value as the first call
        assert_eq!(f(2), memo(2));
    }
}
