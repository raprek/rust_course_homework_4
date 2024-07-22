#[derive(Default)]
pub struct Pair {
    value: (i32, i32),
}

impl Pair {
    pub fn pair_vector_sum(a: Pair, b: Pair) -> Pair {
        Pair{value: (a.value.0 + b.value.0, a.value.1 + b.value.1)}
    }

    pub fn pair_scalar_sum(a: Pair, b: Pair) -> i32 {
        a.value.0 + a.value.1 + b.value.0 + b.value.1
    }

}



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_pair_vector_sum() {
        let mut test_pair = Pair::pair_vector_sum(Pair::default(), Pair::default());
        assert_eq!(test_pair.value.0, Pair{value: (0, 0)}.value.0);
        assert_eq!(test_pair.value.1, Pair{value: (0, 0)}.value.1);

        test_pair = Pair::pair_vector_sum(Pair{value: (1, 2) }, Pair{value: (3, 4)} );
        assert_eq!(test_pair.value.0, 4);
        assert_eq!(test_pair.value.1, 6);
    }

    #[test]
    fn test_pair_scalar_sum() {
        assert_eq!(Pair::pair_scalar_sum(Pair{value: (1, 2) }, Pair{value: (3, 4)}), 10);
    }

    #[test]
    fn test_default_pair() {
        assert_eq!(Pair::default().value, Pair{value: (0, 0)}.value);
    }
}