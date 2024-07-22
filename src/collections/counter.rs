#[derive(Default)]
pub struct SignedCounter  {
    value: isize,
}

pub struct UnsignedCounter  {
    value: isize,
}

impl SignedCounter {
    pub fn new(value: isize) -> Self {
        Self { value }
    }

    pub fn prev_signed(self) -> Self {
        Self {value: self.value - 1}
    }

    pub fn next_signed(self) -> Self {
        Self {value: self.value + 1}
    }
}


impl UnsignedCounter {
    pub fn next_unsigned(self) -> Self {
        Self {value: self.value + 1}
    }
}

impl Default for UnsignedCounter {
    fn default() -> Self {
        Self { value: 0 }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_default_signed_counter() {
        assert_eq!(SignedCounter::default().value, 0);
    }

    #[test]
    fn test_default_unsigned_counter() {
        assert_eq!(UnsignedCounter::default().value, 0);
    }

    #[test]
    fn test_prev_signed() {
        let counter: SignedCounter = SignedCounter { value: 1 };
        assert_eq!(counter.prev_signed().value, 0);
    }

    #[test]
    fn test_next_signed() {
        let counter: SignedCounter = SignedCounter {value: 1};
        assert_eq!(counter.prev_signed().value, 0);
    }

    #[test]
    fn test_next_unsigned() {
        let counter: UnsignedCounter = UnsignedCounter { value: 1 };
        assert_eq!(counter.next_unsigned().value, 2)
    }
}