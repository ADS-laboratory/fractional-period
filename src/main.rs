use time_complexity_plot::{
    algorithms::{PERIOD_NAIVE1, PERIOD_NAIVE2, PERIOD_SMART},
    measurements::measure,
    plot::time_plot,
    random::{
        lengths::{LengthDistribution, EXPONENTIAL},
        strings::{StringGen, METHOD1},
        StringsBuilder,
    },
};

fn main() {
    let length_distribution = LengthDistribution::new(EXPONENTIAL, 1000, 500_000);

    let string_gen = StringGen::new(METHOD1, vec!['a', 'b']);

    let strings_builder = StringsBuilder::new(length_distribution, string_gen);

    let strings = strings_builder.create_random_strings(100);

    let algorithms = vec![PERIOD_NAIVE1, PERIOD_NAIVE2, PERIOD_SMART];

    let results = measure(&strings, &algorithms, 0.01);

    let file_name = "plotters-doc-data/tick_control.svg";

    let result_clone = results.clone();
    result_clone.serialize_json("results.json");

    /*
    for result in result_clone.measurements {
        let log_linear_regression = result.log_scale().linear_regression();
        println!("{}: {} * x + {}", result.algorithm_name , log_linear_regression.0, log_linear_regression.1)
    }
     */

    time_plot(file_name, results);
}
