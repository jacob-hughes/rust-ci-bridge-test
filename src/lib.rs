pub fn add(left: u64, right: u64) -> u64 {
    return left + right;
}

pub fn sub(left: u64, right: u64) -> u64 {
    return left - right;
}

struct Hello(usize);

impl Drop for Hello {
    fn drop(&mut self) {
        panic!("This is a test");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 5);
    }

    #[test]
    fn add_zero() {
        assert_eq!(add(0, 0), 0);
        assert_eq!(add(7, 0), 7);
        assert_eq!(add(0, 7), 7);
    }

    #[test]
    fn add_is_commutative() {
        assert_eq!(add(3, 9), add(9, 3));
    }

    #[test]
    fn add_large() {
        assert_eq!(add(u64::MAX - 1, 1), u64::MAX);
    }

    #[test]
    #[should_panic]
    fn add_overflow_panics() {
        add(u64::MAX, 1);
    }

    #[test]
    fn sub_basic() {
        assert_eq!(sub(10, 3), 7);
    }

    #[test]
    fn sub_same_is_zero() {
        assert_eq!(sub(42, 42), 0);
    }

    #[test]
    fn sub_zero() {
        assert_eq!(sub(5, 0), 5);
    }

    #[test]
    fn sub_undoes_add() {
        assert_eq!(sub(add(12, 30), 30), 12);
    }

    #[test]
    #[should_panic]
    fn sub_underflow_panics() {
        sub(1, 2);
    }
}
