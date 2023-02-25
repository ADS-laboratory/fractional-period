use time_complexity_plot::{random::{Distribution,
                                  lengths::{EXPONENTIAL}, strings::METHOD1},
                                  algorithms::{PERIOD_NAIVE1, PERIOD_NAIVE2, PERIOD_SMART}, plot::time_plot, measurements::measure};
fn main() {

    // Create new exponential distribution
    let rnd = Distribution::new(EXPONENTIAL, 1000, 500_000);

    let strings = rnd.create_random_strings(METHOD1, vec!['a', 'b'], 100);

    let algorithms = vec![PERIOD_NAIVE1, PERIOD_NAIVE2, PERIOD_SMART];

    let results = measure(&strings, &algorithms, 0.001);

    let file_name = "plotters-doc-data/tick_control.svg";

    let result_clone = results.clone();
    result_clone.serialize("results.json");
    /*
    for result in result_clone.measurements {
        let log_linear_regression = result.log_scale().linear_regression();
        println!("{}: {} * x + {}", result.algorithm_name , log_linear_regression.0, log_linear_regression.1)
    }
     */

    time_plot(file_name, results);
}