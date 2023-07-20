use fractional_period::algorithms::{PERIOD_NAIVE1, PERIOD_NAIVE2, PERIOD_SMART};
use fractional_period::input::{StringGen, StringGenFunction};

use chrono_probe::plot::{PlotConfig, Scale};
use chrono_probe::{
    input::{distribution, InputBuilder},
    measurements::measure,
    plot::time_plot,
};

fn main() {
    // Create a distribution for the length of the strings
    let length_distribution = distribution::Reciprocal::new(1000..=500_000);

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
        .with_scale(Scale::LogLog)
        .with_title("Fractional Period")
        .with_caption("The time plot of fractional period algorithms");

    time_plot("results/tick_control.svg", results, &config);
}
