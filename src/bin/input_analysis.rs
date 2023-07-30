use fractional_period::input::{StringGen, StringGenFunction};

use fractional_period::input_plot;
use chrono_probe::input::distribution::Uniform;
use chrono_probe::input::InputBuilder;

fn main() {
    // Input Analysis
    // Create a distribution for the length of the strings
    let distribution_max = 500_000;
    let length_distribution = Uniform::new(500_000..=distribution_max);

    // Generation method for the strings
    let alphabet = vec![b'a', b'b'];
    let string_gens = vec![
        (
            StringGen::new(StringGenFunction::CreateRandomString1, alphabet.clone()),
            "CreateRandomString1",
        ),
        (
            StringGen::new(StringGenFunction::CreateRandomString2, alphabet.clone()),
            "CreateRandomString2",
        ),
        (
            StringGen::new(StringGenFunction::CreateRandomString3, alphabet.clone()),
            "CreateRandomString3",
        ),
        (
            StringGen::new(StringGenFunction::CreateRandomString4, alphabet.clone()),
            "CreateRandomString4",
        ),
    ];

    // Create the builder for the strings
    let string_builders = string_gens
        .iter()
        .map(|(string_gen, str)| {
            (
                InputBuilder::new(length_distribution.clone(), string_gen.clone()),
                *str,
            )
        })
        .collect::<Vec<_>>();

    // Plot a graph of with the input generation analysis
    input_plot::input_analysis(string_builders, distribution_max, 1, 1000);
}
