use fractional_period::{
    algorithms::{Algorithm, PERIOD_NAIVE1, PERIOD_NAIVE2, PERIOD_SMART},
    input::InputString,
};

/// The algorithms to be tested
const ALGORITHMS: [Algorithm; 3] = [PERIOD_NAIVE1, PERIOD_NAIVE2, PERIOD_SMART];

/// Test the given period finding algorithms
///
/// # Arguments
///
/// * `input` - The string to be analyzed
/// * `expected` - The expected period
/// * `algorithms` - The algorithms to be tested
fn test_algorithms(input: InputString, expected: usize, algorithms: &[Algorithm]) {
    for algorithm in algorithms {
        let actual = (algorithm.function)(&input.clone());
        assert_eq!(expected, actual);
    }
}

/// Test all the period finding algorithms
///
/// # Arguments
///
/// * `input` - The string to be analyzed
/// * `expected` - The expected period
fn test(input: InputString, expected: usize) {
    test_algorithms(input, expected, &ALGORITHMS);
}

#[test]
fn test_1() {
    let input: InputString = "abcabcab".into();
    let expected = 3;
    test(input, expected);
}

#[test]
fn test_2() {
    let input: InputString = "aba".into();
    let expected = 2;
    test(input, expected);
}

#[test]
fn test_3() {
    let input = "abca".into();
    let expected = 3;
    test(input, expected);
}

// test used in e-learning page

macro_rules! test {
    ($(($input:expr, $expected:expr)), *) => {
        #[test]
        fn test_examples() {
            $(
                let input: InputString = $input.into();
                let expected = $expected;
                test(input, expected);
            )*
        }
    };
}

test!(
    ("abcabcabc", 3),
    ("abcabcabca", 3),
    ("abcabcabcab", 3),
    ("abab", 2),
    ("ab", 2),
    ("abababa", 2),
    ("abc", 3),
    ("abca", 3),
    ("aaa", 1),
    ("a", 1),
    ("aaaaaaaa", 1),
    ("ababaaba", 5),
    ("ababaababa", 5),
    ("abcabcaabcabcaabc", 7),
    ("abcabcaabcabcaabcabca", 7),
    ("abcabcaabcabcaabcabcab", 17),
    ("abcabcaabcabcaabcabcabc", 17),
    ("ababaababaababaa", 5),
    ("ababaababaababaaababaababaababa", 16),
    ("abbabaabbaababbabaababbaabbabaab", 24)
);