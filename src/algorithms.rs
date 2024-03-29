use chrono_probe::input::InputSet;

use crate::input::InputString;

/// Representation of an algorithm &\[[u8]] -> [usize]
pub struct Algorithm {
    /// The name of the algorithm
    pub name: &'static str,
    /// The function implemented by the algorithm
    pub function: fn(&InputString) -> usize,
}

impl Algorithm {
    /// Compute empirically the expected value of the algorithm with the given input set.
    pub fn expected_value(&self, input_set: &InputSet<InputString>) -> f64 {
        let flattened_input_set = input_set
            .inputs
            .iter()
            .flatten()
            .collect::<Vec<&InputString>>();
        let size = flattened_input_set.len();
        let mut sum = 0.0;
        for input in flattened_input_set {
            sum += (self.function)(input) as f64;
        }
        sum / size as f64
    }
}

// Some predefined algorithms for finding the period of a string:

/// The naive algorithm for finding the period of a string.
/// Time complexity: O(n<sup>2</sup>)
pub const PERIOD_NAIVE1: Algorithm = Algorithm {
    name: "period naive 1",
    function: period_naive1,
};

/// Variation of the naive algorithm for finding the period of a string
/// Time complexity: O(n<sup>2</sup>)
pub const PERIOD_NAIVE2: Algorithm = Algorithm {
    name: "period naive 2",
    function: period_naive2,
};

/// The smart algorithm for finding the period of a string
/// Time complexity: Θ(n<sup>2</sup>)
pub const PERIOD_SMART: Algorithm = Algorithm {
    name: "period smart",
    function: period_smart,
};

/// The naive algorithm for finding the period of a string.
///
/// # Arguments
///
/// * `s` - The string to be analyzed
pub fn period_naive1(s: &InputString) -> usize {
    let n = s.len();

    'outer: for i in 1..n {
        for j in 0..n - i {
            if s[j] != s[j + i] {
                continue 'outer;
            }
        }
        return i;
    }
    n
}

/// Variation of the naive algorithm for finding the period of a string
///
/// # Arguments
///
/// * `s` - The string to be analyzed
pub fn period_naive2(s: &InputString) -> usize {
    let n = s.len();
    for i in 1..n {
        if s[..n - i] == s[i..] {
            return i;
        }
    }
    n
}

/// The smart algorithm for finding the period of a string
///
/// # Arguments
///
/// * `s` - The string to be analyzed
pub fn period_smart(s: &InputString) -> usize {
    let size = s.len();

    // b[i] represents the maximum edge length of s[0..i]
    let mut b = vec![0; size];

    // current maximum edge length
    let mut x;

    // update b[i] as a function of b[0..i-1]
    for i in 1..size {
        // update x with the length of the maximum edge of s[0..i-1]
        x = b[i - 1];
        // if the new character (of the suffix) is not equal to the character
        // following the prefix then the next candidate for the maximum edge is
        // the maximum edge of the prefix
        while s[x] != s[i] && x > 0 {
            x = b[x - 1];
        }

        // if they are equal then the length of the maximum edge is increased
        if s[x] == s[i] {
            x += 1;
        }

        // update b[i]
        b[i] = x;
    }
    // the maximum border of the entire string
    let max_border = b[size - 1];
    // The minimum fractional period is the length of the string minus the maximum border
    size - max_border
}
