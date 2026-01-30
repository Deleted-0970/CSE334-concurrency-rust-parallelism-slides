//! Run this file with `cargo test --test 07_calc`.

// TODO: Create an enum that represents three simple mathematical operations
// (Add a number, subtract a number, and clamp a number).
// Clamping a number `x` to the interval [l, r] means that after the operation
// is performed, `x` cannot be smaller than `l` and cannot be larger than `r`.
//
// Then implement the `perform_calculation` function
// (see tests) that receives a single `i32` number and the enum, which represents which
// operation should be performed on it.
// Hint: max(..) and min(..) methods of `i32` might come in handy.

enum Op {
    Add(i32),
    Sub(i32),
    // Divide,
    Clamp{high: i32, low: i32},
}

fn perform_calculation(a: i32, op: Op) -> i32 {
    let a = match op {
        Op::Add(b) => a + b,
        Op::Sub(b) => a - b,
        Op::Clamp { high, low } => {
            
            a.clamp(low, high)
        },
    };
    a
}

/// Below you can find a set of unit tests.
#[cfg(test)]
mod tests {
    use crate::{Op, perform_calculation};

    #[test]
    fn calc_add() {
        assert_eq!(perform_calculation(0, Op::Add(1)), 1);
        assert_eq!(perform_calculation(3, Op::Add(10)), 13);
    }

    #[test]
    fn calc_sub() {
        assert_eq!(perform_calculation(0, Op::Sub(10)), -10);
        assert_eq!(perform_calculation(3000, Op::Sub(-5)), 3005);
    }

    /// Clamp makes sure that a value is between a minimum and maximum value
    /// (inclusive).
    /// clamp(1, 0, 8)     = 1
    /// clamp(-5, 0, 8)    = 0
    /// clamp(-5, -15, 9)  = -5
    /// clamp(50, 0, 8)    = 8
    /// clamp(50, 0, 80)   = 50
    #[test]
    fn calc_clamp() {
        assert_eq!(perform_calculation(0, Op::Clamp { low: 0, high: 0 }), 0);
        assert_eq!(perform_calculation(5, Op::Clamp { low: 0, high: 0 }), 0);
        assert_eq!(perform_calculation(3, Op::Clamp { low: 2, high: 8 }), 3);
        assert_eq!(perform_calculation(-5, Op::Clamp { low: 0, high: 10 }), 0);
        assert_eq!(perform_calculation(50, Op::Clamp { low: 3, high: 10 }), 10);
        assert_eq!(perform_calculation(50, Op::Clamp { low: 3, high: 100 }), 50);
    }
}
