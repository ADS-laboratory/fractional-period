use plotters::{
    prelude::{ChartBuilder, IntoDrawingArea, LabelAreaPosition, Rectangle, SVGBackend},
    series::LineSeries,
    style::{AsRelative, Color, IntoFont, RGBColor, BLACK, WHITE},
};
use plotters::backend::BitMapBackend;
use plotters::prelude::{Histogram, IntoSegmentedCoord, RED};
use time_complexity_plot::input::{distribution::Distribution, Input, InputBuilder};

use crate::{algorithms::period_smart, input::InputString};

pub fn input_analysis<D: Distribution>(
    input_gen: Vec<(InputBuilder<InputString, D>, &str, RGBColor)>,
    path: &str,
) {
    let root = SVGBackend::new(path, (1024, 768)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let (upper, lower) = root.split_vertically(750);

    lower
        .titled(
            "Input analysis",
            ("sans-serif", 10).into_font().color(&BLACK.mix(0.5)),
        )
        .unwrap();

    let caption = "String generation method analysis";

    let mut chart = ChartBuilder::on(&upper)
        .caption(&caption, ("sans-serif", (5).percent_height()))
        .set_label_area_size(LabelAreaPosition::Left, (8).percent())
        .set_label_area_size(LabelAreaPosition::Bottom, (4).percent())
        .margin((1).percent())
        .build_cartesian_2d(0usize..500_000usize, 0usize..500_000usize)
        .unwrap();

    chart
        .configure_mesh()
        .x_desc("String length")
        .y_desc("Fractional period")
        .draw()
        .unwrap();

    let n = 100;
    let repetitions = 10;
    let _tot = n * repetitions;

    // Iterate over the input generation methods
    for (_i, (input, name, color)) in input_gen.iter().enumerate() {
        // Build the strings
        let strings = input.build_with_repetitions(n, repetitions);

        //let color = Palette99::pick(i).mix(0.9);

        // For each string length, calculate the mean fractional period and draw a line
        chart
            .draw_series(LineSeries::new(
                strings.inputs.iter().map(|same_size_strings| {
                    (
                        same_size_strings[0].get_size(),
                        mean(same_size_strings.iter().map(|string| period_smart(string))),
                    )
                }),
                color.stroke_width(3),
            ))
            .unwrap()
            .label(*name)
            .legend(move |(x, y)| Rectangle::new([(x, y - 5), (x + 10, y + 5)], color.filled()));


        // HISTOGRAMS
        // One histogram showing probability of each period length for each input generation method
        let dir = "plotters-doc-data/histogram";
        let extension = ".png";
        let path = &(dir.to_string() + *name + extension);
        println!("Histogram path: {}", path);
        let root = BitMapBackend::new(path.as_str(), (640, 480)).into_drawing_area();

        root.fill(&WHITE).unwrap();

        let mut prob_analysis = Vec::new();
        for s in strings.inputs.iter().flatten().map(|string| {
            (
                string.get_size() as u32,
                period_smart(string) as u32,
            )
        }) {
            prob_analysis.push(s.0 - s.1);
        }

        // Find the number of buckets (i.e. the max x value)
        let mut max_x = 0;
        for x in prob_analysis.iter() {
            if *x > max_x {
                max_x = *x;
            }
        }
        // If the max x value is more than 25, we want to reduce the number of buckets
        // by summing up values in intervals of buckets
        let step = (max_x as f64 / 25.0).ceil() as u32;
        if max_x > 25 {
            let mut new_prob_analysis = Vec::new();
            for x in prob_analysis.iter() {
                new_prob_analysis.push(*x / step);
            }
            prob_analysis = new_prob_analysis;
        }
        max_x = max_x / step;

        // Find number that apperars most often in the vector (i.e. the max y value)
        let mut prob_analysis_sorted = prob_analysis.clone();
        prob_analysis_sorted.sort();
        let mut past_x = prob_analysis_sorted[0];
        let mut max_y = 1;
        let mut i = 0;
        for x in prob_analysis_sorted.iter() {
            if *x == past_x {
                i += 1;
            } else {
                if i > max_y {
                    max_y = i;
                }
                i = 1;
                past_x = *x;
            }
        }
        if i > max_y {
            max_y = i;
        }

        let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(35)
        .y_label_area_size(40)
        .margin(5)
        .caption("Histogram Test", ("sans-serif", 50.0))
        .build_cartesian_2d((0u32..max_x).into_segmented(), 0u32..max_y).unwrap();

        chart
        .configure_mesh()
        .disable_x_mesh()
        .bold_line_style(&WHITE.mix(0.3))
        .y_desc("Count")
        .x_desc("Bucket")
        .axis_desc_style(("sans-serif", 15))
        .draw().unwrap();

        chart.draw_series(
            Histogram::vertical(&chart)
                .style(RED.mix(0.5).filled())
                .data(prob_analysis.iter().map(|x: &u32| (*x, 1))),
        ).unwrap();

        // To avoid the IO failure being ignored silently, we manually call the present function
        root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
        println!("Result has been saved to {}", path);
    }

    println!("==================================================");
    println!("Input generation analysis results saved to {}", path);
}

fn mean(values: impl Iterator<Item = usize>) -> usize {
    let mut mean = 0.0;
    for (i, value) in values.enumerate() {
        mean += (value as f64 - mean) / (i as f64 + 1.0);
    }
    mean as usize
}
