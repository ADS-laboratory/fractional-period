use fractional_period::{algorithms::{Algorithm, PERIOD_NAIVE1, PERIOD_NAIVE2, PERIOD_SMART}, random::{String, InputString}};

/// The algorithms to be tested
const ALGORITHMS: [Algorithm; 3] = [
    PERIOD_NAIVE1,
    PERIOD_NAIVE2,
    PERIOD_SMART,
];

/// Test the given period finding algorithms
///
/// # Arguments
///
/// * `input` - The string to be analyzed
/// * `expected` - The expected period
/// * `algorithms` - The algorithms to be tested
fn test_algorithms(input: String, expected: usize, algorithms: &[Algorithm]) {
    for algorithm in algorithms {
        let actual = (algorithm.function)(input.clone());
        assert_eq!(expected, actual);
    }
}

/// Test all the period finding algorithms
///
/// # Arguments
///
/// * `input` - The string to be analyzed
/// * `expected` - The expected period
fn test(input: String, expected: usize) {
    test_algorithms(input, expected, &ALGORITHMS);
}

#[test]
fn test_1() {
    let input = InputString(vec![b'a', b'b', b'c', b'a', b'b', b'c', b'a', b'b']);
    let expected = 3;
    test(input, expected);
}

#[test]
fn test_2() {
    let input = InputString(vec![b'a', b'b', b'a']);
    let expected = 2;
    test(input, expected);
}

#[test]
fn test_3() {
    let input = InputString(vec![b'a', b'b', b'c', b'a']);
    let expected = 3;
    test(input, expected);
}

// TODO: import VPL tests