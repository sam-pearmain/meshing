use plotters::prelude::*;

pub fn plot_injective_function<F>(
    f: F,
    x_range: (f64, f64),
    y_range: (f64, f64),
    file_name: &str,
) -> Result<(), Box<dyn std::error::Error>>
where
    F: Fn(f64) -> f64,
{
    // create drawing area with a fixed size
    let root = BitMapBackend::new(file_name, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .margin(10)
        .caption("function plot", ("sans-serif", 40).into_font())
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(x_range.0..x_range.1, y_range.0..y_range.1)?;
    chart.configure_mesh().draw()?;
    // generate 1000 sample points from x_range.0 to x_range.1
    chart.draw_series(LineSeries::new(
        (0..=1000).map(|i| {
            let x = x_range.0 + (x_range.1 - x_range.0) * i as f64 / 1000.0;
            (x, f(x))
        }),
        &RED,
    ))?;
    // ensure the drawing area is saved to file
    root.present()?;
    println!("result saved to {}", file_name);
    Ok(())
}