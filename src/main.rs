use fractional_period::algorithms::{period_naive1, period_naive2, period_smart};
use fractional_period::random::{String, StringGen, StringGenFunction};

use time_complexity_plot::{
    input::{
        distribution::{DistributionBuilder, EXPONENTIAL},
        InputBuilder,
    },
    measurements::measure,
    plot::time_plot,
};

fn main() {
    let length_distribution = DistributionBuilder::new(EXPONENTIAL, 1000, 500_000);

    let string_gen = StringGen::new(StringGenFunction::CreateRandomString1, vec![b'a', b'b']);

    let string_builder = InputBuilder::new(length_distribution, string_gen);

    let strings = string_builder.build_with_repetitions(200, 1);

    let algorithms: Vec<fn(String) -> usize> = vec![period_naive1, period_naive2, period_smart];

    let results = measure(&strings, &algorithms, 0.001);

    let file_name = "plotters-doc-data/tick_control.svg";

    let result_clone = results.clone();
    result_clone.serialize_json("results.json");

    /*
    for result in result_clone.measurements {
        let log_linear_regression = result.log_scale().linear_regression();
        println!("{}: {} * x + {}", result.algorithm_name , log_linear_regression.0, log_linear_regression.1)
    }
     */

    time_plot(file_name, results, string_builder);
}
