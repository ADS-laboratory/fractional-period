use rand::{thread_rng, Rng};
use std::ops::Deref;
use time_complexity_plot::input::Input;

#[derive(Clone)]
pub struct InputString(pub Vec<u8>);

impl From<&str> for InputString {
    fn from(s: &str) -> Self {
        // TODO: check for ascii characters
        InputString(s.as_bytes().to_vec())
    }
}

impl Deref for InputString {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Input for InputString {
    type Builder = StringGen;

    fn get_size(&self) -> usize {
        self.len()
    }

    fn generate_input(size: usize, builder: &Self::Builder) -> Self {
        InputString(builder.create_random_string(size))
    }
}

fn create_random_string1(n: usize, char_set: &Vec<u8>) -> Vec<u8> {
    let mut s: Vec<u8> = Vec::with_capacity(n);
    let number_of_chars = char_set.len();
    for _ in 0..n {
        // generate random character
        let char_index = thread_rng().gen_range(0..number_of_chars);
        let char = char_set[char_index];
        s.push(char);
    }
    s
}

fn create_random_string2(n: usize, char_set: &Vec<u8>) -> Vec<u8> {
    let mut s: Vec<u8> = Vec::with_capacity(n);
    let number_of_chars = char_set.len();
    // TODO: This value should not be generated each time
    let q = thread_rng().gen_range(1..n);
    for _ in 0..q {
        // generate random character
        let char_index = thread_rng().gen_range(0..number_of_chars);
        let char = char_set[char_index];
        s.push(char);
    }
    for i in q..n {
        let char = s[i % q];
        s.push(char);
    }
    s
}

fn create_random_string3(n: usize, char_set: &Vec<u8>) -> Vec<u8> {
    // new ascii character
    fn new_char(char_set: &[u8]) -> u8 {
        for i in 0..128 {
            if !char_set.contains(&i) {
                return i;
            }
        }
        panic!("No new character found");
    }

    let mut s: Vec<u8> = Vec::with_capacity(n);
    let number_of_chars = char_set.len();
    let q = thread_rng().gen_range(0..n);
    for _ in 0..q - 1 {
        // generate random character
        let char_index = thread_rng().gen_range(0..number_of_chars);
        let char = char_set[char_index];
        s.push(char);
    }
    s.push(new_char(char_set));
    for i in q..n {
        let char = s[i % q];
        s.push(char);
    }
    s
}

fn create_random_string4(n: usize, char_set: &Vec<u8>) -> Vec<u8> {
    let mut s = Vec::with_capacity(n);
    let number_of_chars = char_set.len();
    let mut char = char_set[0];
    for i in 0..n - 1 {
        char = char_set[i % number_of_chars];
        s.push(char);
    }
    s.push(char);
    s
}

pub enum StringGenFunction {
    CreateRandomString1,
    CreateRandomString2,
    CreateRandomString3,
    CreateRandomString4,
}

impl StringGenFunction {
    fn get_function(&self) -> fn(n: usize, char_set: &Vec<u8>) -> Vec<u8> {
        match self {
            StringGenFunction::CreateRandomString1 => create_random_string1,
            StringGenFunction::CreateRandomString2 => create_random_string2,
            StringGenFunction::CreateRandomString3 => create_random_string3,
            StringGenFunction::CreateRandomString4 => create_random_string4,
        }
    }
}

#[derive(Clone)]
pub struct StringGen {
    pub function: fn(n: usize, char_set: &Vec<u8>) -> Vec<u8>,
    pub char_set: Vec<u8>,
}

impl StringGen {
    /// Creates a new StringGen struct
    ///
    /// # Arguments
    ///
    /// * `function` - The function used to generate the random string
    /// * `char_set` - The character set used to generate the random string
    ///
    /// # Panics
    ///
    /// * Panics if the character set is empty
    /// * Panics if the character set contains repetitions
    /// * Panics if the character set contains non ascii characters
    ///
    /// # Examples
    ///
    /// ```
    /// use fractional_period::input::{StringGenFunction::CreateRandomString1, StringGen};
    ///
    /// let char_set = vec![b'a', b'b', b'c'];
    /// let string_gen = StringGen::new(CreateRandomString1, char_set);
    /// ```
    pub fn new(function: StringGenFunction, char_set: Vec<u8>) -> Self {
        assert!(!char_set.is_empty(), "The character set must not be empty");

        // checking for repetitions in char_set
        let mut char_set_sorted = char_set.clone();
        char_set_sorted.sort_by(|a, b| b.cmp(a));
        for i in 0..char_set_sorted.len() - 1 {
            if char_set_sorted[i] == char_set_sorted[i + 1] {
                panic!("The character set contains repetitions");
            }
        }

        Self {
            function: function.get_function(),
            char_set,
        }
    }

    /// Creates a random string using the character set specified in the struct
    ///
    /// # Arguments
    ///
    /// * `n` - The length of the string to be generated
    ///
    /// # Panics
    ///
    /// * Panics if the length of the string to be generated is less than 1
    pub fn create_random_string(&self, n: usize) -> Vec<u8> {
        assert!(
            n > 0,
            "The length of the string to be generated must be greater than 0"
        );
        (self.function)(n, &self.char_set)
    }
}
