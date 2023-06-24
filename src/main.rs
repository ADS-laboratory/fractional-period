use fractional_period::algorithms::{
    period_naive1, period_naive2, period_smart, PERIOD_NAIVE1, PERIOD_NAIVE2, PERIOD_SMART,
};
use fractional_period::input::{StringGen, StringGenFunction};

use fractional_period::input_analysis;
use plotters::style::{BLACK, YELLOW, RED, GREEN};
use time_complexity_plot::input::distribution::Uniform;
use time_complexity_plot::plot::PlotConfig;
use time_complexity_plot::{
    input::{distribution::Exponential, InputBuilder},
    measurements::measure,
    plot::time_plot,
};

fn main() {
    // Create a distribution for the length of the strings
    let length_distribution = Exponential::new(1000..=500_000);

    // Generation method for the strings
    let string_gen = StringGen::new(StringGenFunction::CreateRandomString1, vec![b'a', b'b']);

    // Create the builder for the strings
    let string_builder = InputBuilder::new(length_distribution, string_gen);

    // Build the strings
    let strings = string_builder.build(100);

    // Create a slice of the algorithms we want to measure
    let algorithms = [
        (PERIOD_NAIVE1.function, PERIOD_NAIVE1.name),
        (PERIOD_NAIVE2.function, PERIOD_NAIVE2.name),
        (PERIOD_SMART.function, PERIOD_SMART.name),
    ];

    // Measure the algorithms on the strings
    let results = measure(&strings, &algorithms, 0.001);

    // save data to json file
    let result_clone = results.clone();
    result_clone.serialize_json("results.json");

    /*
    for result in result_clone.measurements {
        let log_linear_regression = result.log_scale().linear_regression();
        println!("{}: {} * x + {}", result.algorithm_name , log_linear_regression.0, log_linear_regression.1)
    }
     */

    // Plot the results
    let config = PlotConfig::default()
        .with_builder(&string_builder)
        .with_title("Fractional Period")
        .with_caption("The time plot of fractional period algorithms");

    time_plot("plotters-doc-data/tick_control.svg", results, &config);

    // Input Analysis
    // Create a distribution for the length of the strings
    let length_distribution = Uniform::new(1000..=1_000_000);

    // Generation method for the strings
    let string_gens = vec![
        (StringGen::new(StringGenFunction::CreateRandomString1, vec![b'a', b'b']), "CreateRandomString1", BLACK),
        //(StringGen::new(StringGenFunction::CreateRandomString2, vec![b'a', b'b']), "CreateRandomString2", YELLOW),
        //(StringGen::new(StringGenFunction::CreateRandomString3, vec![b'a', b'b']), "CreateRandomString3", RED),
        //(StringGen::new(StringGenFunction::CreateRandomString4, vec![b'a', b'b']), "CreateRandomString4", GREEN),
    ];

    // Create the builder for the strings
    let string_builders = string_gens
        .iter()
        .map(|(string_gen, str, color)| (InputBuilder::new(length_distribution.clone(), string_gen.clone()), *str, *color))
        .collect::<Vec<_>>();

    // Plot the results
    input_analysis::input_analysis(string_builders, "plotters-doc-data/input_analysis.svg");
}
