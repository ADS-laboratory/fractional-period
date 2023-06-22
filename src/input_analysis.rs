use plotters::{
    prelude::{ChartBuilder, IntoDrawingArea, LabelAreaPosition, SVGBackend},
    series::LineSeries,
    style::{AsRelative, Color, IntoFont, BLACK, WHITE},
};
use time_complexity_plot::input::{distribution::Distribution, Input, InputBuilder};

use crate::{algorithms::period_smart, input::InputString};

pub fn input_analysis<D: Distribution>(input_gen: Vec<InputBuilder<InputString, D>>) {
    let root =
        SVGBackend::new("plotters-doc-data/input_analysis.svg", (1024, 768)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let (upper, lower) = root.split_vertically(750);

    lower
        .titled(
            "Input analysis",
            ("sans-serif", 10).into_font().color(&BLACK.mix(0.5)),
        )
        .unwrap();

    let caption = "caption";

    let mut chart = ChartBuilder::on(&upper)
        .caption(&caption, ("sans-serif", (5).percent_height()))
        .set_label_area_size(LabelAreaPosition::Left, (8).percent())
        .set_label_area_size(LabelAreaPosition::Bottom, (4).percent())
        .margin((1).percent())
        .build_cartesian_2d(0..500_000, 0..500_000)
        .unwrap();

    chart
        .configure_mesh()
        .x_desc("x label")
        .y_desc("y label")
        .draw()
        .unwrap();

    for input in input_gen {
        let strings = input.build_with_repetitions(100, 10);

        // Plot the results using plotters
        chart.draw_series(LineSeries::new(
            strings.inputs.iter().map(|same_size_strings| {
                (
                    mean(same_size_strings.iter().map(|string| period_smart(string))) as i32,
                    same_size_strings[0].get_size() as i32,
                )
            }),
            BLACK.stroke_width(3),
        ));
    }
}

fn mean(values: impl Iterator<Item = usize>) -> f64 {
    let mut sum = 0;
    let mut count = 0;
    for value in values {
        sum += value;
        count += 1;
    }
    sum as f64 / count as f64
}