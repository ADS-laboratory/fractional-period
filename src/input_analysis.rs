use plotters::{
    prelude::{ChartBuilder, IntoDrawingArea, LabelAreaPosition, SVGBackend, Rectangle},
    series::LineSeries,
    style::{AsRelative, Color, IntoFont, BLACK, WHITE, RGBColor, Palette99, Palette},
};
use time_complexity_plot::input::{distribution::Distribution, InputBuilder, Input};

use crate::{algorithms::{period_smart, period_naive1}, input::InputString};

pub fn input_analysis<D: Distribution>(input_gen: Vec<(InputBuilder<InputString, D>, &str, RGBColor)>, path: &str) {
    let root =
        SVGBackend::new(path, (1024, 768)).into_drawing_area();
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
        .build_cartesian_2d(0i32..1_000_000i32, 0i32..1_000_000i32)
        .unwrap();

    chart
        .configure_mesh()
        .x_desc("String length")
        .y_desc("Fractional period")
        .draw()
        .unwrap();

    for (i, (input, name, color)) in input_gen.iter().enumerate() {
        let strings = input.build_with_repetitions(100, 10);

        //let color = Palette99::pick(i).mix(0.9);

        // Plot the results using plotters
        chart.draw_series(LineSeries::new(
            strings.inputs.iter().map(|same_size_strings| {
                (
                    same_size_strings[0].get_size() as i32,
                    mean(same_size_strings.iter().map(|string| period_naive1(string))) as i32,
                )
            }),
            color.stroke_width(3),
        ))
        .unwrap()
        .label(*name)
        .legend(move |(x, y)| Rectangle::new([(x, y - 5), (x + 10, y + 5)], color.filled()));
    }

    print!("Results saved to {}\n", path);
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